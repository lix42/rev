#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub kind: CommentKind,
    pub body: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thread: Vec<Reply>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentKind {
    Inline {
        file: PathBuf,
        line: usize,
        anchor: Box<Anchor>,
    },
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Open,
    Resolved,
    Updated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub id: Uuid,
    pub author: Author,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Author {
    Human,
    Agent { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anchor {
    pub file: PathBuf,
    pub line: usize,
    pub context: [String; 5],
    pub col_range: Option<(usize, usize)>,
    pub ast_path: Option<String>,
}
