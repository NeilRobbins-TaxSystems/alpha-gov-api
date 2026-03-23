# 6. HMRC — Customs & Declarations

[Back to index](../uk-government-apis.md)

---

## 6.1 Customs Declarations API

| | |
|---|---|
| **Type** | REST (XML payloads) |
| **Auth** | OAuth 2.0 |
| **Production** | `https://api.service.hmrc.gov.uk` |
| **Sandbox** | `https://test-api.service.hmrc.gov.uk` |
| **Status** | Live |

**Description:** Core API for the Customs Declaration Service (CDS), which has replaced CHIEF. Enables submission, amendment, and cancellation of import and export customs declarations. Declarations use WCO (World Customs Organisation) standard XML format. Supports the full customs declaration lifecycle including status queries and notifications.

**Hypothesised Utility:** Essential for any business or logistics provider involved in importing/exporting goods to/from the UK. The primary digital channel for customs compliance.

**Resources:**
- [Customs Declarations end-to-end service guide](https://developer.service.hmrc.gov.uk/guides/customs-declarations-end-to-end-service-guide/)
- [Developer setup guide](https://developer.service.hmrc.gov.uk/guides/customs-declarations-end-to-end-service-guide/documentation/set-up-developers.html)
- [Submitting declarations](https://developer.service.hmrc.gov.uk/guides/customs-declarations-end-to-end-service-guide/documentation/submitting-import-and-export-customs-declarations.html)

---

## 6.2 Customs Inventory Linking Exports API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Links export declarations with specific consignments through consolidation, movement, and query operations. Enables cargo tracking through temporary storage facilities at ports and airports.

**Hypothesised Utility:** Required for businesses managing export logistics, linking customs declarations to physical goods movements at ports.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/customs-inventory-linking-exports/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/customs-inventory-linking-exports/)

---

## 6.3 Safety and Security Import Declarations API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Enables submission of Entry Summary Declarations (ENS) for goods being imported into Great Britain. Part of the Safety and Security (S&S GB) system providing advance cargo information to customs authorities.

**Hypothesised Utility:** Required for carriers and freight forwarders importing goods into GB. Provides advance notification to customs before goods arrive.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/import-control-entry-declaration-store/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/safety-and-security-import-declarations/)

---

## 6.4 Safety and Security Import Notifications API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Retrieves and acknowledges notifications relating to ENS submissions — including movement reference numbers, intervention notifications, and error messages.

**Hypothesised Utility:** Companion to the Import Declarations API. Allows software to receive and process customs responses to ENS submissions.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/import-control-entry-declaration-intervention/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/safety-and-security-import-notifications/)

---

## 6.5 Safety and Security Import Outcomes API

| | |
|---|---|
| **Type** | REST |
| **Auth** | OAuth 2.0 |
| **Status** | Live (v1.0) |

**Description:** Retrieves and acknowledges ENS outcome results — the final customs decision on advance cargo information submissions.

**Hypothesised Utility:** Completes the S&S import workflow by providing the customs clearance outcome for advance declarations.

**Resources:**
- [API specification](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/service/import-control-entry-declaration-outcome/1.0)
- [API Catalogue entry](https://www.api.gov.uk/hmrc/safety-and-security-import-outcomes/)

---

## 6.6 Import Control System Northern Ireland (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** Handles electronic Entry Summary Declarations for goods imported into Northern Ireland and for GB-to-NI goods movements under the Windsor Framework.

**Hypothesised Utility:** Required for businesses moving goods into Northern Ireland from outside the UK or from Great Britain.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/Import%20Control%20System)

---

## 6.7 National Export System (NES) (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** Electronic export declaration system. Handles mandatory export declarations and associated authorisation requirements.

**Hypothesised Utility:** Required for businesses exporting goods from the UK that must submit electronic export declarations.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/National%20Export%20System)

---

## 6.8 New Computerised Transit System (NCTS) (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** Processes Union Transit and TIR (Transports Internationaux Routiers) declarations electronically. Part of the common transit framework.

**Hypothesised Utility:** Required for businesses moving goods under transit procedures across the UK and into/out of the common transit area.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/New%20Computerised%20Transit%20System)

---

## 6.9 Excise Movement Control System (EMCS) (XML)

| | |
|---|---|
| **Type** | XML |
| **Auth** | Government Gateway credentials |
| **Status** | Live |

**Description:** Records duty-suspended movements of excise goods (alcohol, tobacco, energy products) within the UK and between the UK and EU. Manages electronic Administrative Documents (e-AD) for excise movements.

**Hypothesised Utility:** Required for businesses dealing in excise goods that move under duty suspension — breweries, distilleries, tobacco manufacturers, fuel distributors, etc.

**Resources:**
- [HMRC Developer Hub](https://developer.service.hmrc.gov.uk/api-documentation/docs/api/xml/Excise%20Movement%20Control%20System)
