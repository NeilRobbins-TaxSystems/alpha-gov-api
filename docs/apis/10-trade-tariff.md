# 10. Trade & Tariff

[Back to index](../uk-government-apis.md)

---

## 10.1 GOV.UK Trade Tariff API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | None |
| **Base URL** | `https://www.trade-tariff.service.gov.uk/uk/api` |
| **Status** | Live |

**Description:** Provides access to the UK trade tariff including commodity codes, import/export controls, customs duty rates, and VAT rates. Data is provided under the Open Government Licence.

**Hypothesised Utility:** Core reference data for customs declarations. Required for correctly classifying goods and determining applicable duties and controls before submitting declarations.

**Resources:**
- [API documentation portal](https://api.trade-tariff.service.gov.uk/)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/gov-uk-trade-tariff-api/#gov-uk-trade-tariff-api)
- Contact: hmrc-trade-tariff-support-g@digital.hmrc.gov.uk

---

## 10.2 Trade Tariff Fast Parcel Operator (FPO) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | API Key |
| **Base URL** | Via `https://hub.trade-tariff.service.gov.uk/` |
| **Status** | Live |

**Description:** Identifies probable commodity codes from product descriptions for rapid customs processing. Designed for high-volume parcel operators who need to classify goods quickly for customs declarations.

**Hypothesised Utility:** Useful for e-commerce and logistics businesses that need to classify large volumes of goods automatically for customs purposes.

**Resources:**
- [Developer Hub](https://hub.trade-tariff.service.gov.uk/)
- [FPO API documentation](https://api.trade-tariff.service.gov.uk/fpo.html)

---

## 10.3 Trader Goods Profile (TGP) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 (`trader-goods-profiles` scope) |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v1.0) |

**Description:** Manages trader commodity records for customs purposes. Supports creating, retrieving, updating, and removing goods profile records. Tracks commodity codes, EORI numbers, classification status, and advice request states. Integrates with Trade Tariff systems and flags when commodity codes expire or restrictions change.

**Hypothesised Utility:** Allows traders to maintain a pre-approved profile of regularly traded goods, streamlining customs declarations and reducing classification errors.

**Resources:**
- [API reference guide](https://developer.service.hmrc.gov.uk/guides/trader-goods-profile-service-guide/documentation/API-reference.html)
- [Service guide](https://developer.service.hmrc.gov.uk/guides/trader-goods-profile-service-guide/)

---

## 10.4 UK Global Tariff — Measures as Defined API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | None |
| **Status** | Live |

**Description:** Displays the commodity code hierarchy with tariff measure definitions. Shows preferential and Most Favoured Nation (MFN) tariff measures for imported goods under the UK's independent tariff policy (effective from 1 January 2021).

**Hypothesised Utility:** Reference data for understanding tariff structures and measure types at a commodity code level.

**Resources:**
- [API Catalogue entry](https://www.api.gov.uk/dbt/uk-global-tariff-measures-as-defined/)

---

## 10.5 UK Global Tariff — Measures on Declarable Commodities API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | None |
| **Status** | Live |

**Description:** Shows measures applicable to 10-digit commodity codes at the most granular classification level — the level used on actual customs declarations.

**Hypothesised Utility:** Provides the specific duty rates and measures needed when completing customs declarations at the declarable commodity level.

**Resources:**
- [API Catalogue entry](https://www.api.gov.uk/dbt/uk-global-tariff-measures-on-declarable-commodities/)

---

## 10.6 Check EORI Number API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | None |
| **Status** | Live (v2.0) |

**Description:** Validates whether an EORI number beginning with GB (issued by the UK) is valid and retrieves the registered business name and address associated with it.

**Hypothesised Utility:** Useful for verifying trading partners' EORI numbers before customs declarations and for due diligence on import/export counterparties.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/check-eori-number-api/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/check-an-eori-number/)

---

## 10.7 Check Barriers to Trading and Investing Abroad API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | None |
| **Status** | Live |

**Description:** Provides data on trade barriers affecting UK businesses trading and investing internationally. Published by the Department for Business and Trade (DBT).

**Hypothesised Utility:** Useful for advisory services helping UK businesses understand trade barriers in target markets before making export/import decisions.

**Resources:**
- [API Catalogue entry](https://www.api.gov.uk/dbt/check-barriers-to-trading-and-investing-abroad/)

---

## 10.8 Bulk Data File List API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Lists available bulk data files for download — distributes daily, monthly, and annual tariff files with download URLs for each file by information type.

**Hypothesised Utility:** Enables integration of full tariff datasets into customs software for offline classification and duty calculation.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/secure-data-exchange-bulk-download/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/bulk-data-file-list/)

---

## 10.9 Department for Business and Trade (DBT) Data API

| | |
|---|---|
| **Type** | REST (JSON, CSV, SQLite, Parquet, ODS) |
| **Auth** | None |
| **Base URL** | `https://data.api.trade.gov.uk` |
| **Status** | Live |

**Description:** Open data platform providing versioned, immutable trade datasets. Known datasets include market barriers, UK tariff data (2021 onwards), and UK trade quotas. Supports S3 Select queries for filtering and aggregation. Multiple output formats available.

**Hypothesised Utility:** Rich data source for trade analysis, tariff research, and quota monitoring. Useful for building reporting and analytics features around UK trade data.

**Resources:**
- [API documentation](https://data.api.trade.gov.uk/)
- [GitHub repository](https://github.com/uktrade/public-data-api)
