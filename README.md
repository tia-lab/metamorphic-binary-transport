# Metamorphic Binary Transport

Status: initial repository shell.

This repository will contain the production redesign of Metamorphic Binary
Transport, abbreviated MBT.

MBT is intended to be a schema-driven binary transport layer:

```text
.proto + MBT options
  -> generated schema binding
  -> checked MBT bytes
  -> trusted access after validation boundary
  -> optional adapter at consumer boundary
```

## Governance

Before changing code or dependencies, read:

- `AGENTS.md`
- `docs/invariants/core_invariants.md`
- `docs/protocols/lifecycle_protocol.md`

Task-specific protocols live under:

- `docs/protocols/`

Architecture guardrails live under:

- `docs/architecture/`

Specs and reviews will live under:

- `docs/specs/`
- `docs/reviews/`

Evidence artifacts will live under:

- `docs/evidence/`

## Current Boundary

This repository is not yet the production MBT implementation. The first
checked-in layer is the governance and protocol layer used to prevent the new
implementation from repeating the compile-surface and ownership issues found in
the experiment crate.
