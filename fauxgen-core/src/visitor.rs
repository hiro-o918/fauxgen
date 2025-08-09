use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
};

use anyhow::Result;
use log::{debug, error};
use rustpython_ast::{
    text_size::TextRange, Stmt, StmtClassDef, StmtImport, StmtImportFrom, Visitor,
};
use rustpython_parser::Parse;

// No external crate imports needed for this module

/// Manages import information for a specific file
#[derive(Debug, Clone, Default)]
struct FileImports {
    /// Maps module aliases to their full module paths
    /// e.g., "pa" -> "pandera" for `import pandera as pa`
    module_aliases: HashMap<String, String>,

    /// Maps imported class/object names to their full module paths
    /// e.g., "DataFrameModel" -> "pandera.DataFrameModel" for `from pandera import DataFrameModel`
    imported_items: HashMap<String, String>,

    /// Root module path used for resolving relative imports
    root_module_path: PathBuf,
}

impl FileImports {
    /// Create a new FileImports with a root module path
    fn new(root_module_path: PathBuf) -> Self {
        Self {
            module_aliases: HashMap::new(),
            imported_items: HashMap::new(),
            root_module_path,
        }
    }

    /// Process a Python import statement
    /// Example: `import pandera as pa`
    fn process_import(&mut self, import_stmt: &StmtImport<TextRange>) {
        for alias in &import_stmt.names {
            let module_name = alias.name.to_string();
            let alias_name = if let Some(asname) = &alias.asname {
                asname.to_string()
            } else {
                module_name.clone()
            };

            // Add the module alias
            self.add_module_alias(alias_name, module_name);
        }
    }

    /// Process a Python from-import statement
    /// Example: `from pandera import DataFrameModel as DFM`
    /// Also handles relative imports like `from ..base_model import BaseModel`
    fn process_import_from(
        &mut self,
        import_from_stmt: &StmtImportFrom<TextRange>,
        current_module_path: &Path,
    ) {
        if let Some(module) = &import_from_stmt.module {
            let module_str = module.to_string();

            // Check if this is a relative import
            let module_path = if module_str.starts_with('.') {
                // Resolve the relative import
                self.resolve_relative_import(&module_str, current_module_path)
            } else {
                module_str
            };

            for alias in &import_from_stmt.names {
                let name = alias.name.to_string();
                let alias_name = if let Some(asname) = &alias.asname {
                    asname.to_string()
                } else {
                    name.clone()
                };

                // Add the imported item with its full path
                let full_path = format!("{}.{}", module_path, name);
                self.add_imported_item(alias_name, full_path);

                // Also track the module itself
                let parts: Vec<&str> = module_path.split('.').collect();
                if !parts.is_empty() {
                    self.add_module_alias(parts[0].to_string(), parts[0].to_string());
                }
            }
        }
    }

    /// Resolve a relative import to an absolute module path
    fn resolve_relative_import(&self, relative_path: &str, current_module_path: &Path) -> String {
        // ドットの数をカウント
        let dot_count = relative_path.chars().take_while(|&c| c == '.').count();

        // 現在のディレクトリを取得
        let mut current_dir = current_module_path
            .parent()
            .unwrap_or(current_module_path)
            .to_path_buf();

        // ドットの数-1の分だけ親ディレクトリに遡る
        for _ in 1..dot_count {
            current_dir = current_dir.parent().unwrap_or(&current_dir).to_path_buf();
        }

        // 残りのパスを取得して処理
        let remaining = &relative_path[dot_count..];

        // ターゲットパスを構築
        let target_path = if remaining.is_empty() {
            // 残りのパスがない場合は現在のディレクトリを使用
            current_dir
        } else {
            // ドット表記をパス区切りに変換して結合
            current_dir.join(remaining.replace('.', "/"))
        };

        // パスをPythonモジュール名に変換
        self.path_to_module_name(&target_path)
    }

