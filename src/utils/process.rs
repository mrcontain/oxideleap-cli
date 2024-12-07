use anyhow::Error as AnyError;
use anyhow::Result;
use serde_yaml::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn process_yaml(path: PathBuf) -> Result<Vec<Value>, AnyError> {
    let file = File::open(path).map_err(AnyError::new)?;
    let reader = BufReader::new(file);
    let values: Vec<Value> = serde_yaml::from_reader(reader).map_err(AnyError::new)?;
    Ok(values)
}
