use crate::{
    fields::{
        Field, FieldType, FieldTypeAny, FieldTypeBool, FieldTypeDate, FieldTypeDatetime,
        FieldTypeFloat, FieldTypeInt, FieldTypeString,
    },
    strings,
    visitor::{ClassDef, ClassID, ClassIDExt, ClassVisitor},
};
use anyhow::{Ok, Result};
use askama::Template;
use log::debug;
use rustpython_parser::{
    ast::{ExprAttribute, ExprCall, ExprName, ExprSubscript, StmtAnnAssign},
    text_size::TextRange,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct ParsedPanderaClass {
    fields: Vec<Field>,
}

pub struct PanderaHandler {
    _phantom: std::marker::PhantomData<TextRange>,
    int_type_regex: regex::Regex,
    float_type_regex: regex::Regex,
    bool_type_regex: regex::Regex,
    string_type_regex: regex::Regex,
    date_type_regex: regex::Regex,
    datetime_type_regex: regex::Regex,
    parsed_pandera_cache: HashMap<ClassID, ParsedPanderaClass>,
}

#[derive(Default)]
struct PanderaFieldParameter {
    ge: Option<f64>,
    le: Option<f64>,
    nullable: Option<bool>,
    description: Option<String>,
}

#[derive(Template)]
#[template(path = "pandera_factory.py.tpl")]
struct PanderaFactoryTemplate<'a> {
    record_class_name: &'a str,
    record_factory_name: &'a str,
    dataframe_schema_class_name: &'a str,
    fields: Vec<Field>,
}

