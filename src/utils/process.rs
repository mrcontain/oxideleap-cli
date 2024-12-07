use anyhow::Result;
use serde_yaml::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn process_yaml(path: PathBuf) -> Result<Vec<Value>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let values: Vec<Value> = serde_yaml::from_reader(reader)?;
    Ok(values)
}
