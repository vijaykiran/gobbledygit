use gobbledygit::{head_status, repo};
use std::process::exit;

fn main() {
    let repo = match repo() {
        Some(r) => r,
        None => exit(0), //No repo found!
    };

    print!("({})", head_status(&repo));
    exit(0)
}
