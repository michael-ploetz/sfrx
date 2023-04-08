#![warn(missing_docs)]

mod alias;
mod errors;
mod orgs;
mod packages;
mod types;

fn main() {
    match orgs::org_list() {
        Ok(org_list_output) => {
            for scratch_org in org_list_output.result.scratch_orgs {
                print!("{}", scratch_org.org_name);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
