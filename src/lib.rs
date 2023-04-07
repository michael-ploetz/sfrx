use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::process::Command;

#[derive(Debug)]
pub enum SfdxError {
    VarError(env::VarError),
    IoError(std::io::Error),
}

impl Error for SfdxError {}

impl Display for SfdxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SfdxError::VarError(err) => write!(f, "Environment variable error: {}", err),
            SfdxError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

pub fn org_list_json() -> Result<String, SfdxError> {
    match env::var("SFDX_RUNTIME") {
        Ok(val) => {
            let output = Command::new(val)
                .args(&["force:org:list", "--json"])
                .output()
                .map_err(|err| SfdxError::IoError(err))?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(stdout)
        }
        Err(e) => Err(SfdxError::VarError(e)),
    }
}
