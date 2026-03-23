# 3. HMRC — Corporate Tax

[Back to index](../uk-government-apis.md)

---

## 3.1 Corporation Tax Online (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** XML-based API for submitting or amending Company Tax Returns (CT600). Company accounts and tax computations must be attached in iXBRL (inline XBRL) format, wrapped within the CT600 XML. Covers the full CT600 form including supplementary pages. Submissions undergo automatic parsing and validation before acceptance.

**Hypothesised Utility:** The primary channel for filing corporation tax returns. Any tax preparation/filing service for companies must integrate with this API. iXBRL generation for accounts and computations is a prerequisite.

**Resources:**
- [HMRC Developer Hub — CT Online](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/Corporation%20Tax%20Online)
- [CT600 technical specifications collection](https://www.gov.uk/government/collections/corporation-tax-online-support-for-software-developers)
- [CT600 valid XML samples](https://www.gov.uk/government/publications/corporation-tax-technical-specifications-ct600-valid-xml-samples)
- [CT600 RIM artefacts](https://www.gov.uk/government/collections/corporation-tax-online-support-for-software-developers) (updated Dec 2025)
- [XBRL and iXBRL format specifications](https://www.gov.uk/government/collections/corporation-tax-online-support-for-software-developers) (updated Nov 2025)
- [CT Online XBRL Technical Pack (PDF)](https://assets.publishing.service.gov.uk/media/5d84bef7e5274a27c2c6d5aa/CT_Online_XBRL_Technical_Pack_2.0.pdf)
- Contact: SDSTeam@hmrc.gov.uk

---

## 3.2 Interest Restriction Return (IRR) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 (`write:interest-restriction-return` scope) |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v1.0) |

**Description:** Manages Corporate Interest Restriction returns for large corporate groups. Allows appointing or revoking a Reporting Company's access, and submitting full or abbreviated Interest Restriction Returns under the Corporate Interest Restriction (CIR) rules.

**Hypothesised Utility:** Required for large corporate groups that need to file interest restriction returns. Relevant for tax advisory and compliance services working with groups subject to CIR.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/interest-restriction-return/1.0)
- [End-to-end service guide](https://developer.service.hmrc.gov.uk/guides/irr-api-end-to-end-service-guide/)

---

## 3.3 CIS Deductions (MTD) API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live (v3.0) |

**Description:** Manages Construction Industry Scheme (CIS) deductions under Making Tax Digital. Allows creation, retrieval, amendment, and deletion of CIS deductions data for subcontractors in the construction industry.

**Hypothesised Utility:** Required for construction companies and their tax advisers to report CIS deductions digitally.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)
