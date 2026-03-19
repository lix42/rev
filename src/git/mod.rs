//! Git integration (git2 crate).

mod repo;

pub use repo::{RepoInfo, open_repo};
