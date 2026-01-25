use std::collections::HashMap;

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeedAuthor {
    pub name: String,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SeedTemplateVariable {
    #[serde(rename = "type")]
    pub var_type: String,
    pub default: Option<serde_json::Value>,
    pub description: String,
    #[serde(default)]
    pub required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Seed {
    pub name: String,
    pub version: Version,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub authors: Vec<SeedAuthor>,
    pub variables: Option<HashMap<String, SeedTemplateVariable>>,
    #[serde(skip)]
    pub repo_path: String,
}
