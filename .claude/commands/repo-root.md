---
description: Print the absolute Git repo root for the current directory (works with git worktrees).
allowed-tools: Bash(git rev-parse --show-toplevel)

---

## Context

- Repo root (if in a Git repo): !`git rev-parse --show-toplevel`

## Your task

If the “Repo root” value above exists, **output exactly that path and nothing else**.
If it’s empty or the command failed, output this line exactly:
[error] Not inside a git repository
