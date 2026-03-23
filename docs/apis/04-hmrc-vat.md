# 4. HMRC — VAT & Indirect Tax

[Back to index](../uk-government-apis.md)

---

## 4.1 VAT (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v1.0) |

**Description:** Making Tax Digital for VAT. Mandatory for VAT-registered businesses above the VAT threshold. Supports: submitting VAT returns, viewing submitted returns, retrieving VAT obligations (filing deadlines), viewing VAT liabilities, and viewing VAT payments. Digital record-keeping software interacts directly with HMRC systems.

**Hypothesised Utility:** Essential for any service handling VAT compliance for corporate entities. Covers the full VAT return lifecycle from obligation tracking through submission to payment reconciliation.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/vat-api)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/vat-mtd/)
- Contact: SDSTeam@hmrc.gov.uk

---

## 4.2 EC Sales List Online (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** XML-based API for submitting EC Sales Lists (ECSL) for VAT-registered traders who supply goods or services to VAT-registered businesses in EU member states.

**Hypothesised Utility:** Relevant for businesses with EU trade that need to file ECSLs. May be relevant post-Brexit for Northern Ireland Protocol obligations.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/EC%20Sales%20List%20Online)
