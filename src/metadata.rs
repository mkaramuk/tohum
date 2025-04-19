use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs, path::Path};

/// Struct that represents a metadata.json file
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub version: String,
    pub name: String,
    pub group: String,
    pub author: Author,
    pub variables: HashMap<String, CustomVariable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CustomVariableType {
    Number,
    String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomVariable {
    pub description: String,
    pub r#type: CustomVariableType,
    pub default: Option<Value>,
}

/// Parses metadata struct from the given file
pub fn parse_metadata_from_file(path: &Path) -> Result<Metadata> {
    let content = fs::read_to_string(path)?;

    Ok(parse_metadata_from_content(&content)?)
}

/// Parses metadata struct from the given content
pub fn parse_metadata_from_content(content: &str) -> Result<Metadata> {
    let json: Metadata = serde_json::from_str(content)?;

    Ok(json)
}
