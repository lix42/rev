# rx — TUI Diff/Review Tool Design Specification

## 1. Overview

`rx` (review experience) is a TUI-native code review tool designed for AI agent coding workflows. It renders diffs with syntax highlighting and provides a structured annotation layer — inline and global comments — that can be exported in a format AI agents understand and act on. The long-term vision is a local PR review loop between human and agent, without leaving the terminal.

### 1.1 Core Value Proposition

- **Fast**: single Rust binary, instant startup, no runtime dependencies
- **Easy to install**: `brew install rx`
- **Easy to use**: launch from the shell, keyboard-driven throughout
- **Review-first**: the core workflow is reading diffs and writing structured comments, not merging
- **Agent-aware**: export comments in `filename#line: comment` format that AI coding agents can parse and act on
- **Live**: auto-reloads when files change on disk, re-anchors comments to shifted lines

### 1.2 Why This Tool Exists

AI coding agents (Claude Code, Cursor, Codex) produce large, multi-file diffs. The review bottleneck isn't reading the diff — it's annotating intent back to the agent in a structured way. Current options all have friction:

| Option | Problem |
|--------|---------|
| Claude Code's diff UI | GUI-only, coarse granularity, no comment export |
| `git diff` in terminal | Read-only, no annotation layer |
| Open a PR on GitHub | Requires a remote, breaks local flow |
| Manual copy-paste notes | Unstructured, easy to lose file/line context |

`rx` is **GitHub PR review, but local, TUI-native, and agent-aware.**

---

## 2. Tech Stack

