# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

**alpha-gov-api** is a Rust CLI tool that exposes UK government APIs (Companies House, HMRC, Trade/Tariff, DEFRA, GOV.UK platform) as structured JSON interfaces for agentic AI consumption. It lives in the TaxSystems GitHub organisation.

Rust edition 2024, workspace resolver 3. See `docs/plan.md` for the full phased plan and GitHub issues for tracked work items.

## Build & Test

```bash
rustup run stable cargo build          # build
rustup run stable cargo test           # all tests
rustup run stable cargo test <name>    # single test
rustup run stable cargo clippy         # lint
rustup run stable cargo fmt --check    # format check
```

**Note:** Use `rustup run stable` prefix — the bash shell proxy may resolve an older toolchain version on Windows. Edition 2024 requires Rust 1.85+.

## Architecture

**Workspace layout:** `crates/alpha-gov-api` (binary) and `crates/alpha-gov-api-core` (library).

**Binary crate** (`alpha-gov-api`) — CLI using `clap` derive macros. Global flags: `--pretty`, `--quiet`, `--config`, `--profile`, `--sandbox`, `--dry-run`.

**Core crate** (`alpha-gov-api-core`) — output contract (`ApiResponse<T>`, `ApiErrorResponse`), config/credential management, error types.

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

Config file: platform-appropriate path via `dirs` crate (`%APPDATA%` on Windows, `~/.config` on Linux, `~/Library/Application Support` on macOS). TOML format with `[defaults]`, `[profile.*]`, and `[credentials]` sections.

Environment variable prefix: `ALPHA_GOV_API_` (e.g., `ALPHA_GOV_API_CH_KEY`, `ALPHA_GOV_API_HMRC_CLIENT_ID`)

**Credential resolution order:** env vars > OS keychain (`keyring` crate) > plaintext TOML `[credentials]` section.

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

**Repositories:**
- **Origin:** `TaxSystems/alpha-gov-api` — master branch is protected (requires approving review to push)
- **Working copy:** `NeilRobbins-TaxSystems/alpha-gov-api` — no branch protection, use for direct pushes
- Git remote `personal` points to the working copy repo

- **Milestones:** Phase 0 through Phase 9 (10 milestones)
- **Issues:** 44 epic issues, each with task checklists — numbered to match plan sections (e.g., issue "1.1 Company profile commands" maps to plan section 1.1)
- **Labels:** `phase-0` through `phase-9`, `epic`, `enhancement`, `infrastructure`, `testing`, `documentation`
