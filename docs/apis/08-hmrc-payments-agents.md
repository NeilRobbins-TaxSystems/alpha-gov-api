# 8. HMRC — Payments & Agent Services

[Back to index](../uk-government-apis.md)

---

## 8.1 Initiate Payment API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 (application-restricted, client credentials) |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v1.0) |

**Description:** Triggers HMRC payment journeys from third-party software. Creates a redirect URL for customers to complete payment through HMRC's payment portal, with optional return-to-software functionality. Currently supports Self Assessment (UTR) and VAT (VRN) payment types. Provides payment status tracking (Completed, Failed, InProgress).

**Hypothesised Utility:** Enables software to initiate tax payments directly — a seamless user experience from filing to payment within a single application.

**Resources:**
- [API specification (OpenAPI)](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/third-party-payments-external-api/1.0/oas/resolved)

---

## 8.2 Agent Authorisation API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v1.0) |

**Description:** Replaces the 64-8 paper form. Allows agents to create authorisation requests to act on clients' behalf for MTD services (VAT and Income Tax). Supports creating, checking status, and cancelling authorisation requests. Uses Agent Reference Numbers (ARN). Requests expire after 21 days.

**Hypothesised Utility:** Required for accountancy practices and tax agents that file on behalf of corporate clients. Establishes the digital authority chain needed before submitting returns via MTD APIs.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/agent-authorisation-api/1.0)

---

## 8.3 Push Pull Notifications API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Manages automated notifications for asynchronous API request events across HMRC services. Supports both push (webhook) and pull notification patterns. Used by CDS and other APIs to deliver status updates on submissions and declarations.

**Hypothesised Utility:** Enables event-driven architectures — software can receive notifications when customs declarations are processed, filings accepted, or status changes occur, rather than polling for updates.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/api-notification-pull/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/push-pull-notifications/)