    /// Convert a file path to a Python module name
    fn path_to_module_name(&self, path: &Path) -> String {
        // Get the path relative to the root module's parent
        let rel_path =
            match path.strip_prefix(self.root_module_path.parent().unwrap_or(Path::new("/"))) {
                Ok(path) => path,
                Err(_) => {
                    // If we can't strip the prefix, just use the filename
                    return path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or("")
                        .trim_end_matches(".py")
                        .to_string();
                }
            };

        // Create a Python-style module path
        let mut module_parts = Vec::new();
        for component in rel_path.components() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_str().unwrap_or("");
                if name_str != "__init__.py" {
                    // For .py files, remove the extension
                    if name_str.ends_with(".py") {
                        module_parts.push(name_str.trim_end_matches(".py"));
                    } else {
                        module_parts.push(name_str);
                    }
                }
            }
        }

        // Join with dots to create the module ID
        module_parts.join(".")
    }

    /// Add a module alias from an import statement
    /// e.g., `import pandera as pa` -> ("pa", "pandera")
    fn add_module_alias(&mut self, alias: String, module: String) {
        self.module_aliases.insert(alias, module);
    }

    /// Add an imported item from a from-import statement
    /// e.g., `from pandera import DataFrameModel` -> ("DataFrameModel", "pandera.DataFrameModel")
    fn add_imported_item(&mut self, item_name: String, full_path: String) {
        self.imported_items.insert(item_name, full_path);
    }

    /// Resolve a name to its full path
    /// First checks imported items, then module aliases
    fn resolve_name(&self, name: &str) -> Option<String> {
        self.imported_items.get(name).cloned()
    }

    /// Resolve a module.attribute expression to its full path
    /// e.g., "pa.DataFrameModel" -> "pandera.DataFrameModel"
    fn resolve_attribute(&self, module_alias: &str, attr_name: &str) -> Option<String> {
        self.module_aliases
            .get(module_alias)
            .map(|full_module| format!("{}.{}", full_module, attr_name))
    }
}

#[derive(Debug, Clone)]
pub struct ClassDef<R = TextRange> {
    // Class id is a unique identifier for the class, which is a combination of the module path and class name
    pub id: String,
    // Name of the class
    pub name: String,
    // path to the module where the class is defined
    pub module_path: PathBuf,
    // List of fields defined in the class
    // this includes fields defined in the class itself and inherited from parent classes
    pub stmt_class_def: StmtClassDef<R>,
    // parent class IDs (for resolving inheritance later)
    pub parent_ids: Vec<String>,
}

impl PartialEq for ClassDef<TextRange> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.module_path == other.module_path
            && self.parent_ids == other.parent_ids
    }
}

// ClassID is a unique identifier for a class, which is a combination of the module path and class name
pub type ClassID = String;

/// Trait for ClassID related operations
pub trait ClassIDExt {
    /// Get the class name from the class ID
    fn class_name(&self) -> &str;
}

impl ClassIDExt for String {
    fn class_name(&self) -> &str {
        self.split('.').last().unwrap_or("")
    }
}

#[derive(Debug)]
pub struct ClassVisitor<R = TextRange> {
    _phantom: std::marker::PhantomData<R>,
    root_module_path: PathBuf,
    current_module_path: PathBuf,

    // Maps file paths to their import information
    file_imports: HashMap<PathBuf, FileImports>,

    file_class_ids: HashMap<PathBuf, Vec<ClassID>>,

    // Collection of all class definitions found
    class_defs: HashMap<ClassID, ClassDef<R>>,

    // Cache for get_class_defs_of_base results
    base_classes_cache: HashMap<ClassID, Vec<ClassDef<R>>>,
}

impl Visitor<TextRange> for ClassVisitor<TextRange> {
    fn visit_stmt_import(&mut self, node: StmtImport<TextRange>) {
        // Get or create the FileImports for the current file
        let file_imports = self
            .file_imports
            .entry(self.current_module_path.clone())
            .or_insert_with(|| FileImports::new(self.root_module_path.clone()));

        // Process the import statement
        file_imports.process_import(&node);
    }

    fn visit_stmt_import_from(&mut self, node: StmtImportFrom<TextRange>) {
        // Get or create the FileImports for the current file
        let file_imports = self
            .file_imports
            .entry(self.current_module_path.clone())
            .or_insert_with(|| FileImports::new(self.root_module_path.clone()));

        // Process the from-import statement with current module path for relative import resolution
        file_imports.process_import_from(&node, &self.current_module_path);
    }

