# Line Drift Resolver (Tier 1)

## Goal

When files change and diffs are recomputed, re-anchor comments to their correct lines using context-line sliding match.

## Approach

- Each comment stores a 5-line context window (2 before, target line, 2 after) in its `Anchor`.
- On file change, run a sliding window match over the new file content.
- Score matches using string equality (exact match percentage of context lines).
- Apply confidence tiers from design spec section 10.1:
  - \>90%: silent re-anchor
  - 60–90%: re-anchor with `~` indicator
  - <60%: mark as `⚠ drifted`, don't auto-re-anchor

## Design

```rust
// In src/review/resolver.rs
pub struct ResolveResult {
    pub new_line: usize,
    pub confidence: f64,
    pub status: AnchorStatus, // Exact, Approximate, Drifted
}

pub fn resolve_anchor(anchor: &Anchor, new_file_content: &[String]) -> ResolveResult;
pub fn resolve_all_comments(session: &mut ReviewSession, changed_files: &HashMap<PathBuf, Vec<String>>) -> Vec<(Uuid, ResolveResult)>;
```

## How to Verify

1. Add a comment, then insert lines above it externally — comment re-anchors silently.
2. Modify lines near the comment — comment re-anchors with `~` indicator.
3. Delete the commented line entirely — comment shows `⚠ drifted`.
4. Unit tests with synthetic file changes covering all three confidence tiers.

## Dependencies

- [comment-crud](comment-crud.md)
- [file-watcher](file-watcher.md)
