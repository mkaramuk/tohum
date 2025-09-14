use anyhow::{Error, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// URL address of the store.json
pub const DEFAULT_STORE_JSON_URL: &str =
    "https://raw.githubusercontent.com/mkaramuk/tohum/main/templates/store.json";

/// Struct that represents template metadata from JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreTemplateMetadata {
    pub description: Option<String>,
    pub versions: Vec<Version>,
}

/// Enum that represents either a path segment (object) or template metadata (object with description and versions)
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum StoreNode {
    /// Template endpoint - contains template metadata with description and versions
    Template(StoreTemplateMetadata),

    /// Path segment - contains nested paths or templates
    PathSegment(HashMap<String, StoreNode>),
}

/// Whole Store object
pub type Store = HashMap<String, StoreNode>;

/// Path <-> Template
pub type StoreTemplates = HashMap<String, StoreTemplateMetadata>;

/// Collects template versions from a field.
/// Simply it continues recursively until it finds the template metadata.
fn collect_templates_from_node(
    node: &StoreNode,
    current_path: &str,
    all_templates: &mut StoreTemplates,
) {
    match node {
        StoreNode::Template(template_metadata) => {
            // We have reached the template metadata, convert versions and store them
            all_templates.insert(current_path.to_string(), template_metadata.clone());
        }
        StoreNode::PathSegment(segments) => {
            // This is a path segment - continue traversing
            for (key, nested_node) in segments {
                let new_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}/{}", current_path, key)
                };

                collect_templates_from_node(nested_node, &new_path, all_templates);
            }
        }
    }
}

/// Collects all the template versions recursively from each field
fn collect_templates_from_store(store: &Store) -> StoreTemplates {
    let mut templates: StoreTemplates = HashMap::new();

    for (key, node) in store {
        collect_templates_from_node(node, key, &mut templates);
    }

    templates
}

/// Fetches and parses the store.json from the given URL
pub async fn fetch_store_template_list(store_json_url: &str) -> Result<StoreTemplates, Error> {
    // Fetch the store.json from the URL and parse it
    let response = reqwest::get(store_json_url).await?;
    let store_content = response.text().await?;
    let store: Store = serde_json::from_str(&store_content)?;

    Ok(collect_templates_from_store(&store))
}

/// Same as `fetch_store_template_list`. Uses default Store
pub async fn fetch_default_store_template_list() -> Result<StoreTemplates, Error> {
    fetch_store_template_list(DEFAULT_STORE_JSON_URL).await
}

/// Gets the latest version from the given template
pub fn get_latest_version_of_template(template: &StoreTemplateMetadata) -> Result<Version, Error> {
    if template.versions.len() == 0 {
        return Err(Error::msg("Template doesn't have any versions"));
    }

    let mut versions = template.versions.clone();
    versions.sort_by(|a, b| b.cmp(a));
    Ok(versions.first().unwrap().clone())
}
