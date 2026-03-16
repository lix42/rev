# Mode Selector UI

## Goal

On launch, display a mode selector screen that lets the user choose which diff comparison to view, instead of requiring CLI flags.

## Approach

- Show the mode selector from design spec section 5.3.
- Display repo name, current branch.
- List comparison options: unstaged changes, staged changes, all local changes, compare to branch, compare to commit.
- Checkbox for "include untracked files".
- Navigate with `j`/`k`, select with `Enter`.
- Option to resume an existing session (`r`).
- After selection, transition to the main three-panel view.

## How to Verify

1. `cargo run` inside a git repo shows the mode selector.
2. All diff mode options are listed.
3. `j`/`k` navigates the list, `Enter` selects.
4. After selection, the app transitions to the diff view with correct data.
5. `r` shows existing sessions (or a message if none exist).

## Dependencies

- [tui-app-skeleton](tui-app-skeleton.md)
- [git-repo-detection](git-repo-detection.md)
