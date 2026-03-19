# Clean Explain Files

Remove all `*.explain.md` files from a target folder.

## Argument

This command takes one optional argument: the module or folder path to clean (e.g., `src/git`, `src/review`, or `.` for the whole project).

If no argument is provided: ask the user "Which module or folder should I clean explain files from? (e.g., `src/git`, `src/review`, or `.` for the whole project)"

## Steps

1. Parse the argument. If the user didn't provide one, ask them using AskUserQuestion.

2. Find all `*.explain.md` files in the target folder recursively using the Glob tool with pattern `<folder>/**/*.explain.md`.

3. If no files found, tell the user: "No *.explain.md files found in `<folder>`."

4. If files found, delete each one using `rm` via the Bash tool. Show the user what was removed:
   ```
   Removed 3 explain files from src/git/:
   - src/git/mod.explain.md
   - src/git/repo.explain.md
   - src/git/diff.explain.md
   ```
