# alpha-gov-api

A Rust CLI tool that exposes government APIs as reliable, structured interfaces for consumption by agentic AIs.

**alpha-gov-api** provides a unified command-line interface to UK government APIs covering company information, tax, statutory accounts, customs, trade, and payments. Every command outputs structured JSON, making it straightforward for AI agents to invoke via shell and parse the results.

## Scope

The tool integrates with APIs from:

- **Companies House** — company data, filing, documents, real-time streaming
- **HMRC** — corporation tax, VAT, PAYE, customs declarations, goods movement, payments
- **Department for Business and Trade** — trade tariffs, quotas, barriers
- **DEFRA** — export health certificates
- **GOV.UK Platform** — payments (GOV.UK Pay), notifications (GOV.UK Notify)

UK is the first jurisdiction. The architecture supports future expansion to other jurisdictions.

## Design Principles

- **Structured JSON output** — every command returns parseable JSON to stdout, errors to stderr
- **Agentic AI first** — commands are self-describing, predictable, and composable
- **Phased delivery** — APIs integrated in priority order (see [plan](docs/plan.md))
- **Configuration as code** — API keys and credentials managed via config files and environment variables

## Status

Early development. See [docs/plan.md](docs/plan.md) for the integration roadmap.

## API Documentation

See [docs/uk-government-apis.md](docs/uk-government-apis.md) for the full catalogue of APIs this tool will integrate with, organised by provider and priority.
