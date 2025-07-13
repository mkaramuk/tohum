use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

/// Struct that represents a metadata.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateMetadata {
    pub version: String,
    pub name: String,
    pub authors: Vec<TemplateAuthor>,
    pub variables: Option<HashMap<String, TemplateCustomVariable>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateAuthor {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TemplateCustomVariableType {
    Number,
    String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateCustomVariable {
    pub description: String,
    pub r#type: TemplateCustomVariableType,
    pub default: Option<Value>,
}

/// Parses metadata struct from the given file
pub fn parse_metadata_from_file(path: &Path) -> Result<TemplateMetadata> {
    let content = fs::read_to_string(path)?;

    Ok(parse_metadata_from_content(&content)?)
}

/// Parses metadata struct from the given string
pub fn parse_metadata_from_content(content: &str) -> Result<TemplateMetadata> {
    let json: TemplateMetadata = serde_json::from_str(content)?;

    Ok(json)
}
