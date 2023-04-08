mod alias;
mod errors;
mod orgs;
mod packages;
mod types;

fn main() {
    match alias::alias_list() {
        Ok(alias_list_output) => {
            print!("{:?}", alias_list_output.result);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
