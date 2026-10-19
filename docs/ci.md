# CI runner budget

This repository uses at most four runners across all workflow runs, including
simultaneous PRs, native package tests, offline checks and documentation builds
and deployments. Every job shares one of four `repository-runner-*` concurrency
groups. Native package matrices also use `max-parallel: 4`.

Jobs wait in GitHub's concurrency queue without occupying runners. Each group
uses `queue: max`, which retains up to 100 pending jobs instead of replacing
the previous pending job. Jobs do not cancel other jobs in their group.
New commits cancel superseded PR runs of the same workflow.

Keep every new runner job in one of these four groups. A separate group would
bypass the repository budget. Workflow-level cancellation groups must remain
separate from the runner groups to avoid blocking their own jobs.

See [GitHub's concurrency documentation](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/control-workflow-concurrency).
