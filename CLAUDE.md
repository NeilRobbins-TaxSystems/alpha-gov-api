# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**alpha-gov-api** is a Rust CLI tool that exposes UK government APIs (Companies House, HMRC, Trade/Tariff, DEFRA, GOV.UK platform) as structured JSON interfaces for agentic AI consumption. It lives in the TaxSystems GitHub organisation.

The Rust project has not yet been scaffolded. Phase 0 (project foundation) is the first implementation milestone. See `docs/plan.md` for the full phased plan and GitHub issues for tracked work items.

## Build & Test (once scaffolded)

```bash
cargo build                    # build
cargo test                     # all tests
cargo test <test_name>         # single test
cargo clippy                   # lint
cargo fmt --check              # format check
```

## Architecture (planned)

**Binary:** `alpha-gov-api` — single CLI binary using `clap` (derive macros) for argument parsing.

**Command hierarchy:** Top-level subcommands group by provider, then resource:
- `ch` — Companies House (`ch company get`, `ch officers list`, `ch stream filings`, `ch file submit`, `ch xmlgw submit`)
- `hmrc` — HMRC (`hmrc vat obligations`, `hmrc customs declaration submit`, `hmrc gvms gmr create`)
- `trade` — Trade/Tariff (`trade tariff lookup`, `trade fpo classify`, `trade barriers search`)
- `govuk` — GOV.UK Platform (`govuk pay create`, `govuk notify email`)
- `defra` — DEFRA (`defra ehc apply`)

**Output contract:** Every command writes JSON to stdout using one of two envelopes:
```
Success: { "ok": true,  "data": { ... }, "meta": { "api", "endpoint", "timestamp", "rate_limit_remaining" } }
Error:   { "ok": false, "error": { "code", "message", "api_status", "api" } }
```

**Global flags:** `--output json|pretty|compact`, `--quiet`, `--config <path>`, `--profile <name>`, `--sandbox`, `--dry-run`

**Key crates (planned):** `clap` (CLI), `reqwest` (HTTP), `serde`/`serde_json` (serialisation), `tokio` (async runtime), `tracing` (logging), `wiremock` (test mocks)

## Configuration

Config file: `~/.config/alpha-gov-api/config.toml` (or platform equivalent)

Environment variable prefix: `ALPHA_GOV_API_` (e.g., `ALPHA_GOV_API_CH_KEY`, `ALPHA_GOV_API_HMRC_CLIENT_ID`)

## Authentication patterns

Three auth mechanisms across the APIs:
1. **API key** (Companies House) — sent as HTTP Basic username, empty password
2. **OAuth 2.0 client credentials** (HMRC application-restricted) — token endpoint at `https://api.service.hmrc.gov.uk/oauth/token`
3. **OAuth 2.0 authorization code** (HMRC user-restricted) — authorize at `https://api.service.hmrc.gov.uk/oauth/authorize`

Sandbox endpoints use `test-api.service.hmrc.gov.uk` (HMRC) and `api-sandbox.company-information.service.gov.uk` (CH).

## API documentation

- `docs/uk-government-apis.md` — index of all ~40 APIs with utility summaries and links to detail files
- `docs/apis/01-12-*.md` — per-category detail files (type, auth, endpoints, resources)
- `docs/plan.md` — phased integration plan with CLI command specifications

Read the index file to identify relevant APIs. Read the detail file only for the specific category you need — avoid loading all 12 into context.

## GitHub structure

- **Milestones:** Phase 0 through Phase 9 (10 milestones)
- **Issues:** 44 epic issues, each with task checklists — numbered to match plan sections (e.g., issue "1.1 Company profile commands" maps to plan section 1.1)
- **Labels:** `phase-0` through `phase-9`, `epic`, `enhancement`, `infrastructure`, `testing`, `documentation`
