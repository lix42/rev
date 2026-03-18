#![allow(dead_code)]

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use super::comment::Comment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSession {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub diff_source: DiffSource,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffSource {
    Git {
        repo_root: PathBuf,
        base_ref: String,
        base_sha: String,
        branch: Option<String>,
    },
}

pub fn close() -> Result<()> {
    // TODO: Close the current review session
    Ok(())
}
