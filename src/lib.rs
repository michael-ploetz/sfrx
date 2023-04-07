use std::process::Command;

pub fn org_list_json() -> Result<String, std::io::Error> {
    let output = Command::new("sfdx")
        .args(&["force:org:list", "--json"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}