impl Default for PanderaHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PanderaHandler {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            int_type_regex: regex::Regex::new(r"^(pa\.(Int|Int8|Int16|Int32|Int64)|pd\.(Int8Dtype|Int16Dtype|Int32Dtype|Int64Dtype))$")
                .unwrap(),
            float_type_regex: regex::Regex::new(r"^(pa\.(Float|Float16|Float32|Float64|Float128)|pd\.(Float32Dtype|Float64Dtype))$")
                .unwrap(),
            bool_type_regex: regex::Regex::new(r"^(pa\.Bool|pd\.BooleanDtype)$").unwrap(),
            string_type_regex: regex::Regex::new(r"^(pa\.String|pd\.StringDtype)$").unwrap(),
            date_type_regex: regex::Regex::new(r"^(pa\.Date)$").unwrap(),
            datetime_type_regex: regex::Regex::new(r"^(pa\.(DateTime|Timestamp)|pd\.DatetimeTZDtype)$").unwrap(),
            parsed_pandera_cache: HashMap::new(),
        }
    }

    pub fn generate_pandera_dataframe_factory(
        &mut self,
        class_id: &ClassID,
        visitor: &mut ClassVisitor,
    ) -> Result<Option<String>> {
        let generic_class_defs =
            visitor.get_class_defs_of_base(class_id, &"pandera.DataFrameModel".to_string());
        let pandas_class_defs =
            visitor.get_class_defs_of_base(class_id, &"pandera.pandas.DataFrame".to_string());
        let this_class_def = visitor
            .get_class_def(class_id)
            .ok_or_else(|| anyhow::anyhow!("Class definition not found for {}", class_id))?
            .clone();
        let all_class_defs = generic_class_defs
            .into_iter()
            .chain(pandas_class_defs)
            .chain(vec![this_class_def])
            .collect::<Vec<_>>();
        debug!(
            "Generating factory code for class {} with definitions: {:?}",
            class_id.class_name(),
            all_class_defs
                .iter()
                .map(|class_def| class_def.name.as_str())
                .collect::<Vec<_>>()
        );
        let parsed_pandera_classes = all_class_defs
            .into_iter()
            .map(|class_def| self.parse_pandera_dataframe_model(class_def))
            .collect::<Result<Vec<_>>>()?;
        let class_name = class_id.class_name();
        // create a set of field names to track duplicates
        let mut field_names = HashSet::new();
        let mut fields = Vec::new();
        for parsed_class in parsed_pandera_classes.into_iter().flatten() {
            for field in parsed_class.fields {
                if field_names.insert(field.name.clone()) {
                    // Only add the field if it is not already present
                    fields.push(field);
                }
            }
        }
        debug!("Final fields for class {}: {:?}", class_name, fields);

        let record_class_name = &format!("{}Record", class_name);
        Ok(Some(
            PanderaFactoryTemplate {
                record_class_name: &format!("{}Record", class_name),
                record_factory_name: strings::to_snake_case(record_class_name).as_str(),
                dataframe_schema_class_name: class_name,
                fields,
            }
            .render()?,
        ))
    }

    fn parse_pandera_dataframe_model(
        &mut self,
        class_def: ClassDef<TextRange>,
    ) -> Result<Option<ParsedPanderaClass>> {
        if let Some(cache) = self.parsed_pandera_cache.get(&class_def.id) {
            return Ok(Some(cache.clone()));
        }
        let fields: Vec<Field> = class_def
            .stmt_class_def
            .body
            .iter()
            .filter_map(|stmt| stmt.as_ann_assign_stmt())
            .map(|ann_assign| self.get_field_from_ann_assign(ann_assign))
            .collect::<Result<Vec<Option<Field>>>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(Some(ParsedPanderaClass { fields }))
    }

    fn get_pandera_parameter_from_field_value(
        &self,
        field_value: &ExprCall<TextRange>,
    ) -> Result<Option<PanderaFieldParameter>> {
        let mut ge = None;
        let mut le = None;
        let mut nullable = None;
        let mut description = None;

        for keyword in field_value.keywords.iter() {
            let Some(arg) = &keyword.arg else {
                continue;
            };
            let Some(value) = keyword.value.as_constant_expr() else {
                continue;
            };
            match arg.as_str() {
                "ge" => {
                    let ge_float = value.value.as_float().cloned();
                    let ge_int = value
                        .value
                        .as_int()
                        .map(|v| v.to_string().parse::<f64>())
                        .transpose()?;
                    ge = ge_float.or(ge_int);
                }
                "le" => {
                    let le_float = value.value.as_float().cloned();
                    let le_int = value
                        .value
                        .as_int()
                        .map(|v| v.to_string().parse::<f64>())
                        .transpose()?;
                    le = le_float.or(le_int);
                }
                "nullable" => nullable = value.value.as_bool().cloned(),
                "description" => description = value.value.as_str().map(|s| s.to_string()),
                _ => {}
            }
        }
        Ok(Some(PanderaFieldParameter {
            ge,
            le,
            nullable,
            description,
        }))
    }

    fn get_field_type_from_attribute_expr(
        &self,
        attribute: &ExprAttribute<TextRange>,
        pandera_field_parameter: &PanderaFieldParameter,
    ) -> Option<FieldType> {
        // extract part of `pa` and `pd`
        // pa.Int -> pa
        // pd.Int8Dtype -> pd
        let module_name = attribute.value.as_name_expr()?.id.as_str();
        // extract part of `Int` and `Float`
        // pa.Int -> Int
        // pd.Int8Dtype -> Int8Dtype
        let class_name = attribute.attr.as_str();
        // get the full type definition like `pa.Int` or `pd.Int8Dtype`
        let type_def = format!("{}.{}", module_name, class_name);

        // check if the class name is in the regex
        if self.int_type_regex.is_match(&type_def) {
            return Some(FieldType::Int(FieldTypeInt {
                ge: pandera_field_parameter.ge,
                le: pandera_field_parameter.le,
                description: pandera_field_parameter.description.clone(),
            }));
        }
        if self.float_type_regex.is_match(&type_def) {
            return Some(FieldType::Float(FieldTypeFloat {
                ge: pandera_field_parameter.ge,
                le: pandera_field_parameter.le,
                description: pandera_field_parameter.description.clone(),
            }));
        }
        if self.bool_type_regex.is_match(&type_def) {
            return Some(FieldType::Bool(FieldTypeBool {
                description: pandera_field_parameter.description.clone(),
            }));
        }
        if self.string_type_regex.is_match(&type_def) {
            return Some(FieldType::String(FieldTypeString {
                min_length: None,
                max_length: None,
                description: pandera_field_parameter.description.clone(),
            }));
        }
        if self.date_type_regex.is_match(&type_def) {
            return Some(FieldType::Date(FieldTypeDate {
                description: pandera_field_parameter.description.clone(),
            }));
        }
        if self.datetime_type_regex.is_match(&type_def) {
            return Some(FieldType::Datetime(FieldTypeDatetime {
                description: pandera_field_parameter.description.clone(),
            }));
        }
        eprintln!("Unknown field type: {}, which handled as Any", type_def);
        Some(FieldType::Any(FieldTypeAny {
            description: pandera_field_parameter.description.clone(),
        }))
    }

    fn get_field_type_from_name_expr(
        &self,
        name_expr: &ExprName<TextRange>,
        pandera_field_parameter: &PanderaFieldParameter,
    ) -> FieldType {
        // handle type of python like int, float, str, bool
        if name_expr.id.as_str() == "int" {
            return FieldType::Int(FieldTypeInt {
                ge: pandera_field_parameter.ge,
                le: pandera_field_parameter.le,
                description: pandera_field_parameter.description.clone(),
            });
        }
        if name_expr.id.as_str() == "float" {
            return FieldType::Float(FieldTypeFloat {
                ge: pandera_field_parameter.ge,
                le: pandera_field_parameter.le,
                description: pandera_field_parameter.description.clone(),
            });
        }
        if name_expr.id.as_str() == "str" {
            return FieldType::String(FieldTypeString {
                min_length: None,
                max_length: None,
                description: pandera_field_parameter.description.clone(),
            });
        }
        if name_expr.id.as_str() == "bool" {
            return FieldType::Bool(FieldTypeBool {
                description: pandera_field_parameter.description.clone(),
            });
        }
        eprintln!("Unknown field type: {}, which handled as Any", name_expr.id);
        FieldType::Any(FieldTypeAny {
            description: pandera_field_parameter.description.clone(),
        })
    }

    fn get_field_type_from_series_ann(
        &self,
        field_ann: &ExprSubscript<TextRange>,
        pandera_field_parameter: &PanderaFieldParameter,
    ) -> Option<FieldType> {
        if field_ann
            .value
            .as_name_expr()
            .is_some_and(|name| name.id.as_str() != "Series")
        {
            // not a Series type
            eprintln!("Unknown field type: {:?}", field_ann.value);
            return None;
        }

        // handle type of pandera like pa.Int
        if let Some(attribute) = field_ann.slice.as_attribute_expr() {
            return self.get_field_type_from_attribute_expr(attribute, pandera_field_parameter);
        }
        // handle type of python like int, float, str, bool
        if let Some(attribute) = field_ann.slice.as_name_expr() {
            return Some(self.get_field_type_from_name_expr(attribute, pandera_field_parameter));
        }

        // handle type with Annotated like `pd.Series[Annotated[pd.DatetimeTZDtype, "us", "Asia/Tokyo"]]`
        if let Some(annotated) = field_ann.slice.as_subscript_expr() {
            if annotated.value.as_name_expr().map(|name| name.id.as_str()) == Some("Annotated") {
                if let Some(tuple) = annotated.slice.as_tuple_expr() {
                    // Extract first element of the tuple `pd.DatetimeTZDtype`
                    if let Some(first_type) = tuple.elts.first() {
                        if let Some(attribute) = first_type.as_attribute_expr() {
                            return self.get_field_type_from_attribute_expr(
                                attribute,
                                pandera_field_parameter,
                            );
                        }
                        if let Some(name_expr) = first_type.as_name_expr() {
                            return Some(self.get_field_type_from_name_expr(
                                name_expr,
                                pandera_field_parameter,
                            ));
                        }
                    }
                }
            }
        }
        eprintln!("Unknown field type: {:?}", field_ann.slice);
        None
    }

    fn get_field_from_ann_assign(
        &self,
        ann_assign_stmt: &StmtAnnAssign<TextRange>,
    ) -> Result<Option<Field>> {
        let Some(name) = ann_assign_stmt
            .target
            .as_name_expr()
            .map(|name_expr| name_expr.id.to_string())
        else {
            return Ok(None);
        };
        let Some(field_ann) = ann_assign_stmt.annotation.as_subscript_expr() else {
            return Ok(None);
        };

        let field_value = ann_assign_stmt.value.as_ref().map(|v| v.as_call_expr());
        let pandera_field_param = match field_value
            .and_then(|v| v)
            .map(|v| self.get_pandera_parameter_from_field_value(v))
        {
            Some(result) => result?.unwrap_or_default(),
            None => PanderaFieldParameter::default(),
        };

        let Some(field_type) = self.get_field_type_from_series_ann(field_ann, &pandera_field_param)
        else {
            return Ok(None);
        };

        Ok(Some(Field {
            name,
            field_type,
            nullable: pandera_field_param.nullable.unwrap_or(false),
        }))
    }
}
