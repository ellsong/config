use std::path::PathBuf;

use directories::ProjectDirs;
use serde_json::Value;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("failed to initialize store")]
    InitError,
    #[error("invalid key-value set")]
    InvalidSet,
    #[error("invalid key")]
    InvalidKey
}

pub struct Store {
    path: PathBuf,
    // schema: serde_json::Value,
    // config: serde_json::Value
}

impl Store {
    pub fn new(company_name: String, app_name: String) -> Result<Store, StoreError> {
        // See if a config json file exists in the UserData directory for the provided app name

        if let Some(proj_dirs) = ProjectDirs::from("com", &company_name, &app_name) {
            let config_dir = PathBuf::from(proj_dirs.config_dir());

            return Ok(Store {path: config_dir});
        } else {
            return Err(StoreError::InitError);
        }

        // If the file exists, load it

        // Validate the config against the schema

        // If it passes, be happy and return the config object

            // if it fails, or if the file doesn't exist, generate a new config using the default values from the schema, return the config object

        
    }

    // Get a value from the store, or the default if it doesn't exist, or error if it isn't a valid key
    pub fn get(key: String) -> Result<(), StoreError> {
        // See if a config json file exists in the UserData directory for the provided app name

        // If the file exists, load it

        // Validate the config against the schema

        return Err(StoreError::InvalidKey)
    }

    // Set a key-value pair
    pub fn set(key: String, value: Value) -> Result<(), StoreError> {
        // Validate value against schema

        // If it passes, write it to the file
        return Err(StoreError::InvalidSet);
    }

    // Check if a key exists
    pub fn has(key: String) -> bool {
        return false;
    }

    // Delete an object
    pub fn delete(key: String) -> Result<(), StoreError> {
        return Err(StoreError::InvalidKey)
    }

    // Reset keys to their default values as defined in the schema
    pub fn reset(key: Option<String>) -> Result<(), StoreError> {
        return Err(StoreError::InvalidKey)
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