    fn visit_stmt_class_def(&mut self, node: rustpython_ast::StmtClassDef<TextRange>) {
        let class_name = node.name.to_string();
        let class_id = self.create_class_id(&class_name);

        // Get the current file's import information
        let file_imports = match self.file_imports.get(&self.current_module_path) {
            Some(imports) => imports,
            None => {
                // If we don't have import information for this file yet, create it
                self.file_imports.insert(
                    self.current_module_path.clone(),
                    FileImports::new(self.root_module_path.clone()),
                );
                self.file_imports.get(&self.current_module_path).unwrap()
            }
        };

        // Extract parent class IDs
        let mut parent_ids = Vec::new();
        for base in &node.bases {
            if let Some(name_expr) = base.as_name_expr() {
                let parent_name = name_expr.id.to_string();

                // First check if this is a directly imported class
                if let Some(full_path) = file_imports.resolve_name(&parent_name) {
                    // This is an imported class with its full path
                    parent_ids.push(full_path);
                } else {
                    // Assume it's in the current module
                    let parent_id = self.create_class_id(&parent_name);
                    parent_ids.push(parent_id);
                }
            } else if let Some(attr_expr) = base.as_attribute_expr() {
                // Handle attribute expressions like 'pa.DataFrameModel'
                if let Some(name_expr) = attr_expr.value.as_name_expr() {
                    let module_alias = name_expr.id.to_string();
                    let class_name = attr_expr.attr.to_string();

                    // Try to resolve the module alias to its full package name
                    if let Some(full_parent_path) =
                        file_imports.resolve_attribute(&module_alias, &class_name)
                    {
                        parent_ids.push(full_parent_path);
                    } else {
                        // Fallback to the alias if full package name is not available
                        let parent_id = format!("{}.{}", module_alias, class_name);
                        parent_ids.push(parent_id);
                    }
                }
            }
        }

        // For now, we're not handling fields - that will be done by the PanderaHandler
        // This is just for tracking class definitions across modules
        let class_def = ClassDef {
            id: class_id.clone(),
            name: class_name,
            module_path: self.current_module_path.clone(),
            stmt_class_def: node,
            parent_ids,
        };

        // Store the class definition in class_defs map
        self.class_defs.insert(class_id.clone(), class_def);

        // Add the class ID to the file_class_ids map
        self.file_class_ids
            .entry(self.current_module_path.clone())
            .or_default()
            .push(class_id);
    }
}

