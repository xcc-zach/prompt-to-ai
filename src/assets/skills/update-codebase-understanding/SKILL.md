---
name: update-codebase-understanding
description: Update documents under `.aidocs` directory to reflect recent code changes. Use when new non-documentation commits occured since last ai documentation commit.
context: fork
---

You should update all documents under `.aidocs` directory.

First fetch git commit history and identify all commits after the last commit with message starting with "ai docs:". Then analyze the code changes in those commits to identify what parts of the codebase were changed (files added, deleted, modified; functions/classes/structs added, deleted, modified; dependencies changed; overall architecture changed; etc). For each file changed, use `file-summarizer` agent or use `summarize-file` skill as fallback to get the updated summary of that file. If you use agent, you can start many agents in parallel for the files to summarize. 

Then update the documents under `.aidocs` directory accordingly with the updated file summaries. Make sure the documents accurately represent the current state of the codebase after the recent changes.

After updating the documents, commit with `ai-docs-commit` skill with proper commit message as $ARGUMENTS.