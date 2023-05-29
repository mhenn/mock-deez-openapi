use serde::Deserialize;
use serde_yaml;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct OpenAPI {
    paths: serde_yaml::Value,
    components: Components,
}

#[derive(Debug, Deserialize)]
struct Components {
    schemas: BTreeMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
enum Schema {
    AllOf(Vec<Schema>),
    Object {
        #[serde(rename = "type")]
        schema_type: Option<String>,
        format: Option<String>,
        // Add other fields specific to the "object" variant
    },
    // Add other variants as needed
}

pub fn get_path_keys(yaml: &OpenAPI) -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    if let Some(map) = yaml.paths.as_mapping() {
        keys = map
            .keys()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
    }
    keys
}

pub fn get_yaml(path: &str) -> Result<OpenAPI, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");
    let yaml: OpenAPI = serde_yaml::from_str(&contents)?;

    Ok(yaml)
}
