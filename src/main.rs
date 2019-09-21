use git2::{Repository, RepositoryOpenFlags, ErrorCode};
use std::path::Path;
use std::process::exit;

fn main() {
    let open_flags = RepositoryOpenFlags::all();
    let paths: [&Path; 0] = []; //Empty path that doesn't need to be
    let repo = match Repository::open_ext(".", open_flags, paths.iter()) {
        Ok(repo) => repo,
        Err(_e) => exit(0),
    };

    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(_e) => return (),
    };
    let head = head.as_ref().and_then(|h| h.shorthand());


    print!("({})", head.unwrap_or("HEAD"));
}
