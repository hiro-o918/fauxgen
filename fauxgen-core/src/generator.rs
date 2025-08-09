use std::path::{Path, PathBuf};

use crate::factories::pandera::PanderaHandler;
use crate::visitor::ClassVisitor;
use anyhow::{Context, Result};

fn walk_dir(dir: &Path, suffix: &str) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.append(&mut walk_dir(&path, suffix)?);
        } else if path.extension().is_some_and(|ext| ext == suffix) {
            files.push(path);
        }
    }
    Ok(files)
}

/// A generator for factory methods
///
/// This struct holds a PanderaHandler and a ClassVisitor, and provides functionality
/// for generating factory methods for pandera DataFrameModel classes.
pub struct Generator {
    pandera_handler: PanderaHandler,
    visitor: ClassVisitor,
    module_dir: PathBuf,
}

impl Generator {
    /// Create a new Generator with the given module directory path
    pub fn new(module_dir: PathBuf) -> Self {
        Self {
            pandera_handler: PanderaHandler::new(),
            visitor: ClassVisitor::new(module_dir.clone()),
            module_dir,
        }
    }

    /// Generate factory code for a statement
    fn render_factory_code(&mut self, file_path: &Path) -> Result<Option<String>> {
        let class_ids = match self.visitor.get_class_ids_for_file(file_path) {
            Some(ids) => ids.clone(), // クローンして所有権を得る
            None => return Ok(None),
        };

        // 複数のクラスからファクトリーコードを生成し、結果を連結
        let mut factory_codes = Vec::new();
        for class_id in class_ids {
            let code = self
                .pandera_handler
                .generate_pandera_dataframe_factory(&class_id, &mut self.visitor)?;
            factory_codes.push(code);
        }

        // 空のベクトルの場合はNoneを返す
        if factory_codes.is_empty() {
            return Ok(None);
        }

        // 結果を連結して返す
        let combined_code = factory_codes
            .into_iter()
            .flatten() // Option<String>からStringを取り出す
            .collect::<Vec<_>>()
            .join("\n\n");

        if combined_code.is_empty() {
            Ok(None)
        } else {
            Ok(Some(combined_code))
        }
    }

    pub fn render_factory_code_from_file(&mut self, file: &Path) -> Result<Option<String>> {
        let factory_code = match self.render_factory_code(file)? {
            Some(code) => code,
            None => return Ok(None),
        };

        let import_statements = r#"import datetime
from typing import Any, TypedDict

import fauxgen as f


"#;
        Ok(Some(import_statements.to_string() + &factory_code))
    }

    /// Generate factory code for all files in a module and write to output directory
    fn write_factory_codes(&mut self, output_dir: &Path) -> Result<()> {
        // First process the module to collect class definitions and inheritance information
        self.visitor.process_module(&self.module_dir)?;

        let files = walk_dir(&self.module_dir, "py")?;
        for file in files {
            let factory_code = match self.render_factory_code_from_file(&file) {
                Ok(Some(code)) => code,
                // If the file does not contain a class definition, we skip it
                Ok(None) => {
                    continue;
                }
                // If there is an error, we log it and continue
                Err(e) => {
                    eprintln!(
                        "Error rendering factory code from file {}: {}",
                        file.display(),
                        e
                    );
                    continue;
                }
            };
            let relative_path = file.strip_prefix(&self.module_dir)?.to_path_buf();
            let factory_file = output_dir.join(relative_path).with_extension("py");

            if let Some(parent) = factory_file.parent() {
                create_dir_all_with_init(output_dir, parent)?;
            }
            std::fs::write(factory_file, factory_code)?;
        }
        Ok(())
    }
}

fn create_dir_all_with_init(from: &Path, target: &Path) -> Result<()> {
    let init_path = target.join("__init__.py");
    // When the target is the same as the from, we need to break the recursion
    // and create the directory and __init__.py file if required
    if from == target {
        if !target.exists() {
            std::fs::create_dir_all(target)
                .with_context(|| format!("Failed to create directory: {}", target.display()))?;
        }
        if !init_path.exists() {
            std::fs::write(&init_path, "")?;
        }
        return Ok(());
    }

    if let Some(parent) = target.parent() {
        create_dir_all_with_init(from, parent)?;
    }

    if !target.exists() {
        std::fs::create_dir(target)
            .with_context(|| format!("Failed to create directory: {}", target.display()))?;
    }
    if !init_path.exists() {
        std::fs::write(&init_path, "")?;
    }

    Ok(())
}

pub fn write_factory_codes(module_dir: &Path, output_dir: &Path) -> Result<()> {
    let mut generator = Generator::new(module_dir.into());
    generator.write_factory_codes(output_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use tempfile::TempDir;

    fn get_all_files(dir: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(get_all_files(&path)?);
            } else {
                files.push(path);
            }
        }
        Ok(files)
    }

    #[rstest]
    fn test_write_factory_codes() {
        let module_dir = PathBuf::from("./resources/generator/write_factory_codes/input");
        let output_dir = TempDir::new().unwrap().path().to_path_buf();
        let expected_dir = PathBuf::from("./resources/generator/write_factory_codes/expected");

        let mut generator = Generator::new(module_dir.clone());
        generator.write_factory_codes(&output_dir).unwrap();

        // Get all files from both directories
        let mut actual_files = get_all_files(&output_dir).unwrap();
        let mut expected_files = get_all_files(&expected_dir).unwrap();

        // Sort files to ensure consistent ordering
        actual_files.sort();
        expected_files.sort();

        // Compare the number of files
        assert_eq!(
            actual_files.len(),
            expected_files.len(),
            "Different number of files found. Expected: {:?}, Actual: {:?}",
            expected_files
                .iter()
                .map(|p| p.strip_prefix(&expected_dir).unwrap())
                .collect::<Vec<_>>(),
            actual_files
                .iter()
                .map(|p| p.strip_prefix(&output_dir).unwrap())
                .collect::<Vec<_>>()
        );

        // Compare each file
        for (actual_path, expected_path) in actual_files.iter().zip(expected_files.iter()) {
            let actual_relative = actual_path.strip_prefix(&output_dir).unwrap();
            let expected_relative = expected_path.strip_prefix(&expected_dir).unwrap();

            // Compare relative paths
            assert_eq!(
                actual_relative, expected_relative,
                "File paths don't match: {:?} != {:?}",
                actual_relative, expected_relative
            );

            // Compare file contents
            let actual_content = std::fs::read_to_string(actual_path).unwrap();
            let expected_content = std::fs::read_to_string(expected_path).unwrap();
            assert_eq!(
                actual_content, expected_content,
                "Content mismatch for file: {:?}",
                actual_relative
            );
        }
    }
}
