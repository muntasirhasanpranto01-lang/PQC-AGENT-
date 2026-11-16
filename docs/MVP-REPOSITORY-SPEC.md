# QRYPTARA MVP Repository Specification

## 1. Purpose

This repository establishes the first implementation milestone for the QRYPTARA concept: a Rust-based MVP that can discover crypto assets, reason about PQC risk, and enforce policy-controlled actions.

## 2. Product direction

The project is intentionally scoped to a useful first release rather than attempting to build all twelve AI agents at once.

The initial MVP includes:

- Rust API server
- Cryptographic asset inventory model
- AI tool execution interface
- PQC recommendation engine
- Security policy gate
- Audit logging scaffolding

## 3. Workspace structure

```text
qryptara/
├── Cargo.toml
├── README.md
├── apps/
│   ├── qryptara-api/
│   └── qryptara-agent/
├── crates/
│   ├── agent-core/
│   ├── crypto-discovery/
│   ├── pqc-engine/
│   ├── policy-engine/
│   └── audit-engine/
├── docs/
│   └── MVP-REPOSITORY-SPEC.md
└── .gitignore
```

## 4. MVP components

### 4.1 qryptara-api

Responsibilities:

- Expose health and analysis endpoints.
- Accept discovery and planning tasks.
- Call the agent execution layer with policy-aware validation.

### 4.2 qryptara-agent

Responsibilities:

- Mock an operational agent runtime.
- Use the asset inventory and recommendation engine to make decisions.
- Interact with the policy layer before action execution.

### 4.3 agent-core

Responsibilities:

- Define `AgentAction` and `AgentRequest`.
- Provide typed operation routing.
- Keep AI actions constrained to allowlisted operations.

### 4.4 crypto-discovery

Responsibilities:

- Represent crypto assets and algorithms.
- Classify RSA/ECC systems as requiring PQC assessment.
- Flag ML-KEM and ML-DSA as quantum-safe algorithm families.

### 4.5 pqc-engine

Responsibilities:

- Recommend likely migration targets.
- Support TLS, SSH, and general cryptographic policy workflows.
- Keep logic deterministic and explainable.

### 4.6 policy-engine

Responsibilities:

- Evaluate whether a request is authorized.
- Force explicit approval before high-risk operations.
- Build a clear boundary between AI reasoning and execution.

### 4.7 audit-engine

Responsibilities:

- Store tamper-aware event records.
- Make operational actions visible and reviewable.
- Enable future integration with PostgreSQL or immutable logs.

## 5. API endpoints

### `GET /health`
Returns service status.

### `POST /scan`
Returns a structured, minimal cryptographic scan result.

### `POST /agent/execute`
Accepts a typed agent request and returns a status string.

## 6. Security principles

- Do not allow arbitrary shell execution from the AI layer.
- Require human approval for production-impacting actions.
- Use typed tool definitions and allowlists.
- Keep sensitive secrets out of the repository and out of AI prompts.
- Separate decision-making logic from cryptographic execution.

## 7. Next implementation steps

1. Add PostgreSQL-based asset and risk storage.
2. Expand scanner modules for TLS/SSH and certificate metadata.
3. Add real crypto policy evaluation and risk scoring.
4. Introduce migration-plan generation and validation flows.
5. Add CI, security tests, and integration coverage.

## 8. Recommended future roadmap

- Phase 1: Inventory + risk + recommendations
- Phase 2: Migration planner + validation engine
- Phase 3: PKI/HSM and enterprise monitoring
- Phase 4: Multi-agent orchestration and government deployment support

This repository is intentionally purpose-built as a realistic starting point for the QRYPTARA product concept and is aligned with the MVP strategy described in the broader product proposal.
