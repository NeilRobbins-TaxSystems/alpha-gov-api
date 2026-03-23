# alpha-gov-api — Phased Integration Plan

## Overview

This plan defines the phased delivery of government API integrations, ordered by value for company data enrichment. Each phase is self-contained and delivers usable functionality. Phases map to GitHub milestones; individual work items map to GitHub issues.

---

## Phase 0 — Project Foundation

**Goal:** Scaffold the Rust project, establish CLI architecture, config management, and shared HTTP/auth infrastructure.

### 0.1 Project scaffold
- Initialise Cargo workspace
- Set up binary crate (`alpha-gov-api`) and library crate(s)
- Choose and configure CLI framework (`clap` with derive)
- Establish JSON output contract (success envelope, error envelope)
- Add `serde`/`serde_json` for serialisation

### 0.2 Configuration & credentials
- Config file support (`~/.config/alpha-gov-api/config.toml` or platform equivalent)
- Environment variable overrides (`ALPHA_GOV_API_CH_KEY`, `ALPHA_GOV_API_HMRC_CLIENT_ID`, etc.)
- Credential store for API keys, OAuth tokens, presenter IDs
- `config show` / `config set` commands

### 0.3 Shared HTTP client
- Async HTTP client (`reqwest`) with retry, rate-limiting, and timeout policies
- Structured error types (API errors, network errors, auth errors)
- Response caching layer (ETag/Last-Modified support)
- Logging/tracing (`tracing` crate) with JSON output option

### 0.4 Auth infrastructure
- API key auth (Companies House)
- OAuth 2.0 client credentials flow (HMRC application-restricted)
- OAuth 2.0 authorization code flow (HMRC user-restricted)
- Government Gateway credential handling (for XML APIs, later phases)

### 0.5 Testing infrastructure
- Unit test harness
- Integration test framework with mock server (`wiremock`)
- CI pipeline (GitHub Actions)

---

## Phase 1 — Companies House Public Data

**Goal:** Full read access to Companies House company data. The foundation every other phase builds on.

**Milestone:** `phase-1-ch-public-data`

### 1.1 Company profile
- `ch company get <number>` — full company profile
- `ch company search <query>` — company name/number search
- `ch company search-advanced <query>` — advanced search with filters
- `ch company registered-office <number>` — registered office address

### 1.2 Officers
- `ch officers list <number>` — list company officers
- `ch officers get <number> <appointment-id>` — individual appointment
- `ch officers search <query>` — search officers across all companies

### 1.3 Persons with Significant Control
- `ch psc list <number>` — list PSCs for a company
- `ch psc get <number> <psc-id>` — individual PSC details
- `ch psc statements <number>` — PSC statements

### 1.4 Filing history
- `ch filings list <number>` — filing history
- `ch filings get <number> <transaction-id>` — individual filing details

### 1.5 Charges
- `ch charges list <number>` — list charges (secured lending)
- `ch charges get <number> <charge-id>` — individual charge details

### 1.6 Insolvency
- `ch insolvency get <number>` — insolvency cases and status

### 1.7 Other resources
- `ch registers get <number>` — company registers
- `ch exemptions get <number>` — exemptions
- `ch disqualifications search <query>` — disqualified officers
- `ch disqualifications get <officer-id>` — individual disqualification

### 1.8 Bulk data
- `ch bulk download` — download and extract Free Company Data Product CSV
- `ch bulk import <file>` — import CSV into local store (optional)

### 1.9 Documents
- `ch documents metadata <document-id>` — document metadata
- `ch documents download <document-id> [--output <path>]` — download filing document (PDF/iXBRL)

---

## Phase 2 — Companies House Real-Time Streaming

**Goal:** Real-time change monitoring for all Companies House data types.

**Milestone:** `phase-2-ch-streaming`

### 2.1 Stream client
- `ch stream companies [--from <timepoint>]` — company information changes
- `ch stream filings [--from <timepoint>]` — filing history changes
- `ch stream officers [--from <timepoint>]` — officer changes
- `ch stream psc [--from <timepoint>]` — PSC changes
- `ch stream charges [--from <timepoint>]` — charge changes
- `ch stream insolvency [--from <timepoint>]` — insolvency changes
- `ch stream disqualifications [--from <timepoint>]` — disqualified officers
- `ch stream exemptions [--from <timepoint>]` — exemption changes
- `ch stream psc-statements [--from <timepoint>]` — PSC statement changes

