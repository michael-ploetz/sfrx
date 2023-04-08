<!-- readme rendered on crates.io -->

# SalesForce ~~Developer~~ Rust Experience

---

This library tries to wrap sfdx/sf cli commands into rust functions.

## How to Use it

```rust
use sfrx::orgs::*;

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
```

### WIP

This project is still in development until i find the time to port more commands and am satisfied with the state of documentation
