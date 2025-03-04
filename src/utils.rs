use std::io::Read;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
pub struct Validators {
    pub locale: Vec<String>,
    pub keyboard_layout: Vec<String>,
    pub keyboard_variant: Vec<String>,
    pub keyboard_toggle: Vec<String>,
    pub sections: Vec<String>,
    pub source_id: Vec<String>,
}

pub fn load_validators_from_file(filepath: &str) -> std::io::Result<Validators> {
    let mut file = std::fs::File::open(filepath)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let validators: Validators =
        serde_json::from_str(&contents).expect("Failed to deserialize JSON validation.json");

    Ok(validators)
}
