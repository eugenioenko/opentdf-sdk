# CI runner budget

This repository uses at most four runners across all workflow runs, including
simultaneous PRs, native package tests, offline checks and documentation builds
and deployments. Every job shares one of four `repository-runner-*` concurrency
groups. The groups enforce the cap directly, without a second matrix limit
that could leave free runner slots idle.

Jobs wait in GitHub's concurrency queue without occupying runners. Each group
uses `queue: max`, which retains up to 100 pending jobs instead of replacing
the previous pending job. Jobs do not cancel other jobs in their group.
New commits cancel superseded PR runs of the same workflow.

Keep every new runner job in one of these four groups. A separate group would
bypass the repository budget. Workflow-level cancellation groups must remain
separate from the runner groups to avoid blocking their own jobs.

See [GitHub's concurrency documentation](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency).

## PR failures and retries

Native package tests wait for the offline format/protocol gate. A failed
offline gate skips that matrix; retrying the gate also retries its dependents.
Within a PR matrix, one failure cancels the remaining queued and running jobs.
Manual delivery runs continue collecting results after failures. Documentation
workflows remain independent.

Retry a failed matrix job from its Actions page. After it passes, the trusted
`Resume CI after retry` workflow requests the remaining failed or cancelled
jobs, keeping successful results. It does not automatically retry a job that
is still failing. It ignores cancelled retry attempts and checks that the PR
is still open at the same commit and attempt before resuming. Source fixes
need a new commit; job retries use the original commit. Automatic continuation
requires the resume workflow to be merged into the default branch.
