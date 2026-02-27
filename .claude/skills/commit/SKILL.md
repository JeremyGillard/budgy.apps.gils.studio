---
name: commit
description: Commit only the latest changes from the current working session
disable-model-invocation: true
allowed-tools: ["Bash", "Read", "Glob", "Grep"]
argument-hint: "[optional commit message]"
---

Commit only the changes from the latest work. Follow this process:

1. Run `git status` to see all modified and untracked files.
2. Run `git diff` (staged and unstaged) to review what changed.
3. Run `git log --oneline -5` to see recent commit message style.
4. Identify only the files that were changed in the current session — do **not** stage unrelated files.
5. Stage the relevant files by name (never use `git add -A` or `git add .`).
6. Write a commit message following [Conventional Commits](https://www.conventionalcommits.org/) (`feat:`, `fix:`, `test:`, `refactor:`, `docs:`, `chore:`, etc.).
   - If the user provided a message via `$ARGUMENTS`, use that as the commit message body.
   - Otherwise, draft a concise message based on the changes.
7. Create the commit. Always append the co-author trailer:
   ```
   Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>
   ```
8. Run `git status` after the commit to confirm success.

**Rules:**
- Never push to remote unless explicitly asked.
- Never use `--amend` unless explicitly asked.
- Never use `--no-verify` unless explicitly asked.
- Do not commit files that likely contain secrets (`.env`, credentials, keys).
