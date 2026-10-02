# SDK repository instructions

## Scope

Implement interoperable TDF3 encryption/decryption SDKs here. Broader OpenTDF Go SDK feature/API parity is outside the current goal; follow docs/delivery-checklist.md. Shared source is Go accepted by the adjacent `../goalchemy/` compiler. All seven targets are required: Go, TypeScript, Python, Java, C#, Rust, and C. TypeScript must support both Node and browsers.

Use `../platform/sdk/` and `../web-sdk/lib/` as pinned references, with `../platform/otdfctl/` and `../web-sdk/cli/` for operational tests. The adjacent Goalchemy clone can be modified for reusable compiler, runtime, and capability work. Follow its own applicable instructions, and those of any other repository changed.

## Coordination

The root agent coordinates one worker at a time. Workers must not spawn agents. Follow `docs/plan.md`; report changed files, verification commands and outcomes, and remaining gaps to the root agent. Do not mark work complete merely because a self-round-trip works or required tests were skipped.

Use GPT-6.1 Sol High for crypto, protocol, compiler, scheduler, and exported API work; use Medium for bounded setup, packaging, documentation, and straightforward adapters. Explicit worker tasks define file ownership. The orchestrator owns the plan, progress log, acceptance review, and phase commits.

## Git and verification

The user requested a commit after each accepted phase. Use Conventional Commit messages and the existing Git identity/signing configuration. Commit relevant changes in Goalchemy separately when a phase changes it; record corresponding commits in the SDK progress log. Do not push or publish without a release instruction. Preserve unrelated working tree changes.

Run checks appropriate to the changes and all required checks in each modified repository. Documentation-only work requires source/link verification and a whitespace check. Format/protocol/crypto changes require meaningful tests and real KAS interoperability before their phase is accepted.

Keep generated output, development secrets and keys, local profiles, logs, dependency installations, and test output under ignored storage such as `.local/` and `out/`. Local platform operations must target this project's services and preserve unrelated Docker projects.