| Concern | Choice | Rationale |
|---------|--------|-----------|
| Language | Rust | Zero-cost startup, single binary, `brew`-installable, memory safe |
| TUI framework | [Ratatui](https://ratatui.rs/) | De facto modern Rust TUI library; replaces `tui-rs`; active, well-documented |
| Diff engine | `similar` crate | Battle-tested, supports word-level and line-level diffs, used by `cargo` itself |
| Syntax highlighting | `syntect` crate | Same engine as VS Code's TextMate grammars; works offline, no subprocess |
| File watching | `notify` crate | Cross-platform, async-friendly, inotify/FSEvents/kqueue under the hood |
| Async runtime | `tokio` | For file watcher events, MCP server, and future extensibility |
| Config | `toml` + `dirs` crate | `~/.config/rx/config.toml`, feels native |
| Distribution | Homebrew tap | Single binary makes `brew install` straightforward |

---

## 3. Architecture

### 3.1 High-Level Layout

```
┌─────────────────────────────────────────────────────────────┐
│                         App Shell                            │
│  ┌──────────┐  ┌─────────────────────┐  ┌────────────────┐  │
│  │  File    │  │    Diff Viewport     │  │ Comment Panel  │  │
│  │  Panel   │  │  ┌───────┬─────────┐│  │                │  │
│  │          │  │  │ OLD   │ NEW     ││  │ ● line 42      │  │
│  │ ● = has  │  │  │       │  ←←← ●  ││  │   token expiry │  │
│  │   comment│  │  │       │         ││  │                │  │
│  │          │  │  └───────┴─────────┘│  │ ○ line 55      │  │
│  └──────────┘  │   inline comment    │  │   magic number │  │
│                │   anchors shown     │  │   [RESOLVED]   │  │
│                └─────────────────────┘  └────────────────┘  │
│  ─────────────────────────────────────────────────────────  │
│  [c] comment  [r] resolve  [e] export  [/] search  [?] help │
└─────────────────────────────────────────────────────────────┘
```

Three panels:

- **File Panel** (left): lists files with diffs, annotated with `●` when a file has comments
- **Diff Viewport** (center): side-by-side or unified diff with syntax highlighting and inline comment anchors
- **Comment Panel** (right): lists all comments for the current file with status indicators

All three panels are independently toggleable to maximize viewport space.

### 3.2 Module Structure

```
src/
├── main.rs                 # Entry point, CLI parsing, app bootstrap
├── app.rs                  # Top-level state machine, event loop
│
├── diff/                   # Pure diff logic, no UI concerns
│   ├── engine.rs           # Wraps `similar`, produces DiffResult with hunk metadata
│   ├── highlighter.rs      # Runs `syntect` on both sides, merges with diff annotations
│   ├── word_diff.rs        # Intra-line word-level diffing
│   └── annotator.rs        # Overlays comment anchors onto diff lines
│
├── review/                 # Core review system
│   ├── session.rs          # CRUD for sessions and comments
│   ├── comment.rs          # Comment data model, lifecycle (Open/Resolved/Updated)
│   ├── export.rs           # Generates filename#line: comment report
│   └── resolver.rs         # Re-anchors comments after file changes (line drift)
│
├── watcher/                # File system events
│   └── mod.rs              # Runs on tokio task, sends FileChanged events via mpsc
│
├── ui/                     # Ratatui widgets
│   ├── file_panel.rs       # Tree/list of files with diff stats and comment indicators
│   ├── diff_view.rs        # Main split viewport, synchronized scrolling, comment markers
│   ├── comment_panel.rs    # Right sidebar listing comments with status
│   ├── comment_editor.rs   # Lightweight floating overlay for writing/editing comments
│   ├── status_bar.rs       # Mode indicator, keybinding hints, search state
│   └── mode_selector.rs    # Startup screen for choosing diff source
│
├── input/                  # Keyboard handling
│   ├── keymap.rs           # Configurable key bindings loaded from config
│   ├── actions.rs          # Enum of Actions decoupled from key codes (testable)
│   └── modal.rs            # Vi-inspired modal system: Normal / Visual / Edit / Search
│
├── git/                    # Git integration
│   └── mod.rs              # Diff source resolution, branch detection, merge-base queries
│
├── mcp/                    # MCP server (Phase 5)
│   └── mod.rs              # Exposes session as MCP tools
│
└── config/                 # Configuration
    └── mod.rs              # Loads and validates config.toml
```

### 3.3 Event Loop

```
User input (key/mouse)          File system events (notify)
        │                               │
        ▼                               ▼
  Event dispatcher              Batch into single update
        │                       (collect all changes within
        │                        debounce window)
   ┌────┴──────────┐                    │
   │    │          │                    ▼
  File  Diff    Comment         Re-read all changed files
  Panel Viewport Panel          Recompute diffs
   │    │          │            Re-anchor all comments once
   │    │          │                    │
   └────┴──────────┴────────────────────┘
                   │
                   ▼
            State update → Re-render
```

The file watcher runs on a separate tokio task. File system events are **batched** within the debounce window (default 200ms) — when an AI agent writes multiple files in rapid succession, all changes are collected and processed as a single update. The diff is recomputed, comments are re-anchored via the resolver once across all changed files, and the viewport re-renders — all without interrupting the user's focus position. This avoids intermediate re-anchor noise and wasted computation from processing partial writes.

---

## 4. Core Data Model

### 4.1 Review Session

```rust
struct ReviewSession {
    id: Uuid,
    created_at: DateTime<Utc>,
    diff_source: DiffSource,
    comments: Vec<Comment>,
}

enum DiffSource {
    Git {
        repo_root: PathBuf,     // canonicalized absolute path (see 9.1)
        base_ref: String,       // resolved to concrete SHA at creation time
        base_sha: String,       // the actual commit SHA (pinned)
        branch: Option<String>, // e.g. "feature/auth"
    },
    // Future: Files, Directory variants
}
```

### 4.2 Comment

```rust
struct Comment {
    id: Uuid,
    kind: CommentKind,
    body: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    thread: Vec<Reply>,
}

enum CommentKind {
    Inline { file: PathBuf, line: usize, anchor: Anchor },
    Global,
}

enum Status {
    Open,
    Resolved,
    Updated,                    // Re-opened with new text after partial fix
}
```

### 4.3 Reply

Replies support threaded conversation between human and agent (used by MCP in Phase 5, but the data model is defined now for forward compatibility).

```rust
struct Reply {
    id: Uuid,
    author: Author,
    body: String,
    created_at: DateTime<Utc>,
}

enum Author {
    Human,
    Agent { name: String },     // e.g. "claude-code", "cursor"
}
```

### 4.4 Comment Lifecycle

```
OPEN → RESOLVED
     ↘ UPDATED (re-opened with new text, e.g. after agent made a partial fix)
```

### 4.5 Anchor (for line drift handling)

```rust
struct Anchor {
    file: PathBuf,
    line: usize,                        // best-effort, may drift
    context: [String; 5],               // 2 lines before, the line, 2 lines after
    col_range: Option<(usize, usize)>,  // optional column range
    ast_path: Option<String>,           // tree-sitter path (Tier 2)
}
```

### 4.6 Session Storage

Sessions are stored as local JSON files:

```
~/.local/share/rx/sessions/<session_hash>.json
```

The session hash is derived from `sha256(canonical_repo_root + base_ref + branch_name)`.

No database needed. Simple, portable, inspectable.

---

## 5. Git Diff Modes

The tool is scoped to `git diff` only for v1. Even within git, the diff surface area is wide. It is modeled as two axes:

### 5.1 Comparison Axis (what changed)

| Comparison | Git equivalent |
|------------|---------------|
| Working tree vs. index (staged) | `git diff` |
| Working tree vs. HEAD | `git diff HEAD` |
| Index vs. HEAD | `git diff --cached` |
| Working tree vs. any commit/branch | `git diff <ref>` |
| Untracked files (opt-in) | `git diff` + `git ls-files --others` |

### 5.2 Scope Axis (how much)

Whole repo, specific paths, or a specific file.

### 5.3 Mode Selector UI

On launch inside a git repo, `rx` detects the repo and presents a mode selector instead of requiring CLI flags:

```
┌─ Open Review Session ─────────────────────────────────┐
│                                                        │
│  Repo:  ~/projects/myapp  (branch: feature/auth)       │
│                                                        │
│  ❯ Unstaged changes          (working tree vs index)   │
│    Staged changes            (index vs HEAD)           │
│    All local changes         (working tree vs HEAD)    │
│    Compare to branch...      main ▾                    │
│    Compare to commit...                                │
│    Include untracked files   [ ]                       │
│                                                        │
│  [Enter] Open    [r] Resume existing session           │
└────────────────────────────────────────────────────────┘
```

This collapses CLI complexity into a single guided moment and makes it easy to re-open with different params without losing comments.

### 5.4 Behavior Outside a Git Repo

If `rx` is launched in a directory that is not inside a git repository, it should display a clear, friendly error:

```
rx: not a git repository (or any parent up to mount point /)

rx currently requires a git repo to work. Run it from inside a project
that uses git, or see https://github.com/<user>/rx for future plans.
```

Non-git diff sources (two-file comparison, directory diff) are a non-goal for v1 but may be added later. The error message should not be hostile — this is likely the user's first interaction with the tool.

---

## 6. Core Features

### 6.1 Side-by-Side Diff View

- Line-aligned, scrollable panes
- Line numbers on both sides
- Word-level inline diffs within changed lines (highlight specific changed tokens, not whole lines)
- Syntax highlighting via `syntect`
- Configurable color theme (ship with sensible defaults, allow user themes in config)
- Comment anchor markers (`●` for open, `○` for resolved) rendered inline

### 6.2 Unified Diff View

- Togglable alternative to side-by-side
- Standard `+`/`-` prefix rendering
- Same syntax highlighting, word-level diffs, and comment markers

### 6.3 File Panel (Left Sidebar)

- Lists all files that contain diffs
- Shows per-file diff stats (e.g., `+12 -3`)
- Annotated with `●` when a file has open comments
- Navigable with `j`/`k` or arrow keys
- Selecting a file loads it in the viewport
- Supports tree view (directory grouping) and flat list view
- Togglable to give viewport full width

### 6.4 Comment Panel (Right Sidebar)

- Lists all comments for the currently viewed file
- Shows comment body, line number, and status (Open / Resolved / Updated)
- Navigate between comments with shortcuts
- Selecting a comment scrolls the diff viewport to the anchored line
- Also supports a "global" view showing comments across all files
- Togglable independently

### 6.5 Comment Editor

- Lightweight floating overlay (not a modal editor)
- Triggered by pressing `c` on a diff line (inline comment) or `C` (global comment)
- Basic text editing: multiline input, minimal formatting
- **Save with `Ctrl+Enter`**, cancel with `Esc`. Plain `Enter` inserts a newline (since comments are multiline, `Enter` must not mean "submit" — this is consistent with how GitHub's comment box works)
- Editing an existing comment: navigate to it and press `c` again

### 6.6 Auto-Reload

- Uses `notify` crate to watch source files
- Debounces rapid saves (configurable, default 200ms) to avoid re-rendering mid-write
- File system events within the debounce window are **batched** — when an agent writes multiple files in quick succession, all changes are collected and processed as a single update (one diff recompute, one resolver pass across all affected comments)
- Preserve scroll position and cursor location as closely as possible
- Visual indicator when a reload occurs (status bar message)

### 6.7 Export Report

The core output artifact. Three formats:

**Default (stdout, paste-ready for AI agents):**
```
src/auth/login.rs#42: This doesn't handle the case where token is expired — add a refresh fallback
src/auth/login.rs#55: Magic number 3600, extract to a named constant
README.md: Global — update the setup section to reflect the new env var
```

**Markdown (for docs/Notion):**
```markdown
## Review Comments
### src/auth/login.rs
- **Line 42** (open): This doesn't handle the case where token is expired...
- **Line 55** (resolved): Magic number 3600...
### Global
- Update the setup section to reflect the new env var
```

**JSON (for MCP / programmatic use):**
```json
{
  "session_id": "...",
  "comments": [
    { "file": "src/auth/login.rs", "line": 42, "status": "open", "body": "..." }
  ]
}
```

**CLI usage:**
```bash
rx export                          # stdout, all open comments (default)
rx export --format markdown
rx export --format json
```

**Filtering options:**
```bash
rx export --status open            # only open comments (default)
rx export --status all             # open + resolved + updated
rx export --status resolved        # only resolved
rx export --file src/auth/login.rs # only comments for a specific file
```

The default behavior is to export only **open** comments — this is what you want when pasting feedback into an AI agent. Resolved comments are noise in that context. The `--status all` flag is available for audit/documentation purposes.

The default stdout format is the killer feature — `rx export | pbcopy` and paste into the AI agent.

---

## 7. Mode System

Vi-inspired, modal input:

| Mode | Entry | What it does |
|------|-------|-------------|
| **Normal** | default | Navigate hunks, scroll, switch panels |
| **Visual** | `v` | Select lines/hunks for copy operations |
| **Edit** | `i` | Inline edit the focused side (bonus feature, lower priority) |
| **Search** | `/` | Fuzzy search within diff |

### 7.1 Keyboard Shortcuts

Every action must be shortcut-accessible. No mouse-only interactions.

| Action | Default Key |
|--------|-------------|
| Scroll down/up | `j` / `k` |
| Page down/up | `Ctrl+d` / `Ctrl+u` |
| Next/prev hunk | `]c` / `[c` (vim-compatible) |
| Next/prev file | `]` / `[` |
| Switch sides (left/right pane) | `Tab` |
| Toggle file panel | `Space` |
| Toggle comment panel | `Ctrl+p` |
| Toggle side-by-side / unified | `t` |
| Add inline comment | `c` |
| Add global comment | `C` |
| Resolve comment | `r` |
| Export comments | `e` |
| Enter edit mode | `i` |
| Exit current mode | `Esc` |
| Copy selection to other side | `y` (in Visual mode) |
| Save edits | `:w` or `Ctrl+s` (in Edit mode) |
| Save comment | `Ctrl+Enter` (in comment editor) |
| Quit | `q` |
| Search | `/` |
| Move drifted comment anchor | `M` |
| Help | `?` |

Note: `e` is reserved for export (the primary workflow). Edit mode uses `i` (consistent with vi's insert mode convention). There is no collision because `e` in Normal mode always means export, and `i` in Normal mode always means enter Edit mode.

All shortcuts are configurable via `config.toml`.

---

## 8. Editing Features (Bonus, Lower Priority)

Editing is a differentiating feature but secondary to the review workflow. It should feel lightweight — not like opening vim.

### 8.1 Inline Edit

- Press `i` to enter edit mode on the currently focused pane
- Basic text editing (insert, delete, backspace)
- Changes are written back to the source file on save (`:w` or `Ctrl+s`)
- Visual mode indicator (status bar shows `-- EDIT --`)
- `Esc` exits edit mode (confirmation prompt if dirty)

### 8.2 Cross-Pane Copy

- Select a range of lines or a hunk on one side with Visual mode (`v`)
- Press `y` to copy that content to the corresponding position on the other side
- This effectively "accepts" a change from one direction
- After copy + save, the diff auto-recomputes via the file watcher

### 8.3 Constraints

- Editing modifies the actual file on disk (not a virtual buffer)
- Undo is per-session only (no persistent undo history)
- Keep edit mode intentionally constrained: no full editor, just hunk-level operations

---

## 9. Session Management

### 9.1 Session Identity

Key: `canonical_repo_root + base_commit_sha + branch_name`

The repo root path is **canonicalized** (symlinks resolved, `.` and `..` collapsed) before hashing to prevent duplicate sessions when the same repo is accessed via different paths (e.g., `~/projects/myapp` vs. `/home/user/projects/myapp` vs. a symlink).

```
~/.local/share/rx/sessions/
  <sha256 of canonical_repo_path + base>_<branch>.json
```

Examples:
```
/home/user/myapp + main + feature/auth  →  one session
/home/user/myapp + main + develop       →  another session
/home/user/myapp + HEAD~3 + (none)      →  point-in-time review
```

A session accumulates — bigger sessions are fine as long as resolved comments can be filtered out.

### 9.2 Session Lifecycle

#### 9.2.1 Creation

A session is created automatically on first use within a given context.

#### 9.2.2 Closure — Layered Detection

Sessions can close through four mechanisms, in priority order:

```
1. EXPLICIT  — user runs `rx close` or presses [X] in session list
               → most reliable, always supported

2. MERGED    — detectable via:
               git merge-base --is-ancestor <session_head> main
               git branch -d <branch> (branch deleted = likely merged)
               → works for normal PR workflows

3. DIVERGED  — base_ref has moved far ahead of session start
               (e.g. 50+ commits on main since session created)
               → weak signal, only prompt user, never auto-close

4. STALE     — no activity for N days (configurable, default 30)
               + all comments resolved
               → auto-archive (not delete), surface as "likely done?"
```

#### 9.2.3 The "Always on Main" Problem

If a user reviews changes on `main` (e.g., comparing `main~5..main`), the base keeps moving with every new commit. Solution:

- At session creation, record `base_ref@{timestamp}` resolved to a **concrete commit SHA**
- The session is pinned to that SHA, not to the moving ref
- "Closed" has a concrete meaning: the commits you were reviewing have landed on main

Detectable with:
```bash
git merge-base --is-ancestor <session_head_commit> main
```

If true → all reviewed code is in main → prompt to archive:

```
┌─ Session update ─────────────────────────────────────────┐
│  The code from this review (abc123f..def456) has been     │
│  merged into main. 3 comments still open.                 │
│                                                           │
│  [A] Archive session   [K] Keep open   [V] View comments  │
└───────────────────────────────────────────────────────────┘
```

---

## 10. Comment Anchoring & Line Drift

When files change (agent edits, auto-reload), comments must be re-anchored to the correct lines. This uses a tiered strategy.

### 10.1 Tier 1 — Context-Line Sliding Match (always available, ship first)

Store 5 lines of context with every comment anchor (2 before, the target line, 2 after). On reload, run a sliding window match over the new file content. Find the position where the context lines best fit (Levenshtein or simple equality).

Confidence tiers:

| Match score | Action |
|-------------|--------|
| >90% | Silent re-anchor (no indicator) |
| 60–90% | Re-anchor with `~` indicator (approximate match) |
| <60% | Mark as `⚠ drifted`, do not auto-re-anchor |

This handles the vast majority of real cases — agent edits are usually local changes, not global reshuffles.

### 10.2 Tier 2 — Tree-sitter Semantic Anchoring (add later, language-dependent)

For supported languages, also record the AST path to the node at the comment's line:

```
file: auth/login.rs
ast_path: function_item[name=authenticate] > block > expression_statement[2]
line: 42 (fallback)
```

On reload, re-parse with tree-sitter and walk the same AST path. If the node exists, anchor there. This is immune to line shifts as long as the function wasn't renamed or deleted.

Constraints:
- Only works for tree-sitter-supported languages (Rust, Python, JS/TS, Go, C/C++ as the v1 set)
- Only works for code files (not config, markdown, prose)
- Always fall back to Tier 1 context lines if tree-sitter fails

### 10.3 Tier 3 — Manual Re-anchor (always available)

When a comment shows `⚠ drifted`:

- User presses `M` to enter move mode
- Navigate with `↑`/`↓` to the correct line
- Press `Enter` to re-anchor

This is the escape hatch. If users frequently use manual re-anchor, it signals Tier 1/2 heuristics need improvement.

### 10.4 Anchor Resolution Flow

```
File changes detected
        │
        ▼
Run context-line sliding match (Tier 1)
        │
   ┌────┴──────────┬──────────────┐
   │               │              │
  >90%          60–90%          <60%
  silent        mark ~          mark ⚠ drifted
  re-anchor     re-anchor            │
                                     │
                          tree-sitter available
                          and language supported?
                                     │
                                ┌────┴────┐
                                │         │
                             AST match   no match
                             re-anchor   stay ⚠
                             clear ~     offer [M]anual
```

---

## 11. MCP Server (Phase 5)

Expose the review session as an MCP server so AI agents can interact with comments directly, without copy-paste.

### 11.1 MCP Tools

| Tool | Description |
|------|-------------|
| `list_comments` | Returns all open/resolved comments for the session |
| `reply_to_comment` | Agent posts a reply to a specific comment |
| `resolve_comment` | Agent marks a comment as resolved (after fixing) |
| `get_diff_context` | Returns the diff context around a comment |

### 11.2 Workflow

```
Human writes comments in TUI
        │
        ▼
Agent reads comments via MCP (list_comments)
        │
        ▼
Agent makes code changes
        │
        ▼
Agent replies via MCP (reply_to_comment / resolve_comment)
        │
        ▼
TUI auto-reloads, shows agent replies in comment thread
        │
        ▼
Human reviews, resolves, or adds new comments
```

This is where `rx` goes from a good review tool to a **platform** — a local async review loop between human and agent.

---

## 12. Configuration

### 12.1 Config File Location

```
~/.config/rx/config.toml
```

### 12.2 Configurable Options

```toml
[appearance]
theme = "default"              # or path to a custom theme file
side_by_side = true            # default view mode
line_numbers = true
word_diff = true               # highlight word-level changes within lines
tab_width = 4

[behavior]
auto_reload = true
reload_debounce_ms = 200       # debounce rapid file system events
session_stale_days = 30        # auto-archive sessions older than this

[keybindings]
scroll_down = "j"
scroll_up = "k"
next_hunk = "]c"
prev_hunk = "[c"
toggle_file_panel = "space"
add_comment = "c"
resolve_comment = "r"
export = "e"
enter_edit = "i"
quit = "q"
# ... all shortcuts overridable

[git]
default_base = "HEAD"          # default comparison base for git mode

[export]
default_format = "text"        # text | markdown | json
default_status = "open"        # open | resolved | updated | all
```

---

## 13. Accessibility & Terminal Compatibility

### 13.1 Color Degradation

`rx` should detect terminal color capabilities and degrade gracefully:

| Terminal capability | Behavior |
|---------------------|----------|
| 24-bit truecolor | Full theme colors, syntax highlighting |
| 256-color | Mapped palette, syntax highlighting with reduced fidelity |
| 16-color / no color | Text-only markers, bold/underline for emphasis |

Comment status markers should not rely solely on color. The `●` (open) and `○` (resolved) Unicode markers are always shown regardless of color support. On terminals that can't render Unicode, fall back to ASCII: `*` (open) and `o` (resolved).

### 13.2 Colorblind Considerations

Default theme should avoid red/green as the only differentiator for added/removed lines. Use blue/orange or adjust saturation so that the diff is readable under common forms of color vision deficiency. The user can also override all colors via theme configuration.

---

## 14. Distribution

### 14.1 Homebrew

```
brew tap <user>/rx
brew install rx
```

### 14.2 Cargo

```
cargo install rx
```

### 14.3 Pre-built Binaries

GitHub Releases with binaries for:

- macOS (Apple Silicon + Intel)
- Linux (x86_64, aarch64)
- Windows (x86_64)

CI/CD via GitHub Actions with cross-compilation.

---

## 15. Phased Build Plan

| Phase | Deliverable | Value |
|-------|-------------|-------|
| **1** | Side-by-side diff, file panel, syntax highlighting, hunk navigation, `brew` installable | Useful as a basic diff viewer |
| **2** | Inline + global comments, comment panel, session persistence | Core review workflow works |
| **3** | Export report (`filename#line:` format), resolve/update lifecycle, comment filtering | End-to-end agent review loop via copy-paste |
| **4** | Auto-reload, line drift re-anchoring (Tier 1 context lines), search | Smooth iterative review after agent edits |
| **5** | MCP server — agent reads/replies to comments directly | Full async human-agent review platform |

Editing features (inline edit, cross-pane copy) can be added in parallel at any phase but are not on the critical path.

---

## 16. Non-Goals (Explicit Exclusions for v1)

- **Not a `git diff` drop-in replacement**: No goal of being pipe-compatible with `git diff` output format
- **No non-git diff sources in v1**: Directory and two-file comparison deferred
- **No remote/networked diffs**: Local files and local git repos only
- **No merge conflict resolution UI**: Out of scope
- **No plugin system**: Configuration yes, extensibility no (for now)
- **No mouse-first design**: Mouse may work for scrolling, but the UI is keyboard-first

---

## 17. Open Questions

1. **Tree-sitter v1 scope**: Which languages to support initially? Rust, Python, JS/TS, Go, C/C++ cover the most common agent-generated code.
2. **Comment storage format**: JSON is simple but not human-editable. TOML would be more consistent with the config file. Decide before implementing sessions.
3. **Git integration depth**: How much of libgit2 (via `git2` crate) to use vs. shelling out to `git`? libgit2 is more robust but adds binary size and build complexity.
4. **Editing priority**: Should editing features ship in v1, or be deferred entirely to keep scope focused on the review workflow?
5. **MCP protocol details**: Which MCP transport to use? stdio vs. HTTP. Depends on agent integration patterns at the time of implementation.
