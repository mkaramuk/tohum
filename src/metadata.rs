use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    version: String,
    name: String,
    group: String,
    author: Author,
    variables: HashMap<String, CustomVariable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    name: String,
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CustomVariableType {
    Number,
    String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomVariable {
    description: String,
    r#type: CustomVariableType,
    default: Value,
}

pub fn parse_metadata_from_file(path: &str) -> Result<Metadata> {
    let content = fs::read_to_string(path)?;
    let json: Metadata = serde_json::from_str(&content)?;

    Ok(json)
}

pub fn parse_metadata(content: &str) -> Result<Metadata> {
    let json: Metadata = serde_json::from_str(content)?;

    Ok(json)
}
