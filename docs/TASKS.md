# rx — Project Tasks

Task tracking for the rx project. See [design-spec.md](design-spec.md) for full context.

Each task has a detailed file in [tasks/](tasks/). Stories follow the phased build plan from the design spec.

**Legend:** `[ ]` = not started, `[~]` = in progress, `[x]` = done

---

## Story 1: Basic Diff Viewer

> Goal: Side-by-side diff with syntax highlighting, file panel, hunk navigation. Useful as a standalone diff viewer.

- [ ] [Git repo detection](tasks/git-repo-detection.md)
- [ ] [Git diff computation](tasks/git-diff-computation.md)
- [ ] [Config loading](tasks/config-loading.md)
- [ ] [TUI app skeleton](tasks/tui-app-skeleton.md)
- [ ] [Input system & modal keybindings](tasks/input-system.md)
- [ ] [Three-panel layout](tasks/three-panel-layout.md)
- [ ] [Status bar](tasks/status-bar.md)
- [ ] [File panel](tasks/file-panel.md)
- [ ] [Side-by-side diff view](tasks/side-by-side-diff-view.md)
- [ ] [Syntax highlighting](tasks/syntax-highlighting.md)
- [ ] [Word-level diff](tasks/word-level-diff.md)
- [ ] [Hunk navigation](tasks/hunk-navigation.md)
- [ ] [Unified diff view](tasks/unified-diff-view.md)
- [ ] [Mode selector UI](tasks/mode-selector-ui.md)
- [ ] [Help overlay](tasks/help-overlay.md)

## Story 2: Core Review Workflow

> Goal: Inline + global comments, comment panel, session persistence. The review workflow works end-to-end.

- [ ] [Session persistence](tasks/session-persistence.md)
- [ ] [Comment editor widget](tasks/comment-editor-widget.md)
- [ ] [Comment CRUD](tasks/comment-crud.md)
- [ ] [Comment panel](tasks/comment-panel.md)
- [ ] [Comment lifecycle](tasks/comment-lifecycle.md)
- [ ] [Inline comment anchors in diff view](tasks/inline-comment-anchors.md)

## Story 3: Export & Agent Loop

> Goal: Export comments in agent-friendly format. End-to-end review loop via copy-paste.

- [ ] [Export comments (text, markdown, JSON)](tasks/export.md)

## Story 4: Live Reload & Line Drift

> Goal: Auto-reload on file changes, re-anchor comments after edits, search.

- [ ] [File watcher](tasks/file-watcher.md)
- [ ] [Line drift resolver (Tier 1)](tasks/line-drift-resolver.md)
- [ ] [Manual re-anchor](tasks/manual-reanchor.md)
- [ ] [Search](tasks/search.md)

## Story 5: MCP Server

> Goal: AI agents can read/reply to comments directly via MCP protocol.

- [ ] [MCP server research & design](tasks/mcp-server-research.md)
- [ ] [MCP server implementation](tasks/mcp-server-implementation.md)

## Story 6: Session Management (Cross-cutting)

> Goal: Full session lifecycle — resume, close, archive, detect stale/merged sessions.

- [ ] [Session list & resume](tasks/session-list-resume.md)
- [ ] [Session close & archive](tasks/session-close.md)
- [ ] [Session staleness & merge detection](tasks/session-staleness-detection.md)

## Story 7: Polish & Accessibility

> Goal: Terminal compatibility, color themes, and accessibility. Not blocking for MVP but required for release.

- [ ] [Color degradation & accessibility](tasks/color-and-accessibility.md)
- [ ] [Configurable color themes](tasks/color-themes.md)
