

use std::option::Option;
use std::io::Result;
use std::fs;
use toml;
use super::project;


/// package information
pub(crate) struct Package {

    /// package name
    name: String,
    /// version
    version: String,
    /// authors
    authors: Option<Vec<String>>,
    /// description
    description: Option<String>,
    /// repository
    repository: Option<String>,
    /// keywords
    keywords: Option<Vec<String>>,
}

/// package implementation
impl Package {
    /// package name
    #[allow(dead_code)]
    pub(crate) fn name(&self) -> String {
        self.name.clone() 
    }

    /// version
    pub(crate) fn version(&self) -> String {
        self.version.clone()
    }
    /// authors
    pub(crate) fn authors(&self) -> Option<Vec<String>> {
        self.authors.clone()
    }
    /// description
    pub(crate) fn description(&self) -> Option<String> {
        self.description.clone()
    }
    /// repository
    pub(crate) fn repository(&self) ->  Option<String> {
        self.repository.clone()
    }

    /// keywords
    pub(crate) fn keywords(&self) -> Option<Vec<String>> {
        self.keywords.clone()
    }

    /// load package from default manifest
    pub(crate) fn load_from_manifest() -> Result<Package> {
        let mut cargo_path = project::root_dir(); 
        cargo_path.push("Cargo.toml");

        let cargo_contents = fs::read_to_string(cargo_path)?;

        let config = cargo_contents.parse::<toml::Value>().unwrap();
        let mut authors = None;
        let mut description = None;
        let mut repository = None;
        let mut keywords = None;

        let name = config["package"]["name"].as_str().unwrap().to_string();
        let version = 
            config["package"]["version"].as_str().unwrap().to_string();
        if let Some(val) = config["package"].get("authors") {
            if let toml::Value::Array(values) = val {
                let mut authors_vec = Vec::new();
                for auth_item in values {
                    authors_vec.push(auth_item.as_str().unwrap().to_string());
                }
                authors = Some(authors_vec);
            }
        }
        if let Some(val) = config["package"].get("keywords") {
            if let toml::Value::Array(values) = val {
                let mut keywords_vec = Vec::new();
                for keyword_item in values {
                    keywords_vec.push(
                        keyword_item.as_str().unwrap().to_string());
                }
                keywords = Some(keywords_vec);
            }
        }
        if let Some(val) = config["package"].get("description") {
            description = Some(val.as_str().unwrap().to_string());
        }
        if let Some(val) = config["package"].get("repository") {
            repository = Some(val.as_str().unwrap().to_string());
        }
        Ok(Package {
            name,
            version,
            authors,
            description,
            repository,
            keywords,
        })
    }
}

// vi: se ts=4 sw=4 et:
