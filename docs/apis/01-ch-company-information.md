# 1. Companies House — Company Information & Search

[Back to index](../uk-government-apis.md)

---

## 1.1 Companies House Public Data API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | API Key / Basic Auth |
| **Base URL** | `https://api.company-information.service.gov.uk` |
| **Status** | Live |

**Description:** Read-only API providing access to all public company data held by Companies House, free of charge. Covers company profiles, registered office addresses, officer lists and appointments, persons with significant control (PSC), filing history, charges (secured lending), insolvency cases, company registers, exemptions, disqualified officers, and UK establishments of overseas companies. Also provides advanced search across companies, officers, and disqualified officers.

**Hypothesised Utility:** Core data source for company lookup, verification, officer checks, charge searches, and monitoring filing compliance. Essential for any service that needs to validate or display company information prior to filing or payment.

**Resources:**
- [API specification (OpenAPI)](https://developer-specs.company-information.service.gov.uk/companies-house-public-data-api/reference)
- [Developer Hub overview](https://developer.company-information.service.gov.uk/overview)
- [Getting started guide](https://developer.company-information.service.gov.uk/get-started/)
- [API Catalogue entry](https://www.api.gov.uk/ch/companies-house/#companies-house)
- [Developer Forum](https://forum.companieshouse.gov.uk/)

---

## 1.2 Companies House Streaming API

| | |
|---|---|
| **Type** | REST (streaming JSON) |
| **Auth** | Stream Key / API Key / Basic Auth |
| **Base URL** | `https://stream.company-information.service.gov.uk` |
| **Status** | Live |

**Description:** Real-time streaming API providing continuous change feeds for 9 data types: company information, filing history, officers, persons with significant control, PSC statements, charges, insolvency cases, disqualified officers, and company exemptions. Delivers updates as they are recorded at Companies House.

**Hypothesised Utility:** Enables real-time monitoring of company changes — essential for services that need to detect filing events, officer changes, new charges, or insolvency proceedings as they happen, rather than polling the Public Data API.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/streaming-api/reference)
- [API Catalogue entry](https://www.api.gov.uk/ch/companies-house-streaming/#companies-house-streaming)
- [Blog: Launching our streaming API](https://companieshouse.blog.gov.uk/2019/10/24/launching-our-streaming-api-service-for-company-data/)

---

## 1.3 Companies House Document API

| | |
|---|---|
| **Type** | REST (JSON + binary) |
| **Auth** | API Key / Bearer Token / Basic Auth |
| **Base URL** | `https://document-api.company-information.service.gov.uk` |
| **Status** | Live |

**Description:** Retrieves company filing documents and their metadata. Documents include filed accounts, annual returns/confirmation statements, officer forms, and other statutory documents. Supports document metadata retrieval and content download (PDF, iXBRL/XHTML).

**Hypothesised Utility:** Allows retrieval of previously filed statutory accounts and other documents for review, audit trail, or pre-population of new filings. Useful for fetching iXBRL accounts that have been submitted.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/document-api/reference)

---

## 1.4 Companies House Discrepancies API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | Bearer Token |
| **Base URL** | `https://api.company-information.service.gov.uk` |
| **Status** | Live |

**Description:** Allows obliged entities (as defined under anti-money laundering regulations) to report discrepancies in PSC (Persons with Significant Control) information held by Companies House.

**Hypothesised Utility:** Relevant for regulated entities that need to programmatically report discrepancies discovered during due diligence or KYC processes.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/discrepancies/reference)

---

## 1.5 Companies House Free Company Data Product

| | |
|---|---|
| **Type** | Bulk CSV download |
| **Auth** | None |
| **Status** | Live |

**Description:** Downloadable bulk dataset of basic company information — company number, name, registered address, company type, status, incorporation date, accounts and confirmation statement dates. Available as a single large file or multiple smaller files. Updated regularly.

**Hypothesised Utility:** Useful for initial data loads, offline analysis, or building local indexes of company data without needing to call the API for each company individually.

**Resources:**
- [Download portal](https://download.companieshouse.gov.uk/en_output.html)
