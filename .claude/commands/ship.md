# Ship

End-to-end shipping workflow for completing a feature. Run this when you're done with a feature and ready to create a PR.

This is a sequential workflow — complete each step before moving to the next. Stop and involve the user whenever a step needs human judgment.

## Step 1: Rebase onto origin/main

There are likely uncommitted changes at this point. Stash them first, rebase, then restore.

```bash
git stash push -m "ship: stash before rebase"
git fetch origin
git rebase origin/main
git stash pop
```

- If rebase succeeds cleanly and stash pops without conflicts: continue to Step 2.
- If rebase has conflicts: stop immediately. Show the user which files conflict and work with them to resolve each one. After resolution, run `git rebase --continue`, then `git stash pop`. If the conflicts are too complex or the user wants to abort, run `git rebase --abort` and `git stash pop`, then stop the workflow.
- If stash pop has conflicts after a successful rebase: show the user which files conflict and work with them to resolve. The stash ref remains available via `git stash` until conflicts are resolved.

## Step 2: Review the PR

Invoke `/pr-review-toolkit:review-pr` to run a comprehensive code review.

- Present the review results to the user.
- If there are **critical issues**: stop and fix them before continuing. Re-run the review after fixes if needed.
- If there are only suggestions or minor issues: note them but continue — the user can decide whether to address them.

## Step 3: Update CLAUDE.md

Invoke `/claude-md-management:revise-claude-md` to capture any learnings from this session into CLAUDE.md.

- Present the proposed changes to the user for approval before writing.

## Step 4: Update task status

If this feature corresponds to a task in `docs/TASKS.md`, mark it as done (`[x]`).

## Step 5: Commit, push, and create PR

Invoke `/commit-commands:commit-push-pr` to create commits, push to remote, and open a pull request.

- Make sure all changes (including review fixes, CLAUDE.md updates, and task status updates from previous steps) are included.

## Step 6: Monitor the PR

After the PR is created, actively monitor it:

### CI checks (lint/test/build)
- Run `gh pr checks <pr-number> --watch` to monitor CI status.
- If any checks fail: read the failure logs with `gh run view <run-id> --log-failed`, diagnose the issue, fix it, commit, and push. Then resume monitoring.

### Review comments
- Check for comments with `gh api repos/{owner}/{repo}/pulls/{pr-number}/reviews` and `gh api repos/{owner}/{repo}/pulls/{pr-number}/comments`.
- For each comment, assess it:
  - **Not a concern** (style nit, misunderstanding, or disagreement): reply explaining why and resolve the comment.
  - **Straightforward fix** (typo, naming, small logic change): fix it, push the update, reply noting the fix, and resolve.
  - **Complex or debatable issue**: present the comment to the user with context and your analysis. Discuss before taking action.

### When to stop monitoring
- All CI checks pass AND no unresolved comments: tell the user the PR is ready for merge.
- If the PR has been idle with no new activity after all checks pass, let the user know.
