# Export Comments

## Goal

Implement the `rev export` command that outputs comments in multiple formats. The text format (`filename#line: comment`) is the MVP — designed for pasting into AI agents. Markdown and JSON formats follow.

## Approach

**Priority order: text format first (the killer feature), then markdown and JSON.**

- Read the current session from disk.
- Filter comments by status (default: open only) and optionally by file.
- Text format: `file#line: body` for inline, `file: Global — body` for global. Output to stdout.
- Markdown format: grouped by file with status indicators.
- JSON format: structured output for MCP / programmatic use.
- `rev export | pbcopy` should Just Work.

## Design

Three export functions in `src/review/export.rs`:

```rust
pub fn export_text(session: &ReviewSession, status_filter: &StatusFilter, file_filter: Option<&str>) -> String;
pub fn export_markdown(session: &ReviewSession, status_filter: &StatusFilter, file_filter: Option<&str>) -> String;
pub fn export_json(session: &ReviewSession, status_filter: &StatusFilter, file_filter: Option<&str>) -> String;
```

## How to Verify

1. `rev export` outputs only open comments in `file#line: body` format.
2. `rev export --format markdown` outputs markdown grouped by file.
3. `rev export --format json` outputs valid, parseable JSON.
4. `rev export --status all` includes resolved comments.
5. `rev export --file src/main.rs` filters to one file.
6. With no comments, output is empty (no error).
7. Unit tests for each format with known comment data.

## Dependencies

- [session-persistence](session-persistence.md)
- [comment-crud](comment-crud.md)
