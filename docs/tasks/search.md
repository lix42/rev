# Search

## Goal

Implement `/` search mode for fuzzy-finding text within the current diff view (design spec section 7).

## Approach

- `/` enters Search mode — a full modal state (Normal → Search transition).
- Type a query → matching lines are highlighted in the diff view.
- `n` / `N` to jump to next/previous match.
- `Esc` exits search mode, clears highlights.
- Search across both old and new sides of the diff.
- Scope: current file's diff only (cross-file search is a future enhancement).

## How to Verify

1. Press `/`, type a query — matching text is highlighted.
2. `n` jumps to the next match.
3. `N` jumps to the previous match.
4. `Esc` clears the search.
5. Search works across hunk boundaries.

## Dependencies

- [side-by-side-diff-view](side-by-side-diff-view.md)
- [input-system](input-system.md)
