use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

use crate::errors::SfdxError;
use crate::types::RecordAttributes;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Org {
    pub username: String,
    pub access_token: String,
    pub instance_url: String,
    pub org_id: String,
    pub login_url: String,
    pub client_id: String,
    pub instance_api_version: String,
    pub instance_api_version_last_retrieved: String,
    pub is_dev_hub: bool,
    pub alias: Option<String>,
    pub is_default_dev_hub_username: bool,
    pub is_default_username: bool,
    pub last_used: String,
    pub connected_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScratchOrg {
    pub dev_hub_username: String,
    pub created: String,
    pub expiration_date: String,
    pub created_org_instance: String,
    pub tracks_source: bool,
    pub signup_username: String,
    pub created_by: String,
    pub created_date: String,
    pub dev_hub_org_id: String,
    pub dev_hub_id: String,
    pub attributes: RecordAttributes,
    pub org_name: String,
    pub edition: String,
    pub status: String,
    pub is_expired: bool,
    pub namespace: String,
    pub username: String,
    pub access_token: String,
    pub instance_url: String,
    pub org_id: String,
    pub login_url: String,
    pub client_id: String,
    pub instance_api_version: String,
    pub instance_api_version_last_retrieved: String,
    pub is_dev_hub: bool,
    pub alias: Option<String>,
    pub is_default_dev_hub_username: bool,
    pub is_default_username: bool,
    pub last_used: String,
    pub connected_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgListResult {
    pub non_scratch_orgs: Vec<Org>,
    pub scratch_orgs: Vec<ScratchOrg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgListOutput {
    pub status: i32,
    pub result: OrgListResult,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgOpenResult {
    pub org_id: String,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrgOpenOutput {
    pub status: i32,
    pub result: OrgOpenResult,
    pub warnings: Vec<String>,
}

pub fn org_list() -> Result<OrgListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&["org", "list", "--json"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: OrgListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}

pub fn open_org(username: &str) -> Result<OrgOpenOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&["force:org:open", "--target-org", username, "--json"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: OrgOpenOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}
