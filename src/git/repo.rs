//! Git repository detection and metadata extraction.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Metadata about the current git repository.
#[derive(Debug, Clone)]
pub struct RepoInfo {
    repo_root: PathBuf,
    head_sha: String,
    branch: Option<String>,
}

impl RepoInfo {
    /// Canonicalized path to the repository working directory.
    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    /// Full hex SHA of the HEAD commit.
    pub fn head_sha(&self) -> &str {
        &self.head_sha
    }

    /// Current branch name, or `None` if HEAD is detached.
    pub fn branch(&self) -> Option<&str> {
        self.branch.as_deref()
    }
}

/// Discover a git repository from `path` (searching parent directories)
/// and extract its metadata.
pub fn open_repo(path: &Path) -> Result<RepoInfo> {
    let repo = git2::Repository::discover(path)
        .context("not a git repository (or any parent up to mount point /)")?;

    let workdir = repo
        .workdir()
        .context("bare repositories are not supported")?;

    let repo_root = workdir
        .canonicalize()
        .context("failed to canonicalize repo root path")?;

    let head = repo.head().context("failed to read HEAD")?;

    let head_sha = head
        .target()
        .context("HEAD does not point to a valid commit")?
        .to_string();

    let branch = if head.is_branch() {
        head.shorthand().map(String::from)
    } else {
        None
    };

    Ok(RepoInfo {
        repo_root,
        head_sha,
        branch,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn open_repo_succeeds_in_project_dir() {
        let info = open_repo(&PathBuf::from(".")).expect("should detect git repo");

        assert!(info.repo_root().is_absolute());
        assert!(info.repo_root().exists());
        assert_eq!(info.head_sha().len(), 40);
        assert!(info.head_sha().chars().all(|c| c.is_ascii_hexdigit()));
        // Branch may be None in detached HEAD CI checkouts
        if let Some(branch) = info.branch() {
            assert!(!branch.is_empty());
        }
    }

    #[test]
    fn open_repo_fails_outside_git_repo() {
        let tmp = tempfile::tempdir().expect("should create temp dir");
        let result = open_repo(tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn repo_root_is_canonicalized() {
        let info = open_repo(&PathBuf::from(".")).expect("should detect git repo");
        let re_canonicalized = info
            .repo_root()
            .canonicalize()
            .expect("should canonicalize");
        assert_eq!(info.repo_root(), re_canonicalized.as_path());
    }

    #[test]
    fn detached_head_returns_no_branch() {
        // Create a temporary repo and detach HEAD
        let tmp = tempfile::tempdir().expect("should create temp dir");
        let repo = git2::Repository::init(tmp.path()).expect("should init repo");

        // Create an initial commit so HEAD is valid
        let sig = git2::Signature::now("test", "test@test.com").unwrap();
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        let commit_oid = repo
            .commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
            .unwrap();

        // Detach HEAD to the commit directly
        repo.set_head_detached(commit_oid).unwrap();

        let info = open_repo(tmp.path()).expect("should detect git repo");
        assert!(info.branch().is_none());
        assert_eq!(info.head_sha().len(), 40);
    }

    #[test]
    fn open_repo_rejects_bare_repository() {
        let tmp = tempfile::tempdir().expect("should create temp dir");
        git2::Repository::init_bare(tmp.path()).expect("should init bare repo");
        let result = open_repo(tmp.path());
        assert!(result.is_err());
    }

    #[test]
    fn open_repo_discovers_from_subdirectory() {
        let tmp = tempfile::tempdir().expect("should create temp dir");
        let repo = git2::Repository::init(tmp.path()).expect("should init repo");

        // Create an initial commit so HEAD is valid
        let sig = git2::Signature::now("test", "test@test.com").unwrap();
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "initial", &tree, &[])
            .unwrap();

        // Create a nested subdirectory and discover from there
        let sub = tmp.path().join("deep").join("nested");
        std::fs::create_dir_all(&sub).unwrap();
        let info = open_repo(&sub).expect("should discover repo from subdirectory");
        assert_eq!(info.repo_root(), tmp.path().canonicalize().unwrap());
    }
}
