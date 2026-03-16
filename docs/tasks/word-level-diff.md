# Word-Level Diff

## Goal

Within changed lines, highlight the specific words/tokens that changed rather than the entire line.

## Approach

- For each pair of corresponding old/new lines in a hunk, run `similar`'s word-level or character-level diff.
- Mark changed spans within the line with a stronger highlight (e.g., bold or brighter background).
- Implement in `src/diff/word_diff.rs`.
- Integrate with the syntax highlighter — word-level diff overlays on top of syntax colors.

## Design

```rust
pub struct InlineChange {
    pub range: Range<usize>,  // byte range within the line
    pub kind: ChangeKind,     // Added, Removed, Equal
}

pub fn compute_word_diff(old_line: &str, new_line: &str) -> (Vec<InlineChange>, Vec<InlineChange>);
```

## How to Verify

1. A line where only one variable name changed — only that name is strongly highlighted, the rest of the line has normal diff background.
2. A line where a function argument was added — only the new argument is highlighted.
3. Lines with no corresponding pair (pure add/remove) show the whole line highlighted as before.

## Dependencies

- [side-by-side-diff-view](side-by-side-diff-view.md)
