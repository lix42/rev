---
name: task-done
description: Mark a task as done [x] in docs/TASKS.md. Used by /ship workflow and can be invoked manually.
---

# Mark Task Done

Mark a task as done (`[x]`) in `docs/TASKS.md`.

## Arguments

- `$ARGUMENTS` — (optional) task name or keyword to match. If omitted, look for the current in-progress (`[~]`) task.

## Instructions

1. Read `docs/TASKS.md`.
2. If `$ARGUMENTS` is provided, find the task line containing that text.
   - If no match, show available in-progress (`[~]`) tasks and ask the user to pick one.
3. If `$ARGUMENTS` is omitted:
   - If exactly one task is `[~]`, use that one.
   - If multiple tasks are `[~]`, list them and ask the user which one to mark done.
   - If no tasks are `[~]`, list not-done tasks and ask.
4. Change the matched task's `[~]` (or `[ ]`) to `[x]`.
5. Confirm which task was marked done.
6. Do NOT mark a task that is already `[x]`.
