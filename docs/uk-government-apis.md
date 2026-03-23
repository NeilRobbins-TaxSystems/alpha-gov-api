# UK Government APIs for Corporate Services

APIs relevant to the preparation, filing, and lifecycle management of company information, tax returns, statutory accounts, payments, customs declarations, and trade — focused on corporate entities.

---

## 1. [Companies House — Company Information & Search](apis/01-ch-company-information.md)

| API | Type | Utility |
|-----|------|---------|
| [Public Data API](apis/01-ch-company-information.md#11-companies-house-public-data-api) | REST | Core data source for company lookup, verification, officer checks, charge searches, and monitoring filing compliance. Essential for validating or displaying company information prior to filing or payment. |
| [Streaming API](apis/01-ch-company-information.md#12-companies-house-streaming-api) | REST (streaming) | Real-time monitoring of company changes — detect filing events, officer changes, charges, or insolvency as they happen. |
| [Document API](apis/01-ch-company-information.md#13-companies-house-document-api) | REST | Retrieve previously filed statutory accounts and documents for review, audit trail, or pre-population of new filings. |
| [Discrepancies API](apis/01-ch-company-information.md#14-companies-house-discrepancies-api) | REST | Programmatically report PSC discrepancies discovered during due diligence or KYC processes. |
| [Free Company Data Product](apis/01-ch-company-information.md#15-companies-house-free-company-data-product) | Bulk CSV | Initial data loads, offline analysis, or building local indexes of company data. |

---

## 2. [Companies House — Filing & Transactions](apis/02-ch-filing-transactions.md)

| API | Type | Utility |
|-----|------|---------|
| [REST Filing API](apis/02-ch-filing-transactions.md#21-manipulate-company-data--api-filing-rest) | REST | Modern programmatic filing of company changes (registered office, email, insolvency). Expanding to replace XML Gateway long-term. |
| [XML Gateway](apis/02-ch-filing-transactions.md#22-companies-house-xml-gateway) | XML/SOAP | Most comprehensive filing channel — 30+ form types including accounts, officer changes, confirmation statements, PSC, charges, and incorporations. |
| [Identity Service](apis/02-ch-filing-transactions.md#23-companies-house-identity-service) | REST (OAuth 2.0) | Authentication for the REST Filing API — issues bearer tokens for filing on behalf of authorised officers/agents. |
| [Sandbox Test Data Generator](apis/02-ch-filing-transactions.md#24-sandbox-test-data-generator-api) | REST | Generate test companies and data for development and testing of filing integrations. |

---

## 3. [HMRC — Corporate Tax](apis/03-hmrc-corporate-tax.md)

| API | Type | Utility |
|-----|------|---------|
| [Corporation Tax Online](apis/03-hmrc-corporate-tax.md#31-corporation-tax-online-xml) | XML | Primary channel for filing CT600 corporation tax returns with iXBRL accounts and computations. |
| [Interest Restriction Return (IRR)](apis/03-hmrc-corporate-tax.md#32-interest-restriction-return-irr-api) | REST | Filing interest restriction returns for large corporate groups subject to CIR rules. |
| [CIS Deductions (MTD)](apis/03-hmrc-corporate-tax.md#33-cis-deductions-mtd-api) | REST | Digital reporting of Construction Industry Scheme deductions. |

---

## 4. [HMRC — VAT & Indirect Tax](apis/04-hmrc-vat.md)

| API | Type | Utility |
|-----|------|---------|
| [VAT (MTD)](apis/04-hmrc-vat.md#41-vat-mtd-api) | REST | Full VAT return lifecycle — obligations, submission, liabilities, and payments. Mandatory for businesses above VAT threshold. |
| [EC Sales List Online](apis/04-hmrc-vat.md#42-ec-sales-list-online-xml) | XML | ECSL submissions for VAT-registered traders with EU trade. |

---

## 5. [HMRC — Payroll & Employment](apis/05-hmrc-payroll.md)

| API | Type | Utility |
|-----|------|---------|
| [PAYE Online (RTI)](apis/05-hmrc-payroll.md#51-paye-online-xml--real-time-information) | XML | Real Time Information employer submissions (FPS, EPS). Core to payroll processing, tax deduction reporting, and NI contributions. |

---

## 6. [HMRC — Customs & Declarations](apis/06-hmrc-customs-declarations.md)

| API | Type | Utility |
|-----|------|---------|
| [Customs Declarations API](apis/06-hmrc-customs-declarations.md#61-customs-declarations-api) | REST | Core CDS API for submitting/amending/cancelling import and export customs declarations. |
| [Customs Inventory Linking Exports](apis/06-hmrc-customs-declarations.md#62-customs-inventory-linking-exports-api) | REST | Link export declarations to physical consignments at ports and airports. |
| [Safety & Security Import Declarations](apis/06-hmrc-customs-declarations.md#63-safety-and-security-import-declarations-api) | REST | Submit Entry Summary Declarations (ENS) for goods imported into Great Britain. |
| [Safety & Security Import Notifications](apis/06-hmrc-customs-declarations.md#64-safety-and-security-import-notifications-api) | REST | Receive and process customs responses to ENS submissions. |
| [Safety & Security Import Outcomes](apis/06-hmrc-customs-declarations.md#65-safety-and-security-import-outcomes-api) | REST | Retrieve final customs clearance outcomes for advance declarations. |
| [Import Control System NI](apis/06-hmrc-customs-declarations.md#66-import-control-system-northern-ireland-xml) | XML | Entry Summary Declarations for NI imports and GB-to-NI movements. |
| [National Export System (NES)](apis/06-hmrc-customs-declarations.md#67-national-export-system-nes-xml) | XML | Electronic export declarations. |
| [NCTS](apis/06-hmrc-customs-declarations.md#68-new-computerised-transit-system-ncts-xml) | XML | Union Transit and TIR declarations. |
| [EMCS](apis/06-hmrc-customs-declarations.md#69-excise-movement-control-system-emcs-xml) | XML | Duty-suspended movements of excise goods (alcohol, tobacco, energy). |

---

## 7. [HMRC — Goods Movement](apis/07-hmrc-goods-movement.md)

| API | Type | Utility |
|-----|------|---------|
| [GVMS Haulier API](apis/07-hmrc-goods-movement.md#71-goods-vehicle-movement-service-gvms-haulier-api) | REST | Manage Goods Movement Records (GMRs) for cross-border vehicle movements — consolidates customs references into one frontier presentation. |
| [CTC Traders API](apis/07-hmrc-goods-movement.md#72-common-transit-convention-ctc-traders-api) | REST | Send departure/arrival notifications for goods transiting under the Common Transit Convention. |
| [CTC Guarantee Balance](apis/07-hmrc-goods-movement.md#73-ctc-guarantee-balance-api) | REST | Check remaining balance on transit guarantee accounts. |

---

## 8. [HMRC — Payments & Agent Services](apis/08-hmrc-payments-agents.md)

| API | Type | Utility |
|-----|------|---------|
| [Initiate Payment API](apis/08-hmrc-payments-agents.md#81-initiate-payment-api) | REST | Trigger HMRC payment journeys from software — seamless filing-to-payment experience. Supports SA and VAT. |
| [Agent Authorisation API](apis/08-hmrc-payments-agents.md#82-agent-authorisation-api) | REST | Digital replacement for 64-8 form — establish agent-client authority for MTD filing. |
| [Push Pull Notifications API](apis/08-hmrc-payments-agents.md#83-push-pull-notifications-api) | REST | Event-driven notifications for customs declarations, filing status changes, and other async events. |

---

## 9. [HMRC — Business & Obligations](apis/09-hmrc-business-obligations.md)

| API | Type | Utility |
|-----|------|---------|
| [Business Details (MTD)](apis/09-hmrc-business-obligations.md#91-business-details-mtd-api) | REST | Identify businesses associated with a taxpayer and retrieve their MTD configuration. |
| [Business Income Source Summary (MTD)](apis/09-hmrc-business-obligations.md#92-business-income-source-summary-mtd-api) | REST | Retrieve summarised business financial data for tax calculation. |
| [Business Source Adjustable Summary (MTD)](apis/09-hmrc-business-obligations.md#93-business-source-adjustable-summary-mtd-api) | REST | Compute and adjust business profit figures before final tax calculation. |
| [Obligations (MTD)](apis/09-hmrc-business-obligations.md#94-obligations-mtd-api) | REST | Track filing deadlines, due periods, and fulfilment status. |
| [Exchange Rates from HMRC](apis/09-hmrc-business-obligations.md#95-exchange-rates-from-hmrc-xml) | XML | HMRC-approved exchange rates for foreign currency tax computations. |

---

## 10. [Trade & Tariff](apis/10-trade-tariff.md)

| API | Type | Utility |
|-----|------|---------|
| [GOV.UK Trade Tariff API](apis/10-trade-tariff.md#101-govuk-trade-tariff-api) | REST | Core reference data — commodity codes, duty rates, VAT rates, import/export controls. |
| [Fast Parcel Operator (FPO) API](apis/10-trade-tariff.md#102-trade-tariff-fast-parcel-operator-fpo-api) | REST | Automatic commodity classification from product descriptions for high-volume customs processing. |
| [Trader Goods Profile (TGP) API](apis/10-trade-tariff.md#103-trader-goods-profile-tgp-api) | REST | Maintain pre-approved profiles of regularly traded goods to streamline declarations. |
| [UK Global Tariff — Measures as Defined](apis/10-trade-tariff.md#104-uk-global-tariff--measures-as-defined-api) | REST | Commodity code hierarchy with preferential and MFN tariff measure definitions. |
| [UK Global Tariff — Declarable Commodities](apis/10-trade-tariff.md#105-uk-global-tariff--measures-on-declarable-commodities-api) | REST | Duty rates and measures at the 10-digit declarable commodity level. |
| [Check EORI Number](apis/10-trade-tariff.md#106-check-eori-number-api) | REST | Validate GB EORI numbers and retrieve associated business details. |
| [Check Barriers to Trading Abroad](apis/10-trade-tariff.md#107-check-barriers-to-trading-and-investing-abroad-api) | REST | Trade barriers data affecting UK businesses in international markets. |
| [Bulk Data File List](apis/10-trade-tariff.md#108-bulk-data-file-list-api) | REST | Download daily/monthly/annual tariff datasets for offline classification. |
| [DBT Data API](apis/10-trade-tariff.md#109-department-for-business-and-trade-dbt-data-api) | REST | Open trade datasets — tariffs, quotas, market barriers. Multiple output formats. |

---

## 11. [DEFRA — Trade & Export](apis/11-defra.md)

| API | Type | Utility |
|-----|------|---------|
| [Export Health Certificates](apis/11-defra.md#111-export-health-certificates-api) | REST | Apply for export health certificates for agri-food goods and track application status. |

---

## 12. [GOV.UK Platform Services](apis/12-govuk-platform.md)

| API | Type | Utility |
|-----|------|---------|
| [GOV.UK Pay](apis/12-govuk-platform.md#121-govuk-pay-api) | REST | PCI-compliant government payment processing — cards, Apple/Google Pay, refunds, reconciliation. |
| [GOV.UK Notify](apis/12-govuk-platform.md#122-govuk-notify-api) | REST | Send filing confirmations, deadline reminders, payment receipts via email, SMS, or letter. |

---

## Key Developer Resources

| Resource | URL |
|----------|-----|
| UK Government API Catalogue | https://www.api.gov.uk/ |
| Companies House Developer Hub | https://developer.company-information.service.gov.uk/ |
| Companies House API Specifications | https://developer-specs.company-information.service.gov.uk/ |
| Companies House Developer Forum | https://forum.companieshouse.gov.uk/ |
| Companies House XML Gateway Forum | https://xmlforum.companieshouse.gov.uk/ |
| HMRC Developer Hub | https://developer.service.hmrc.gov.uk/api-documentation |
| HMRC API Documentation | https://developer.service.hmrc.gov.uk/api-documentation/docs/api |
| HMRC Software Development Guidance | https://www.gov.uk/guidance/software-development-apis-available-on-the-hmrc-developer-hub |
| GOV.UK Pay Documentation | https://docs.payments.service.gov.uk/ |
| Trade Tariff Developer Hub | https://hub.trade-tariff.service.gov.uk/ |
| DBT Data API | https://data.api.trade.gov.uk/ |
| DEFRA Trade Developer Portal | https://developer-portal.trade.defra.gov.uk/explore-apis |
| API Catalogue — Borders | https://www.api.gov.uk/Borders/#borders |
| API Catalogue — HMRC | https://www.api.gov.uk/hmrc/ |

---

## Important Notes

- **Joint Filing Closure:** The Companies House / HMRC joint filing service for accounts and Company Tax Returns is closing on **31 March 2026**. Software will need to file to each organisation separately.
- **XML to REST Migration:** Companies House plans to replace the XML Gateway with REST APIs long-term, but no firm timeline has been announced.
- **iXBRL Requirement:** Corporation tax returns require accounts and computations in iXBRL format. These cannot be submitted as standalone documents — they must be wrapped in CT600 XML.
- **MTD Mandation:** Making Tax Digital for VAT is mandatory for businesses above the VAT threshold. MTD for Income Tax is being rolled out progressively.
- **HMRC API Strategy:** HMRC's strategy is to move all tax services to RESTful APIs over time, replacing legacy XML services.

*Last updated: March 2026*
