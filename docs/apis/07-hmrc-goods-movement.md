# 7. HMRC — Goods Movement

[Back to index](../uk-government-apis.md)

---

## 7.1 Goods Vehicle Movement Service (GVMS) Haulier API

| | |
|---|---|
| **Type** | REST (JSON) |
| **Auth** | OAuth 2.0 (organisation-restricted) |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Beta (v1.0) |

**Description:** Manages Goods Movement Records (GMRs) for cross-border vehicle movements. Links multiple customs declaration references into a single frontier presentation reference. Supports creating, updating, deleting, and listing GMRs. Tracks movement lifecycle states (OPEN, CHECKED_IN, EMBARKED) and inspection requirements. Supports barcode generation (Code 128) for check-in scanning.

**Hypothesised Utility:** Essential for hauliers, freight forwarders, and logistics companies moving goods across UK borders. Consolidates all customs references into one record per vehicle crossing.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/goods-movement-system-haulier-api/1.0)
- [GVMS end-to-end service guide](https://developer.service.hmrc.gov.uk/guides/gvms-end-to-end-service-guide/)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/goods-vehicle-movements/#goods-vehicle-movements)

---

## 7.2 Common Transit Convention (CTC) Traders API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Status** | Live (v2.1) |

**Description:** Enables sending departure and arrival notifications to the New Computerised Transit System (NCTS) and retrieving messages. Manages transit declarations for goods moving under the Common Transit Convention.

**Hypothesised Utility:** Required for businesses and agents managing goods transit across the UK and CTC partner countries.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/common-transit-convention-traders/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/common-transit-convention-traders/)

---

## 7.3 CTC Guarantee Balance API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v2.0) |

**Description:** Checks the balance remaining on transit guarantee accounts used for Common Transit Convention movements.

**Hypothesised Utility:** Allows transit operators to verify they have sufficient guarantee balance before initiating transit movements.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api)