### 2.2 Stream features
- Reconnection with resume from last-seen timepoint
- Output as newline-delimited JSON (NDJSON)
- Optional filtering by company number or data type

---

## Phase 3 — Trade Identity & Reference Data

**Goal:** Enrich company profiles with trade identity and reference data. No company consent required.

**Milestone:** `phase-3-trade-reference`

### 3.1 EORI validation
- `hmrc eori check <number>` — validate GB EORI, return business details

### 3.2 Trade Tariff
- `trade tariff lookup <commodity-code>` — commodity details, duty rates, controls
- `trade tariff search <query>` — search commodity codes by description
- `trade tariff measures <commodity-code>` — applicable measures at declarable level

### 3.3 UK Global Tariff
- `trade global-tariff measures <commodity-code>` — preferential and MFN measures
- `trade global-tariff declarable <commodity-code>` — 10-digit declarable measures

### 3.4 Exchange rates
- `hmrc exchange-rates [--month <YYYY-MM>] [--currency <code>]` — HMRC exchange rates

### 3.5 Trade data
- `trade data datasets` — list available DBT datasets
- `trade data query <dataset> [--filter <expr>]` — query trade datasets
- `trade barriers search [--country <code>] [--sector <code>]` — trade barriers

### 3.6 FPO classification
- `trade fpo classify <description>` — classify goods from product description

---

## Phase 4 — HMRC Agent Authorisation & VAT

**Goal:** Establish agent authority and access VAT compliance data for consenting companies.

**Milestone:** `phase-4-hmrc-vat`

### 4.1 Agent authorisation
- `hmrc agent authorise <service> <client-id>` — create authorisation request
- `hmrc agent status <invitation-id>` — check authorisation status
- `hmrc agent cancel <invitation-id>` — cancel request
- `hmrc agent relationships list` — list active relationships

### 4.2 VAT (MTD)
- `hmrc vat obligations <vrn> [--from <date>] [--to <date>]` — VAT obligations
- `hmrc vat return get <vrn> <period-key>` — view submitted return
- `hmrc vat liabilities <vrn> [--from <date>] [--to <date>]` — VAT liabilities
- `hmrc vat payments <vrn> [--from <date>] [--to <date>]` — VAT payments

### 4.3 Obligations
- `hmrc obligations <nino-or-vrn> [--type <type>]` — business filing obligations

### 4.4 Business details
- `hmrc business list <nino>` — list businesses
- `hmrc business get <nino> <business-id>` — business details
- `hmrc business income-summary <nino> <tax-year>` — BISS
- `hmrc business adjustable-summary <calc-id>` — BSAS

---

## Phase 5 — HMRC Payments & Notifications

**Goal:** Enable payment initiation and event-driven notifications.

**Milestone:** `phase-5-hmrc-payments`

### 5.1 Initiate payment
- `hmrc pay initiate <regime> <reference> <amount>` — start payment journey
- `hmrc pay status <journey-id>` — check payment status

### 5.2 Push/pull notifications
- `hmrc notifications pull <box-id>` — pull pending notifications
- `hmrc notifications acknowledge <notification-id>` — acknowledge notification

---

## Phase 6 — Customs & Goods Movement

**Goal:** Customs declaration and goods movement data for trade-active companies.

**Milestone:** `phase-6-customs`

### 6.1 Trader Goods Profile
- `hmrc tgp list <eori>` — list goods profile records
- `hmrc tgp get <eori> <record-id>` — get profile record

### 6.2 Customs declarations
- `hmrc customs declaration submit <file>` — submit declaration (XML)
- `hmrc customs declaration status <mrn>` — declaration status
- `hmrc customs declaration cancel <mrn>` — cancel declaration

### 6.3 Goods Vehicle Movements
- `hmrc gvms gmr create <json>` — create GMR
- `hmrc gvms gmr get <gmr-id>` — get GMR
- `hmrc gvms gmr list` — list active GMRs
- `hmrc gvms gmr update <gmr-id> <json>` — update GMR

### 6.4 Safety & security imports
- `hmrc safety-security ens submit <xml>` — submit ENS
- `hmrc safety-security notifications pull` — pull ENS notifications
- `hmrc safety-security outcomes get <mrn>` — get ENS outcome

