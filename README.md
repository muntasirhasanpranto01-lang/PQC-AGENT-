# QRYPTARA

An autonomous post-quantum cryptography (PQC) security platform written in Rust.

This repository provides a practical MVP foundation for the QRYPTARA concept: a Rust workspace, API server, cryptographic inventory, policy logic, PQC recommendation engine, and structured security audit layer.

## Repository goals

- Discover and classify cryptographic assets.
- Evaluate migration readiness for RSA/ECC ecosystems.
- Recommend quantum-safe algorithm pathways.
- Enforce policy and approval gates before high-risk actions.
- Provide a clean Rust workspace for continued product development.

## Included MVP pieces

- `apps/qryptara-api`: REST API for discovery and policy-guided actions.
- `apps/qryptara-agent`: example agent runtime.
- `crates/agent-core`: agent action model and tool execution interface.
- `crates/crypto-discovery`: inventory classification for cryptographic assets.
- `crates/pqc-engine`: PQC recommendation and planning logic.
- `crates/policy-engine`: approval and policy evaluation.
- `crates/audit-engine`: tamper-aware event logging scaffolding.
- `docs/MVP-REPOSITORY-SPEC.md`: implementation plan and project charter.

## Quick start

```bash
cargo run -p qryptara-api
```

## Notes

This is a working MVP scaffold, not a production cryptographic scanner or production PQC implementation. It is intended to be the foundation for the full QRYPTARA security platform described in the product concept.
