# MCP Server Research & Design

## Goal

Research MCP protocol requirements and design the `rx` MCP server architecture before implementation.

## Scope

- Determine which MCP transport to use (stdio vs HTTP) based on current agent integration patterns.
- Define the tool schemas for `list_comments`, `reply_to_comment`, `resolve_comment`, `get_diff_context`.
- Design how the MCP server coexists with the TUI (separate process? embedded server?).
- Identify the Rust MCP SDK or library to use.
- Produce a design document covering the above decisions.

## How to Verify

1. A design document exists at `docs/mcp-design.md` covering transport, tool schemas, architecture, and library choice.
2. The design is reviewed and approved before implementation begins.

## Dependencies

- [comment-crud](comment-crud.md) — need to understand the comment data model
- [session-persistence](session-persistence.md) — MCP server reads/writes sessions
