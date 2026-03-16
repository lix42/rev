# Session Staleness & Merge Detection

## Goal

Detect when sessions are stale (inactive + all resolved) or merged (branch landed on main), and prompt the user to archive.

## Approach

- **Merged detection:** On session load, run `git merge-base --is-ancestor <session_head> main`. If true, prompt to archive (design spec 9.2.2).
- **Stale detection:** If no activity for N days (configurable, default 30) and all comments resolved, show a "likely done?" prompt.
- **Diverged detection:** If base ref has moved far ahead (50+ commits), prompt the user (weak signal, never auto-close).
- Prompts offer: `[A] Archive`, `[K] Keep open`, `[V] View comments`.

## How to Verify

1. After merging a branch into main, launching `rx` detects the session as merged and shows the archive prompt.
2. A 30+ day old session with all comments resolved shows the staleness prompt.
3. User can choose to archive or keep open.
4. Neither detection auto-closes without user confirmation.

## Dependencies

- [session-persistence](session-persistence.md)
- [git-repo-detection](git-repo-detection.md)
- [comment-lifecycle](comment-lifecycle.md)
