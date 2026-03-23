# 12. GOV.UK Platform Services

[Back to index](../uk-government-apis.md)

---

## 12.1 GOV.UK Pay API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 Bearer Token (API key) |
| **Base URL** | `https://publicapi.payments.service.gov.uk` |
| **Status** | Live |

**Description:** Government payment processing platform. Supports creating and managing one-time payments, recurring payment agreements, digital wallets (Apple Pay, Google Pay), MOTO payments, delayed capture, refund processing, payment search/reconciliation, dispute tracking, and webhook notifications. No charge for API usage — PSP transaction fees apply separately. Available to central government, local authorities, and NHS.

**Hypothesised Utility:** Could be used to accept payments for filing services or fees. Provides a pre-built, PCI-compliant payment journey that can be integrated into any government-facing or government-adjacent service.

**Resources:**
- [Technical documentation](https://docs.payments.service.gov.uk/)
- [API reference](https://docs.payments.service.gov.uk/api_reference/)
- [Integration guide](https://docs.payments.service.gov.uk/integrate_with_govuk_pay/)
- [Quick start guide](https://docs.payments.service.gov.uk/quick_start_guide/)
- [API Catalogue entry](https://www.api.gov.uk/gds/gov-uk-pay/)

---

## 12.2 GOV.UK Notify API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | API Key |
| **Status** | Live |

**Description:** Government notification platform for sending emails, text messages, and letters. Supports template-based messaging, batch sends via spreadsheet upload, delivery tracking, and status reporting. No monthly charge, no setup fee. Available to central government, local authorities, and NHS. Supported 24/7.

**Hypothesised Utility:** Could be used to send filing confirmations, deadline reminders, payment receipts, or status notifications to company directors, agents, or accountants.

**Resources:**
- [GOV.UK Notify](https://www.notifications.service.gov.uk/)
- [Features overview](https://www.notifications.service.gov.uk/features)
- [Using Notify](https://www.notifications.service.gov.uk/using-notify)
- [API Catalogue entry](https://www.api.gov.uk/gds/gov-uk-notify/)
