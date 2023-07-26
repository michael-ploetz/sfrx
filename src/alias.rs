use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

use crate::errors::SfdxError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub alias: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AliasListOutput {
    pub status: i32,
    pub result: Vec<Alias>,
    pub warnings: Option<Vec<String>>,
}

pub fn alias_list() -> Result<AliasListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&["alias", "list", "--json"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: AliasListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}
