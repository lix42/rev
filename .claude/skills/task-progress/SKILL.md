---
name: task-progress
description: Mark a task as in-progress [~] in docs/TASKS.md. Used by SessionStart hook and can be invoked manually.
---

# Mark Task In-Progress

Mark a task as in-progress (`[~]`) in `docs/TASKS.md`.

## Arguments

- `$ARGUMENTS` — (optional) task name or keyword to match. If omitted, present the list of not-started tasks and ask the user which one.

## Instructions

1. Read `docs/TASKS.md`.
2. If `$ARGUMENTS` is provided, find the task line containing that text.
   - If no match, show available not-started (`[ ]`) tasks and ask the user to pick one.
3. If `$ARGUMENTS` is omitted, list all not-started (`[ ]`) tasks and ask the user which one they're starting.
4. Change the matched task's `[ ]` to `[~]`.
5. Confirm which task was marked in-progress.
6. Do NOT mark a task that is already `[~]` or `[x]`.
