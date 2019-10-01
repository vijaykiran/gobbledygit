use core::fmt;
use git2::{ErrorCode, Repository, RepositoryOpenFlags, Status, Statuses};
use std::fmt::{Error, Formatter};
use std::path::Path;

struct GitStatus {
    new: i32,
    modified: i32,
    deleted: i32,
    renamed: i32,
    type_changed: i32,
    conflicted: i32,
}

impl fmt::Display for GitStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut fmt_string = String::new();
        // TODO: Too much duplication, refactor this into a impl fn?
        if self.new != 0 {
            fmt_string.push_str(format!("{}A", self.new).as_str())
        }

        if self.modified != 0 {
            fmt_string.push_str(format!("{}M", self.modified).as_str())
        }

        if self.deleted != 0 {
            fmt_string.push_str(format!("{}D", self.deleted).as_str())
        }

        if self.renamed != 0 {
            fmt_string.push_str(format!("{}R", self.renamed).as_str())
        }

        if self.type_changed != 0 {
            fmt_string.push_str(format!("{}T", self.type_changed).as_str())
        }
        if self.conflicted != 0 {
            fmt_string.push_str(format!("{}C", self.conflicted).as_str())
        }
        write!(f, "{}", fmt_string)
    }
}

pub fn repo() -> Option<Repository> {
    let open_flags = RepositoryOpenFlags::all();
    let paths: [&Path; 0] = []; //Empty path that doesn't need to be
    match Repository::open_ext(".", open_flags, paths.iter()) {
        Ok(repo) => Some(repo),
        Err(_e) => None,
    }
}

pub fn head_status(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => {
            None
        }
        Err(_e) => None,
    };

    head.as_ref()
        .and_then(|r| r.shorthand())
        .unwrap_or("HEAD")
        .to_string()
}

pub fn status(repo: &Repository) -> String {
    let result = repo.statuses(None);

    match result.as_ref() {
        Ok(statuses) => format!("{}", git_status(statuses)),
        Err(_e) => format!(""),
    }
}

fn git_status(statuses: &Statuses) -> GitStatus {
    //TODO: Currently combines both index and working tree status. Separate them
    let mut new = 0;
    let mut modified = 0;
    let mut renamed = 0;
    let mut deleted = 0;
    let mut type_changed = 0;
    let mut conflicted = 0;

    for entry in statuses.iter().filter(|e| e.status() != Status::CURRENT) {
        match entry.status() {
            s if s.contains(Status::INDEX_NEW) => new += 1,
            s if s.contains(Status::INDEX_MODIFIED) => modified += 1,
            s if s.contains(Status::INDEX_DELETED) => deleted += 1,
            s if s.contains(Status::INDEX_RENAMED) => renamed += 1,
            s if s.contains(Status::INDEX_TYPECHANGE) => type_changed += 1,
            s if s.contains(Status::CONFLICTED) => conflicted += 1,
            _ => (),
        };

        match entry.status() {
            s if s.contains(git2::Status::WT_MODIFIED) => modified += 1,
            s if s.contains(git2::Status::WT_DELETED) => deleted += 1,
            s if s.contains(git2::Status::WT_RENAMED) => renamed += 1,
            s if s.contains(git2::Status::WT_TYPECHANGE) => type_changed += 1,
            _ => (),
        };
    }

    GitStatus {
        new,
        modified,
        renamed,
        deleted,
        type_changed,
        conflicted,
    }
}
