---
description: Execute feature implementation tasks from specification and track progress.
argument-hint: [tasks-file-path]
allowed-tools: Bash, View, AddTodos, UpdateTodoStatus, AttemptCompletion
---

Given the tasks execution instruction provided as an argument, do this:

1. Read `$ARGUMENTS` as the tasks execution instruction ready to be executed.
2. Load execution context:
   - IF EXISTS: Read `exec_context.md` from the same directory as `$ARGUMENTS`.
   - IF NOT EXISTS: Create `exec_context.md` in the same directory as `$ARGUMENTS` to record task execution status.
3. Run through the tasks from `$ARGUMENTS` and keep it up-to-date with the current state in `exec_context.md` as it is being worked on, so that work can be paused and resumed later.
4. Report completion status and any remaining tasks or blockers.

Note: The execution context file tracks progress across sessions, allowing for incremental implementation and recovery from interruptions.
