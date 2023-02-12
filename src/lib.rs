use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use directories::ProjectDirs;
use jsonschema::JSONSchema;
use serde_json::Value;
use thiserror::Error;

fn default_config(schema: &JSONSchema) -> Value {
    return Value::from("value");
}

#[derive(Error, Debug, PartialEq)]
pub enum StoreError {
    #[error("failed to initialize store")]
    InitError,
    #[error("invalid key-value set")]
    InvalidSet,
    #[error("invalid key")]
    InvalidKey,
}

#[derive(Debug)]
pub struct Store {
    path: PathBuf,
    schema: Option<JSONSchema>,
    config: Value,
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.to_string_lossy())
    }
}

impl Store {
    pub fn new(
        company_name: String,
        app_name: String,
        schema_path: Option<PathBuf>,
        path_override: Option<PathBuf>,
    ) -> Result<Store, StoreError> {
        // Initialize schema as None, then load if a path was provided
        let mut schema: Option<JSONSchema> = None;
        if let Some(schema_path) = schema_path {
            schema = Some(
                JSONSchema::compile(
                    &serde_json::from_reader(BufReader::new(
                        File::open(&schema_path).expect("Failed to open file"),
                    ))
                    .unwrap(),
                )
                .unwrap(),
            );
        }

        let mut config_path: PathBuf = PathBuf::new();

        // If a path override was provided, use that for config path
        if let Some(path) = path_override {
            // make sure the path is a directory that exists
            if path.is_dir() && path.exists() {
                config_path = path.join("config.json");
            } else {
                panic!("invalid override path");
            }
        } else {
            // See if a config json file exists in the UserData directory for the provided app name
            // Get the config directory
            if let Some(proj_dirs) = ProjectDirs::from("com", &company_name, &app_name) {
                config_path = PathBuf::from(proj_dirs.config_dir()).join("config.json");
            }
        }

        // If the file exists, load it
        if config_path.exists() {
            let mut config: Value = serde_json::from_reader(BufReader::new(
                File::open(&config_path).expect("Failed to open file"),
            ))
            .unwrap();
            // Validate the config against the schema
            if let Some(s) = &schema {
                // if the config passes validation, return the Store
                if s.is_valid(&config) {
                    return Ok(Store {
                        path: config_path,
                        config,
                        schema,
                    });
                } else {
                    // otherwise, generate a default config and return a store
                    config = default_config(s);
                    return Ok(Store {
                        path: config_path,
                        config,
                        schema,
                    });
                }
            } else {
                // if no schema, just return the store
                return Ok(Store {
                    path: config_path,
                    config,
                    schema,
                });
            }
        } else if let Some(s) = &schema {
            // if no config exists but there is a schema, create a default config and return the store
            let config: Value = default_config(s);
            return Ok(Store {
                path: config_path,
                config,
                schema,
            });
        } else {
            // if there is no config and no schema, error
            return Err(StoreError::InitError);
        }
    }

    // Get a value from the store, or the default if it doesn't exist, or error if it isn't a valid key
    pub fn get(&self, keys: String) -> Result<Value, StoreError> {
        let mut current_value: &Value = &self.config;
        for key in keys.split(".") {
            if let Some(v) = current_value.get(key) {
                current_value = v;
            } else {
                return Err(StoreError::InvalidKey);
            }
        }

        return Ok(current_value.clone());
    }

    // Set a key-value pair
    pub fn set(&mut self, keys: String, value: Value) -> Result<(), StoreError> {
        // make a copy of the config
        let mut config = self.config.clone();
        let mut current_value: &mut Value = &mut config;
        // // update the value in the config copy
        for key in keys.split(".") {
            if let Some(v) = current_value.get_mut(key) {
                current_value = v;
            } else {
                return Err(StoreError::InvalidSet);
            }
        }
        *current_value = value;

        if let Some(schema) = &self.schema {
            if schema.is_valid(&config) {
                self.config = config;
            } else {
                return Err(StoreError::InvalidSet);
            }
        }

        return Ok(());
    }

    // Check if a key exists
    pub fn has(&self, key: String) -> bool {
        let mut current_value: &Value = &self.config;
        for index in key.split(".") {
            if let Some(object) = current_value.get(index) {
                current_value = object;
            } else {
                return false;
            }
        }
        return true;
    }

    // Delete an object
    pub fn delete(key: String) -> Result<(), StoreError> {
        return Err(StoreError::InvalidKey);
    }

    // Reset keys to their default values as defined in the schema
    pub fn reset(key: Option<String>) -> Result<(), StoreError> {
        return Err(StoreError::InvalidKey);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests basic store initialization
    #[test]
    fn init_store() {
        let company_name: &str = "ACME";
        let app_name: &str = "Dynamite";
        if let Ok(store) = Store::new(company_name.to_string(), app_name.to_string(), None, None) {
            println!("{}", store);
        } else {
            panic!("Failed to initialize store")
        }
    }
    // tests schema validation
    #[test]
    fn test_validate_schema() {
        let schema_path = PathBuf::from("tests/config.schema.json");
        let schema: Value = serde_json::from_reader(BufReader::new(
            File::open(&schema_path).expect("Failed to open file"),
        ))
        .unwrap();
        let compiled_schema = JSONSchema::compile(&schema).expect("Failed to compile schema");

        let config_path = PathBuf::from("tests/config.json");
        let config: Value = serde_json::from_reader(BufReader::new(
            File::open(&config_path).expect("Failed to open file"),
        ))
        .unwrap();

        assert!(compiled_schema.is_valid(&config));
    }
    // tests if it can get a value from the store
    #[test]
    fn test_get() {
        let store = Store::new(
            String::from("ACME"),
            String::from("Dynamite"),
            Some(PathBuf::from("tests/config.schema.json")),
            Some(PathBuf::from("tests/")),
        )
        .expect("failed to load test config");
        let input = String::from("aSetting.i");
        assert_eq!(store.get(input).unwrap(), 400);
    }
    // test check if value is present
    #[test]
    fn test_has() {
        let store = Store::new(
            String::from("ACME"),
            String::from("Dynamite"),
            Some(PathBuf::from("tests/config.schema.json")),
            Some(PathBuf::from("tests/")),
        )
        .expect("failed to load test config");

        let input_true = String::from("anotherSetting.y");
        assert!(store.has(input_true));
        let input_false = String::from("not.here");
        assert!(!store.has(input_false));
    }
    #[test]
    fn test_set() {
        let mut store = Store::new(
            String::from("ACME"),
            String::from("Dynamite"),
            Some(PathBuf::from("tests/config.schema.json")),
            Some(PathBuf::from("tests/")),
        )
        .expect("failed to load test config");

        store
            .set(
                String::from("aSetting.i"),
                serde_json::to_value(10).unwrap(),
            )
            .unwrap();
        assert_eq!(store.get(String::from("aSetting.i")).unwrap(), 10);
        let result = store
            .set(
                String::from("aSetting.i"),
                serde_json::to_value(-10).unwrap(),
            )
            .unwrap_err();
        let expected = StoreError::InvalidSet;
        assert_eq!(result, expected);
    }
}
