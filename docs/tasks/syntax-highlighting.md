# Syntax Highlighting

## Goal

Apply syntax highlighting to diff content using `syntect`, so code in both old and new panes is colorized based on file type.

## Approach

- Use `syntect` to load bundled syntax definitions and a default theme.
- Detect the file's language from its extension (syntect has built-in extension mapping).
- Highlight both old and new file content independently.
- Merge syntax highlighting spans with diff coloring (additions/removals should retain syntax colors but with a tinted background).
- Implement in `src/diff/highlighter.rs`.

## Design

```rust
pub fn highlight_lines(lines: &[String], file_path: &Path) -> Vec<Vec<StyledSpan>>;

pub struct StyledSpan {
    pub text: String,
    pub fg: Color,
    pub bg: Option<Color>,  // diff background tint
}
```

The diff view then renders `StyledSpan`s instead of plain strings.

## How to Verify

1. Open a `.rs` file diff — keywords (`fn`, `let`, `struct`) are colorized.
2. Open a `.py` file diff — Python syntax is highlighted.
3. Added lines have syntax colors on a green-tinted background.
4. Removed lines have syntax colors on a red-tinted background.
5. Unknown file types render without highlighting but don't crash.

## Dependencies

- [side-by-side-diff-view](side-by-side-diff-view.md)
