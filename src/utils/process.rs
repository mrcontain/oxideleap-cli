use anyhow::Result;
use serde_yaml::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/**
 * @description:
 * @param {PathBuf} file_path
 * @return {anyhow::Result<Vec<Value>>}
 */
pub fn process_yaml(file_path: PathBuf) -> Result<Vec<Value>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let values: Vec<Value> = serde_yaml::from_reader(reader)?;
    Ok(values)
}
