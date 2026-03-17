# MCP Server Implementation

## Goal

Implement the MCP server that exposes review session tools so AI agents can interact with comments directly.

## Approach

- Implement the four MCP tools from design spec section 11.1:
  - `list_comments` — returns all open/resolved comments for the session.
  - `reply_to_comment` — agent posts a reply to a specific comment.
  - `resolve_comment` — agent marks a comment as resolved.
  - `get_diff_context` — returns diff context around a comment.
- The MCP server reads/writes the same session JSON files as the TUI.
- TUI auto-reloads when the session file changes (via file watcher), showing agent replies.

## How to Verify

1. Start the MCP server alongside or as part of `rev`.
2. An MCP client can call `list_comments` and get valid results.
3. `reply_to_comment` adds a reply that appears in the TUI.
4. `resolve_comment` changes comment status, visible in TUI.
5. `get_diff_context` returns the correct diff lines around a comment.

## Dependencies

- [mcp-server-research](mcp-server-research.md)
- [comment-crud](comment-crud.md)
- [session-persistence](session-persistence.md)
- [file-watcher](file-watcher.md)
