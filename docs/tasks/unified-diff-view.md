# Unified Diff View

## Goal

Implement a unified diff view as a togglable alternative to side-by-side, using standard `+`/`-` prefix rendering.

## Approach

- Toggle between side-by-side and unified with `t` key.
- Unified view shows a single column with `+`/`-`/` ` prefixes.
- Same syntax highlighting, word-level diffs, and comment markers.
- Same scrolling and hunk navigation.

## How to Verify

1. Press `t` — view switches from side-by-side to unified.
2. Press `t` again — switches back.
3. Added lines show with `+` prefix and green highlighting.
4. Removed lines show with `-` prefix and red highlighting.
5. Comment markers appear in the gutter.
6. All navigation (`j`/`k`, `]c`/`[c`) works in unified mode.

## Dependencies

- [side-by-side-diff-view](side-by-side-diff-view.md)
- [syntax-highlighting](syntax-highlighting.md)