### 6.5 Transit (CTC)
- `hmrc ctc departure submit <xml>` — submit departure
- `hmrc ctc arrival submit <xml>` — submit arrival
- `hmrc ctc messages get <mrn>` — get messages
- `hmrc ctc guarantee balance <grn>` — check guarantee balance

### 6.6 Other customs
- `hmrc customs inventory-link <declaration-id> <consignment>` — link export to consignment

---

## Phase 7 — Companies House Filing

**Goal:** Submit statutory filings to Companies House.

**Milestone:** `phase-7-ch-filing`

### 7.1 REST Filing API
- `ch file transaction create <company-number>` — create transaction
- `ch file registered-office <txn-id> <address-json>` — file registered office change
- `ch file registered-email <txn-id> <email>` — file registered email
- `ch file submit <txn-id>` — submit transaction
- `ch file status <txn-id>` — check filing status

### 7.2 XML Gateway (comprehensive filing)
- `ch xmlgw submit <form-type> <xml-file>` — submit XML form
- `ch xmlgw status <submission-id>` — check submission status
- `ch xmlgw forms` — list supported form types

---

## Phase 8 — HMRC Tax Filing

**Goal:** Submit tax returns and payroll data.

**Milestone:** `phase-8-hmrc-filing`

### 8.1 Corporation Tax
- `hmrc ct submit <ct600-xml>` — submit CT600
- `hmrc ct amend <ct600-xml>` — amend CT600

### 8.2 VAT returns
- `hmrc vat return submit <vrn> <return-json>` — submit VAT return

### 8.3 PAYE / RTI
- `hmrc paye fps submit <xml>` — submit Full Payment Submission
- `hmrc paye eps submit <xml>` — submit Employer Payment Summary

### 8.4 Specialist filings
- `hmrc irr submit <json>` — Interest Restriction Return
- `hmrc cis deductions create <json>` — CIS deductions
- `hmrc ecsl submit <xml>` — EC Sales List

---

## Phase 9 — Platform Services & Specialist APIs

**Goal:** Government platform integrations and remaining specialist APIs.

**Milestone:** `phase-9-platform`

### 9.1 GOV.UK Pay
- `govuk pay create <amount> <description>` — create payment
- `govuk pay status <payment-id>` — payment status
- `govuk pay refund <payment-id> <amount>` — refund payment
- `govuk pay search [--from <date>] [--to <date>]` — search payments

### 9.2 GOV.UK Notify
- `govuk notify email <template-id> <email> <personalisation-json>` — send email
- `govuk notify sms <template-id> <phone> <personalisation-json>` — send SMS
- `govuk notify status <notification-id>` — delivery status

### 9.3 DEFRA
- `defra ehc apply <json>` — apply for export health certificate
- `defra ehc status <application-id>` — check application status

### 9.4 Remaining customs
- `hmrc customs nes submit <xml>` — National Export System
- `hmrc customs ncts submit <xml>` — NCTS transit declaration
- `hmrc customs emcs submit <xml>` — EMCS excise movement
- `hmrc customs ics-ni submit <xml>` — Import Control System NI

---

## Cross-Cutting Concerns (All Phases)

### Output contract
Every command returns JSON to stdout in one of two envelopes:

```json
{
  "ok": true,
  "data": { ... },
  "meta": {
    "api": "companies-house-public-data",
    "endpoint": "/company/12345678",
    "timestamp": "2026-03-23T15:00:00Z",
    "rate_limit_remaining": 580
  }
}
```

```json
{
  "ok": false,
  "error": {
    "code": "not_found",
    "message": "Company 99999999 not found",
    "api_status": 404,
    "api": "companies-house-public-data"
  }
}
```

### Global flags
- `--output json|pretty|compact` — output format (default: `json` = pretty)
- `--quiet` — suppress non-essential output
- `--config <path>` — config file override
- `--profile <name>` — credential profile
- `--sandbox` — use sandbox/test environment where available
- `--dry-run` — for write operations, validate without submitting

---

## GitHub Issue Structure

Each phase becomes a GitHub **milestone**. Within each milestone:

- One **epic issue** per numbered subsection (e.g., "1.1 Company profile")
- Individual **task issues** for each command within the epic
- A **testing issue** for integration tests per subsection
- A **documentation issue** for command help text and examples

Labels: `phase-0` through `phase-9`, `enhancement`, `testing`, `documentation`
