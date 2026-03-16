# Comment Lifecycle

## Goal

Implement the full comment status lifecycle: Open → Resolved, and Open → Updated (re-opened with new text).

## Approach

- `r` on an open comment → marks as Resolved, updates `updated_at`.
- Editing a resolved comment → changes status to Updated (re-opened with new text, per design spec 4.4).

> **Design decision needed:** The design spec section 4.4 shows a one-way lifecycle (Open → Resolved, Open → Updated) with no arrow from Resolved back to Open. Decide during implementation whether `r` should toggle (Resolved → Open) for convenience, or whether resolved comments can only be re-opened by editing them (which sets status to Updated). The toggle is more ergonomic; the one-way model is more intentional. Document the decision.
- Export filtering respects all three statuses.
- Visual indicators in comment panel and inline markers:
  - `●` = Open
  - `○` = Resolved
  - `◐` = Updated

## How to Verify

1. New comment starts as Open (`●`).
2. Press `r` → becomes Resolved (`○`).
3. Edit a resolved comment → becomes Updated (`◐`).
5. `rx export --status open` excludes resolved comments.
6. `rx export --status all` includes all.

## Dependencies

- [comment-crud](comment-crud.md)
