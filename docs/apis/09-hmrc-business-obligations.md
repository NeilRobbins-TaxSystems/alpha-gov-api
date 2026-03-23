# 9. HMRC — Business & Obligations

[Back to index](../uk-government-apis.md)

---

## 9.1 Business Details (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v2.0) |

**Description:** Lists businesses associated with a National Insurance number and retrieves detailed business information. Provides business type, trading name, accounting periods, and other metadata.

**Hypothesised Utility:** Useful for software that needs to identify all businesses associated with a taxpayer and retrieve their MTD configuration.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)

---

## 9.2 Business Income Source Summary (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v3.0) |

**Description:** Provides income and expenditure summary data for businesses, giving a consolidated view of business financial performance for tax calculation purposes.

**Hypothesised Utility:** Useful for tax preparation software to retrieve summarised business financial data before computing tax liabilities.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)

---

## 9.3 Business Source Adjustable Summary (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v7.0) |

**Description:** Manages Business Source Adjustable Summary (BSAS) calculations and adjustments. Allows triggering BSAS calculations and submitting accounting adjustments for self-employment, UK property, and foreign property businesses.

**Hypothesised Utility:** Required for tax preparation software that needs to compute and adjust business profit figures before final tax calculation.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)

---

## 9.4 Obligations (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v3.0) |

**Description:** Retrieves business obligations — the deadlines and periods for which returns must be filed. Covers quarterly update obligations, annual obligations, and end-of-period statements.

**Hypothesised Utility:** Essential for any compliance monitoring service — track what's due, when, and whether it's been fulfilled.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)

---

## 9.5 Exchange Rates from HMRC (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | None |
| **Status** | Live |

**Description:** Provides HMRC-published exchange rates including monthly rates, yearly averages, spot rates, and weekly amendments. Used for converting foreign currency transactions to GBP for tax purposes.

**Hypothesised Utility:** Required for any tax computation involving foreign currency — ensures the correct HMRC-approved rates are used.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/Exchange%20rates%20from%20HMRC)
