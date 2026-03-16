# Color Degradation & Accessibility

## Goal

Detect terminal color capabilities and degrade gracefully. Ensure the UI is usable under color vision deficiency and limited terminal capabilities.

## Approach

- Detect terminal color support: 24-bit truecolor, 256-color, 16-color, no color.
- Degrade rendering per design spec section 13.1:
  - 24-bit: full theme colors and syntax highlighting.
  - 256-color: mapped palette, reduced fidelity syntax highlighting.
  - 16-color / no color: text-only markers, bold/underline for emphasis.
- Comment status markers never rely solely on color: `●` (open), `○` (resolved), `◐` (updated).
- On terminals that can't render Unicode, fall back to ASCII: `*` (open), `o` (resolved).
- Default theme avoids red/green as sole differentiator (design spec 13.2) — use blue/orange or saturation differences.

## How to Verify

1. In a truecolor terminal, full syntax highlighting works.
2. With `TERM=xterm-256color`, rendering degrades to 256-color palette.
3. With `NO_COLOR=1`, rendering uses text-only markers and bold/underline.
4. Diff is readable for common color vision deficiency (check with a simulator).
5. Unicode fallback: set a terminal that can't render `●` → shows `*` instead.

## Dependencies

- [syntax-highlighting](syntax-highlighting.md)
- [side-by-side-diff-view](side-by-side-diff-view.md)
