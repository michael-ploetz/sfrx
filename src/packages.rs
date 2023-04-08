use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;

use crate::types::RecordAttributes;

use crate::errors::SfdxError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Package {
    pub id: String,
    pub subscriber_package_id: String,
    pub name: String,
    pub description: String,
    pub namespace_prefix: String,
    pub container_options: String,
    pub converted_from_package_id: String,
    pub alias: String,
    pub is_org_dependent: String,
    pub package_error_username: String,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageVersion {
    pub package_2_id: String,
    pub branch: String,
    pub tag: String,
    pub major_version: i32,
    pub minor_version: i32,
    pub patch_version: i32,
    pub build_number: i32,
    pub id: String,
    pub subscriber_package_version_id: String,
    pub converted_from_version_id: String,
    pub name: String,
    pub namespace_prefix: String,
    pub package_2_name: String,
    pub description: String,
    pub version: String,
    pub is_password_protected: bool,
    pub is_released: bool,
    pub created_date: String,
    pub last_modified_date: String,
    pub install_url: String,
    pub code_coverage: String,
    pub has_passed_code_coverage_check: bool,
    pub validation_skipped: bool,
    pub ancestor_id: String,
    pub ancestor_version: String,
    pub alias: String,
    pub is_org_dependent: String,
    pub release_version: String,
    pub build_duration_in_seconds: String,
    pub has_metadata_removed: String,
    pub created_by: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageListOutput {
    pub status: i32,
    pub result: Vec<Package>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageVersionListOutput {
    pub status: i32,
    pub result: Vec<PackageVersion>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDependencyValue {
    pub subscriber_package_version_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDependenciesField {
    pub ids: Vec<PackageDependencyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PackageDependenciesRecord {
    pub attributes: RecordAttributes,
    pub dependencies: PackageDependenciesField,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDependenciesRecords {
    pub records: Vec<PackageDependenciesRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageDependenciesListOutput {
    pub status: i32,
    pub result: PackageDependenciesRecords,
    pub warnings: Vec<String>,
}

pub fn package_list_for_dev_hub(dev_hub_username: &str) -> Result<PackageListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&[
            "package",
            "list",
            "-v",
            dev_hub_username,
            "--verbose",
            "--json",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: PackageListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}

pub fn package_versions_list_for_dev_hub(
    dev_hub_username: &str,
) -> Result<PackageVersionListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&[
            "package",
            "version",
            "list",
            "-v",
            dev_hub_username,
            "--verbose",
            "--json",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: PackageVersionListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}

pub fn package_versions_list_for_package(
    dev_hub_username: &str,
    package_id: &str,
) -> Result<PackageVersionListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&[
            "package",
            "version",
            "list",
            "-v",
            dev_hub_username,
            "-p",
            package_id,
            "--verbose",
            "--json",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: PackageVersionListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}

pub fn dependencies_list_for_package_version(
    dev_hub_username: &str,
    package_version_id: &str,
    package_version_key: &str,
) -> Result<PackageDependenciesListOutput, SfdxError> {
    let output = Command::new(env::var("SFDX_RUNTIME")?)
        .args(&[
            "data",
            "query",
            "--target-org",
            dev_hub_username,
            "-t",
            "-q",
            &format!("\"SELECT Dependencies FROM SubscriberPackageVersion WHERE Id =\'{}\' AND InstallationKey = \'{}\'\"", package_version_id, package_version_key),
            "--json",
        ])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    print!("{}", stdout);
    let output_json: PackageDependenciesListOutput = serde_json::from_str(&stdout)?;
    Ok(output_json)
}
