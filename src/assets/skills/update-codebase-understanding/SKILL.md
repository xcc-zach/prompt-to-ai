---
name: update-codebase-understanding
description: Update documents created via read-through-codebase skill to reflect recent code changes. Use when new non-documentation commits occured since last ai documentation commit.
context: fork
---

You should update all documents under `docs` directory that were created via `read-through-codebase` skills.
First fetch git commit history and identify all commits after the last commit with message starting with "ai docs:". Then analyze the code changes in those commits to identify what parts of the codebase were changed (files added, deleted, modified; functions/classes/structs added, deleted, modified; dependencies changed; overall architecture changed; etc). For each file changed, use `summarize-file` skill to get the updated summary of that file. Then update the documents under `docs/` directory accordingly with the updated file summaries. Make sure the documents accurately represent the current state of the codebase after the recent changes.

After updating the documents, commit with `ai-docs-commit` skill with arguments "update docs after code changes".