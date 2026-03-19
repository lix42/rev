//! Git integration (git2 crate).

mod repo;

#[allow(unused_imports)] // RepoInfo will be used by downstream consumers
pub use repo::{RepoInfo, open_repo};
