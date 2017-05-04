use serde::de::Deserialize;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn load_data<T>(path: &str) -> Result<T, String>
    where T: Deserialize {
    let file = File::open(Path::new(&format!("assets/{}", path)));
    match file {
        Ok(file) => parse_json(file),
        Err(err) => Err(format!("Error while loading file {}: {}", path, err))
    }
}

fn parse_json<T>(mut file: File) -> Result<T, String>
    where T: Deserialize {
    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
    match super::serde_json::from_str(&content) {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("Error while parsing json: {}", err))
    }
}