impl ClassVisitor {
    pub fn new(root_module_path: PathBuf) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            root_module_path: root_module_path.clone(),
            current_module_path: root_module_path,
            file_imports: HashMap::new(),
            file_class_ids: HashMap::new(),
            class_defs: HashMap::new(),
            base_classes_cache: HashMap::new(),
        }
    }

    // Set the current module path when processing a new file
    pub fn set_current_module_path(&mut self, module_path: PathBuf) {
        self.current_module_path = module_path;
    }

    // Create a unique class ID based on module path and class name
    fn create_class_id(&self, class_name: &str) -> String {
        // Get the module path relative to the root module
        let rel_path = match self
            .current_module_path
            .strip_prefix(self.root_module_path.parent().unwrap())
        {
            Ok(path) => path,
            Err(_) => {
                // If we can't strip the prefix, just use the filename
                return format!(
                    "{}.{}",
                    self.current_module_path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or(""),
                    class_name
                );
            }
        };

        // Create a Python-style module path
        let mut module_parts = Vec::new();
        for component in rel_path.components() {
            if let std::path::Component::Normal(name) = component {
                let name_str = name.to_str().unwrap_or("");
                if name_str != "__init__.py" {
                    // For .py files, remove the extension
                    if name_str.ends_with(".py") {
                        module_parts.push(name_str.trim_end_matches(".py"));
                    } else {
                        module_parts.push(name_str);
                    }
                }
            }
        }

        // Create the module identifier
        let module_id = if !module_parts.is_empty() {
            module_parts.join(".")
        } else {
            // Default to the file name if we can't determine the module path
            self.current_module_path
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("")
                .trim_end_matches(".py")
                .to_string()
        };

        // Create the full class ID
        format!("{}.{}", module_id, class_name)
    }

    // Process a Python file and extract class definitions
    fn process_file(&mut self, file_path: &Path, stmts: Vec<Stmt<TextRange>>) {
        // Set the current module path for this file
        self.set_current_module_path(file_path.to_path_buf());

        // Process all statements using the visitor pattern
        for stmt in stmts {
            // Use the Visitor trait's visit_stmt method
            // This will call our visit_stmt method, which then calls generic_visit_stmt
            // which in turn will call the appropriate visitor method for each type
            self.visit_stmt(stmt);
        }
    }

    // Process a Python module (directory with __init__.py) and all its files
    pub fn process_module(&mut self, module_dir: &Path) -> Result<()> {
        // Record the original module path to restore it later
        let original_path = self.current_module_path.clone();

        // Process the module's __init__.py file first if it exists
        let init_file = module_dir.join("__init__.py");
        if init_file.exists() {
            debug!("Processing module init: {}", init_file.display());
            let content = std::fs::read_to_string(&init_file)?;
            let stmts =
                rustpython_parser::ast::Suite::parse(&content, init_file.to_str().unwrap())?;
            self.process_file(&init_file, stmts);
        }

        // Process all Python files in the module directory
        for entry in std::fs::read_dir(module_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip __init__.py as we've already processed it
            if path.file_name().is_some_and(|f| f == "__init__.py") {
                continue;
            }

            if path.is_file() && path.extension().is_some_and(|ext| ext == "py") {
                // Process Python file
                debug!("Processing file: {}", path.display());
                let content = std::fs::read_to_string(&path)?;
                match rustpython_parser::ast::Suite::parse(&content, path.to_str().unwrap()) {
                    Ok(stmts) => self.process_file(&path, stmts),
                    Err(e) => {
                        error!("Error parsing file {}: {}", path.display(), e);
                        // Skip invalid files
                        continue;
                    }
                };
            } else if path.is_dir() {
                // Recursively process subdirectories as submodules
                debug!("Processing submodule: {}", path.display());
                self.process_module(&path)?;
            }
        }

        // Restore the original module path
        self.current_module_path = original_path;
        Ok(())
    }

    // Get the inheritance map (child class ID -> parent class IDs)
    pub fn get_inheritance_map(&self) -> HashMap<String, Vec<String>> {
        let mut inheritance_map = HashMap::new();

        for (class_id, class_def) in &self.class_defs {
            if !class_def.parent_ids.is_empty() {
                inheritance_map.insert(class_id.clone(), class_def.parent_ids.clone());
            }
        }

        inheritance_map
    }

    // Get all class definitions
    pub fn get_class_defs(&self) -> &HashMap<ClassID, ClassDef<TextRange>> {
        &self.class_defs
    }

    pub fn get_class_def(&self, class_id: &ClassID) -> Option<&ClassDef<TextRange>> {
        self.class_defs.get(class_id)
    }

    // Get class IDs for a specific file
    pub fn get_class_ids_for_file(&self, file_path: &Path) -> Option<&Vec<ClassID>> {
        self.file_class_ids.get(file_path)
    }

    // Get all file to class IDs mappings
    pub fn get_file_class_ids(&self) -> &HashMap<PathBuf, Vec<ClassID>> {
        &self.file_class_ids
    }

    /// Check if a class inherits from a specified base class
    /// - target_class_id: The class ID to check
    /// - base_class: The base class ID to check against
    ///   Returns true if target_class inherits from base_class (directly or indirectly)
    pub fn get_class_defs_of_base(
        &mut self,
        target_class_id: &ClassID,
        base_class: &ClassID,
    ) -> Vec<ClassDef> {
        // Check if we have a cached result
        if let Some(cached_result) = self.base_classes_cache.get(target_class_id) {
            return cached_result.clone();
        }

        let mut result = Vec::new();

        // If the target class is the base class, return empty vector
        if target_class_id == base_class {
            return result;
        }

        // Make a copy of the class definition and parent IDs to avoid borrowing self
        let class_def_clone_opt = self.class_defs.get(target_class_id).cloned();

        if let Some(class_def) = class_def_clone_opt {
            // Check if the target class directly inherits from the base class
            if class_def
                .parent_ids
                .iter()
                .any(|parent_id| parent_id == base_class)
            {
                // Direct inheritance found
                result.push(class_def.clone());
            }

            // Clone the parent_ids to avoid borrow issues
            let parent_ids = class_def.parent_ids.clone();

            // Check for indirect inheritance through parents
            for parent_id in &parent_ids {
                // Recursively check each parent
                let mut parent_results = self.get_class_defs_of_base(parent_id, base_class);
                result.append(&mut parent_results);
            }
            debug!(
                "Class {} inherits from base class {}: {}",
                target_class_id,
                base_class,
                !result.is_empty()
            );
        }

        // Cache the empty result
        self.base_classes_cache
            .insert(target_class_id.clone(), result.clone());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use rstest::{fixture, rstest};
    use std::path::PathBuf;

    #[fixture]
    fn dummy_stmt_class_def() -> StmtClassDef<TextRange> {
        StmtClassDef {
            range: TextRange::default(),
            name: "Dummy".into(),
            bases: vec![],
            keywords: vec![],
            body: vec![],
            decorator_list: vec![],
            type_params: vec![],
        }
    }

    #[rstest]
    #[test]
    fn test_visitor_tracks_relative_imports(
        dummy_stmt_class_def: StmtClassDef<TextRange>,
    ) -> Result<()> {
        // Get the absolute path to the resources directory
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_module_path = project_root.join("resources/visitor/test_relative_import");

        // Setup visitor with the test resources directory
        let mut visitor = ClassVisitor::new(root_module_path.clone());

        // Process the entire module
        visitor.process_module(&root_module_path)?;

        // Check inheritance map
        let inheritance_map = visitor.get_inheritance_map();
        assert!(
            inheritance_map
                == HashMap::from([
                    (
                        "test_relative_import.subpackage.relative_import.RelativeUser".to_string(),
                        vec!["base_model.BaseModel".to_string()]
                    ),
                    (
                        "test_relative_import.base_model.BaseModel".to_string(),
                        vec!["pandera.DataFrameModel".to_string()]
                    )
                ])
        );

        // Check class definitions
        let class_defs = visitor.get_class_defs();

        assert!(
            class_defs
                == &HashMap::from([
                    (
                        "test_relative_import.subpackage.relative_import.RelativeUser".to_string(),
                        ClassDef {
                            id: "test_relative_import.subpackage.relative_import.RelativeUser"
                                .to_string(),
                            name: "RelativeUser".to_string(),
                            module_path: root_module_path.join("subpackage/relative_import.py"),
                            parent_ids: vec!["base_model.BaseModel".to_string()],
                            stmt_class_def: dummy_stmt_class_def.clone()
                        }
                    ),
                    (
                        "test_relative_import.base_model.BaseModel".to_string(),
                        ClassDef {
                            id: "test_relative_import.base_model.BaseModel".to_string(),
                            name: "BaseModel".to_string(),
                            module_path: root_module_path.join("base_model.py"),
                            stmt_class_def: dummy_stmt_class_def.clone(),
                            parent_ids: vec!["pandera.DataFrameModel".to_string()]
                        }
                    )
                ])
        );

        // Check file_class_ids mapping
        let file_class_ids = visitor.get_file_class_ids();
        // Define expected file_class_ids mapping
        let expected_file_class_ids: HashMap<PathBuf, Vec<String>> = HashMap::from([
            (
                root_module_path.join("subpackage/relative_import.py"),
                vec!["test_relative_import.subpackage.relative_import.RelativeUser".to_string()],
            ),
            (
                root_module_path.join("base_model.py"),
                vec!["test_relative_import.base_model.BaseModel".to_string()],
            ),
        ]);

        // Compare entire maps
        assert_eq!(file_class_ids, &expected_file_class_ids);

        Ok(())
    }

    #[rstest]
    #[test]
    fn test_visitor_tracks_class_definitions(
        dummy_stmt_class_def: StmtClassDef<TextRange>,
    ) -> Result<()> {
        // Get the absolute path to the resources directory
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_module_path = project_root.join("resources/visitor/test_inheritance");

        // Setup visitor with the test resources directory
        let mut visitor = ClassVisitor::new(root_module_path.clone());

        // Process the entire module
        visitor.process_module(&root_module_path)?;

        // Check inheritance map
        let inheritance_map = visitor.get_inheritance_map();
        assert!(
            inheritance_map
                == HashMap::from([
                    (
                        "test_inheritance.base_models.UserBase".to_string(),
                        vec!["test_inheritance.base_models.BaseDataFrameModel".to_string()]
                    ),
                    (
                        "test_inheritance.base_models.BaseDataFrameModel".to_string(),
                        vec!["pandera.DataFrameModel".to_string()]
                    )
                ])
        );

        // Check class definitions
        let class_defs = visitor.get_class_defs();
        assert!(
            class_defs
                == &HashMap::from([
                    (
                        "test_inheritance.base_models.UserBase".to_string(),
                        ClassDef {
                            id: "test_inheritance.base_models.UserBase".to_string(),
                            name: "UserBase".to_string(),
                            module_path: root_module_path.join("base_models.py"),
                            parent_ids: vec![
                                "test_inheritance.base_models.BaseDataFrameModel".to_string()
                            ],
                            stmt_class_def: dummy_stmt_class_def.clone()
                        }
                    ),
                    (
                        "test_inheritance.base_models.BaseDataFrameModel".to_string(),
                        ClassDef {
                            id: "test_inheritance.base_models.BaseDataFrameModel".to_string(),
                            name: "BaseDataFrameModel".to_string(),
                            module_path: root_module_path.join("base_models.py"),
                            stmt_class_def: dummy_stmt_class_def.clone(),
                            parent_ids: vec!["pandera.DataFrameModel".to_string()]
                        }
                    )
                ])
        );

        // Check file_class_ids mapping
        let file_class_ids = visitor.get_file_class_ids();
        let expected_file_class_ids: HashMap<PathBuf, Vec<String>> = HashMap::from([(
            root_module_path.join("base_models.py"),
            vec![
                "test_inheritance.base_models.BaseDataFrameModel".to_string(),
                "test_inheritance.base_models.UserBase".to_string(),
            ],
        )]);

        // Compare entire maps
        assert_eq!(file_class_ids, &expected_file_class_ids);

        Ok(())
    }

    #[rstest]
    #[test]
    fn test_visitor_tracks_nested_inheritance(
        dummy_stmt_class_def: StmtClassDef<TextRange>,
    ) -> Result<()> {
        // Get the absolute path to the resources directory
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_module_path = project_root.join("resources/visitor/test_nested_inheritance");
        debug!("Root module path: {}", root_module_path.display());

        // Setup visitor with the test resources directory
        let mut visitor = ClassVisitor::new(root_module_path.clone());

        // Process the entire module
        visitor.process_module(&root_module_path)?;

        // Check inheritance map
        let inheritance_map = visitor.get_inheritance_map();
        assert!(
            inheritance_map
                == HashMap::from([
                    (
                        "test_nested_inheritance.nested_model.User".to_string(),
                        vec!["test_nested_inheritance.nested_model.BaseDataFrameModel".to_string()]
                    ),
                    (
                        "test_nested_inheritance.nested_model.UserExtension".to_string(),
                        vec!["test_nested_inheritance.nested_model.User".to_string()]
                    ),
                    (
                        "test_nested_inheritance.nested_model.BaseDataFrameModel".to_string(),
                        vec!["pandera.DataFrameModel".to_string()]
                    )
                ])
        );

        // Check class definitions
        let class_defs = visitor.get_class_defs();
        assert!(
            class_defs
                == &HashMap::from([
                    (
                        "test_nested_inheritance.nested_model.User".to_string(),
                        ClassDef {
                            id: "test_nested_inheritance.nested_model.User".to_string(),
                            name: "User".to_string(),
                            module_path: root_module_path.join("nested_model.py"),
                            parent_ids: vec![
                                "test_nested_inheritance.nested_model.BaseDataFrameModel"
                                    .to_string()
                            ],
                            stmt_class_def: dummy_stmt_class_def.clone()
                        }
                    ),
                    (
                        "test_nested_inheritance.nested_model.UserExtension".to_string(),
                        ClassDef {
                            id: "test_nested_inheritance.nested_model.UserExtension".to_string(),
                            name: "UserExtension".to_string(),
                            module_path: root_module_path.join("nested_model.py"),
                            parent_ids: vec![
                                "test_nested_inheritance.nested_model.User".to_string()
                            ],
                            stmt_class_def: dummy_stmt_class_def.clone()
                        }
                    ),
                    (
                        "test_nested_inheritance.nested_model.BaseDataFrameModel".to_string(),
                        ClassDef {
                            id: "test_nested_inheritance.nested_model.BaseDataFrameModel"
                                .to_string(),
                            name: "BaseDataFrameModel".to_string(),
                            module_path: root_module_path.join("nested_model.py"),
                            stmt_class_def: dummy_stmt_class_def.clone(),
                            parent_ids: vec!["pandera.DataFrameModel".to_string()]
                        }
                    )
                ])
        );

        // Check file_class_ids mapping
        let file_class_ids = visitor.get_file_class_ids();
        let expected_file_class_ids: HashMap<PathBuf, Vec<String>> = HashMap::from([(
            root_module_path.join("nested_model.py"),
            vec![
                "test_nested_inheritance.nested_model.BaseDataFrameModel".to_string(),
                "test_nested_inheritance.nested_model.User".to_string(),
                "test_nested_inheritance.nested_model.UserExtension".to_string(),
            ],
        )]);
        assert_eq!(file_class_ids, &expected_file_class_ids);
        Ok(())
    }

    #[rstest]
    #[test]
    fn test_visitor_tracks_alias_imports(
        dummy_stmt_class_def: StmtClassDef<TextRange>,
    ) -> Result<()> {
        // Get the absolute path to the resources directory
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let root_module_path = project_root.join("resources/visitor/test_alias_import");
        debug!("Root module path: {}", root_module_path.display());

        // Setup visitor with the test resources directory
        let mut visitor = ClassVisitor::new(root_module_path.clone());

        // Process the entire module
        visitor.process_module(&root_module_path)?;

        // Check inheritance map
        let inheritance_map = visitor.get_inheritance_map();
        assert!(
            inheritance_map
                == HashMap::from([
                    (
                        "test_alias_import.alias_import.AliasUser".to_string(),
                        vec!["visitor.base_models.BaseDataFrameModel".to_string()]
                    ),
                    (
                        "test_alias_import.base_models.BaseDataFrameModel".to_string(),
                        vec!["pandera.DataFrameModel".to_string()]
                    )
                ])
        );

        // Check class definitions
        let class_defs = visitor.get_class_defs();
        assert!(
            class_defs
                == &HashMap::from([
                    (
                        "test_alias_import.alias_import.AliasUser".to_string(),
                        ClassDef {
                            id: "test_alias_import.alias_import.AliasUser".to_string(),
                            name: "AliasUser".to_string(),
                            module_path: root_module_path.join("alias_import.py"),
                            parent_ids: vec!["visitor.base_models.BaseDataFrameModel".to_string()],
                            stmt_class_def: dummy_stmt_class_def.clone()
                        }
                    ),
                    (
                        "test_alias_import.base_models.BaseDataFrameModel".to_string(),
                        ClassDef {
                            id: "test_alias_import.base_models.BaseDataFrameModel".to_string(),
                            name: "BaseDataFrameModel".to_string(),
                            module_path: root_module_path.join("base_models.py"),
                            stmt_class_def: dummy_stmt_class_def.clone(),
                            parent_ids: vec!["pandera.DataFrameModel".to_string()]
                        }
                    )
                ])
        );

        // Check file_class_ids mapping
        let file_class_ids = visitor.get_file_class_ids();
        let expected_file_class_ids: HashMap<PathBuf, Vec<String>> = HashMap::from([
            (
                root_module_path.join("alias_import.py"),
                vec!["test_alias_import.alias_import.AliasUser".to_string()],
            ),
            (
                root_module_path.join("base_models.py"),
                vec!["test_alias_import.base_models.BaseDataFrameModel".to_string()],
            ),
        ]);
        assert!(file_class_ids == &expected_file_class_ids);

        Ok(())
    }
}
