# 2. Companies House — Filing & Transactions

[Back to index](../uk-government-apis.md)

---

## 2.1 Manipulate Company Data — API Filing (REST)

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 Bearer Token |
| **Base URL** | `https://api.company-information.service.gov.uk` |
| **Sandbox** | `https://api-sandbox.company-information.service.gov.uk` |
| **Status** | Live (expanding) |

**Description:** REST API for filing changes to company data. Currently supports: registered office address changes, registered email address filings, insolvency proceedings (including practitioner appointments, attachments, progress reports, resolutions, and statements of affairs). Uses a transaction-based model — create a transaction, add filing resources, then submit. Companies House plans to expand this API to replace the XML Gateway in the long term.

**Hypothesised Utility:** The modern, programmatic way to submit company changes. As it expands to cover more form types, this will become the primary filing channel. The transaction model allows staged preparation before submission.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/manipulate-company-data-api-filing/reference)
- [Filing overview guide](https://developer-specs.company-information.service.gov.uk/manipulate-company-data-api-filing/guides/overview)
- [API testing guide](https://developer.company-information.service.gov.uk/api-testing)
- [Third Party Test Harness (example app)](https://github.com/companieshouse/confirmation-statement-api)

---

## 2.2 Companies House XML Gateway

| | |
|---|---|
| **Type** | XML/SOAP |
| **Auth** | Presenter ID + MD5-hashed password |
| **Gateway URL** | `https://xmlgw.companieshouse.gov.uk/v1-0/xmlgw/Gateway` |
| **Status** | Live (legacy — planned replacement by REST API) |

**Description:** Comprehensive electronic filing gateway supporting 30+ form types via XML submissions. Supported filings include:

**Accounts:** Annual accounts (including XBRL), change of accounting reference date (AA01)

**Officers:** Director appointment (AP01), corporate director appointment (AP02), secretary appointment (AP03/AP04), officer resignation (TM01/TM02), change of officer details (CH01-CH04) — plus LLP equivalents

**Company changes:** Registered office address change (AD01), SAIL address (AD02-AD04), company name change, company incorporation (IN01)

**Confirmation & PSC:** Confirmation statement (CS01), PSC notifications (PSC01-PSC04), PSC cessation, PSC verification statement

**Shares & capital:** Return of allotment (SH01), increase in nominal capital

**Charges:** Charge registration, charge update (satisfaction/release), charge search

**Utility services:** Document retrieval, submission status checks, e-reminders, payment period queries, company data queries

**Hypothesised Utility:** Currently the most comprehensive electronic filing channel. Essential for any service that needs to file the full range of company statutory forms. Note: Companies House plans to replace this with the REST API over time, but no firm timeline has been given.

**Resources:**
- [XML Gateway portal](https://xmlgw.companieshouse.gov.uk/)
- [Schema status & XSD downloads](http://xmlgw.companieshouse.gov.uk/SchemaStatus)
- [Important information for software developers](https://www.gov.uk/government/publications/technical-interface-specifications-for-companies-house-software/important-information-for-software-developers-read-first)
- [XML Gateway Forum](https://xmlforum.companieshouse.gov.uk/)
- Contact: xml@companieshouse.gov.uk

---

## 2.3 Companies House Identity Service

| | |
|---|---|
| **Type** | REST (OAuth 2.0) |
| **Auth** | Client credentials |
| **Sandbox** | `https://identity-sandbox.company-information.service.gov.uk` |
| **Status** | Live |

**Description:** OAuth 2.0 authentication and identity service for Companies House filing APIs. Issues access tokens for services that need to submit data on behalf of authorised company officers or agents.

**Hypothesised Utility:** Required for any application using the REST Filing API. Manages the authentication flow to obtain bearer tokens for filing operations.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/companies-house-identity-service/reference)

---

## 2.4 Sandbox Test Data Generator API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | API Key |
| **Base URL** | `https://test-data-sandbox.company-information.service.gov.uk` |
| **Status** | Sandbox only |

**Description:** Generates test company data on demand within the sandbox environment. Creates test companies with authentication codes that can be used to test filing operations.

**Hypothesised Utility:** Essential for development and testing of filing integrations without affecting live company data.

**Resources:**
- [API specification](https://developer-specs.company-information.service.gov.uk/sandbox-test-data-generator-api/reference)
- [API testing guide](https://developer.company-information.service.gov.uk/api-testing)
