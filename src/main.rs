use gobbledygit::{head_status, repo, status};
use std::process::exit;

fn main() {
    match repo() {
        Some(r) => {
            let res = format!("[{}] {}", head_status(&r), status(&r));
            print!("{}", res.trim());
        }
        None => exit(0), //No repo found!

    }
}
