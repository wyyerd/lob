use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use serde::export::Formatter;
use std::collections::BTreeMap;

pub mod object {
    macro_rules! object_name {
        ($name:ident, $value:expr) => {
            #[derive(Debug, Clone, Copy)]
            pub struct $name;

            impl serde::Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str($value)
                }
            }

            impl<'de> serde::Deserialize<'de> for $name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use serde::de::Error;
                    let s: &'de str = serde::Deserialize::deserialize(deserializer)?;
                    if s == $value {
                        Ok($name)
                    } else {
                        Err(D::Error::custom(format!(
                            "Expected {}, found {}",
                            $value, &s
                        )))
                    }
                }
            }
        };
    }

    object_name!(Address, "address");
    object_name!(UsVerification, "us_verification");
    object_name!(UsAutocompletion, "us_autocompletion");
    object_name!(UsZipLookup, "us_zip_lookup");
    object_name!(InternationalVerification, "intl_verification");
    object_name!(Postcard, "postcard");
    object_name!(Letter, "letter");
    object_name!(Check, "check");
    object_name!(BankAccount, "bank_account");
    object_name!(TrackingEvent, "tracking_event");
    object_name!(Event, "event");
    object_name!(EventType, "event_type");
    object_name!(Envelope, "envelope");
    object_name!(List, "list");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_country: Option<String>,
    pub metadata: serde_json::Value,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub deleted: bool,
    object: object::Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAddress {
    pub description: Option<String>,
    pub name: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_country: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct UsVerification {
    pub id: String,
    pub recipient: String,
    pub primary_line: String,
    pub secondary_line: Option<String>,
    pub urbanization: Option<String>,
    pub last_line: String,
    pub deliverability: Deliverability,
    pub components: VerificationComponents,
    pub deliverability_analysis: DeliverabilityAnalysis,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Deliverability {
    Deliverable,
    DeliverableUnnecesaryUnit,
    DeliverableIncorrectUnit,
    DeliverableMissingUnit,
    Undeliverable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationComponents {
    pub primary_number: String,
    #[serde(with = "none_as_empty_string")]
    pub street_predirection: Option<String>,
    pub street_name: String,
    #[serde(with = "none_as_empty_string")]
    pub street_suffix: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub street_postdirection: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub secondary_designator: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub secondary_number: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub pmb_designator: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub pmb_number: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub extra_secondary_designator: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub extra_secondary_number: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    #[serde(with = "none_as_empty_string")]
    pub zip_code_plus_4: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub zip_code_type: Option<ZipCodeType>,
    #[serde(with = "none_as_empty_string")]
    pub delivery_point_barcode: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub address_type: Option<AddressType>,
    #[serde(with = "none_as_empty_string")]
    pub record_type: Option<RecordType>,
    pub default_building_address: bool,
    pub county: String,
    pub county_fips: String,
    pub carrier_route: String,
    pub carrier_route_type: CarrierRouteType,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverabilityAnalysis {
    // None indicates undeliverable address
    dpv_confirmation: Option<DpvConfirmation>,
    #[serde(with = "yn_empty")]
    dpv_cmra: Option<bool>,
    #[serde(with = "yn_empty")]
    dpv_vacant: Option<bool>,
    #[serde(with = "yn_empty")]
    dpv_active: Option<bool>,
    dpv_footnotes: Vec<DpvCode>,
    ews_match: bool,
    #[serde(with = "yn_empty")]
    lacs_indicator: Option<bool>,
    #[serde(with = "none_as_empty_string")]
    lacs_return_code: Option<LacsReturnCode>,
    #[serde(with = "none_as_empty_string")]
    suite_return_code: Option<SuiteReturnCode>,
    object: object::UsVerification,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZipCodeType {
    Standard,
    Military,
    Unique,
    PoBox,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Residential,
    Commercial,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
    Street,
    Highrise,
    Firm,
    PoBox,
    RuralRoute,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CarrierRouteType {
    CityDelivery,
    RuralRoute,
    HighwayContract,
    PoBox,
    GeneralDelivery,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DpvConfirmation {
    // The address is deliverable by the USPS.
    Y,
    // The address is deliverable by removing the provided secondary unit designator. This
    // information may be incorrect or unnecessary.
    S,
    // The address is deliverable to the building's default address but is missing a secondary unit
    // designator and/or number. There is a chance the mail will not reach the intended recipient.
    D,
    // The address is not deliverable according to the USPS, but parts of the address are valid
    // (such as the street and ZIP code).
    N,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DpvCode {
    AA, // Some parts of the address (such as the street and ZIP code) are valid.
    A1, // The address is invalid based on given inputs.
    BB, // The address is deliverable.
    CC, // The address is deliverable by removing the provided secondary unit designator.
    N1, // The address is deliverable but is missing a secondary information (apartment, unit, etc).
    F1, // The address is a deliverable military address.
    G1, // The address is a deliverable General Delivery address. General Delivery is a USPS service which allows individuals without permanent addresses to receive mail.
    U1, // The address is a deliverable unique address. A unique ZIP code is assigned to a single organization (such as a government agency) that receives a large volume of mail.
    M1, // The primary number is missing.
    M3, // The primary number is invalid.
    P1, // PO Box, Rural Route, or Highway Contract box number is missing.
    P3, // PO Box, Rural Route, or Highway Contract box number is invalid.
    R1, // The address matched to a CMRA and private mailbox information is not present.
    R7, // The address matched to a Phantom Carrier Route (carrier_route of R777), which corresponds to physical addresses that are not eligible for delivery.
    RR, // The address matched to a CMRA and private mailbox information is present.
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LacsReturnCode {
    A, // A new address was produced because a match was found in LACSLink.
    #[serde(rename = "92")]
    _92, // A LACSLink record was matched after dropping secondary information.
    #[serde(rename = "14")]
    _14, // A match was found in LACSLink, but could not be converted to a deliverable address.
    #[serde(rename = "00")]
    _00, // A match was not found in LACSLink, and no new address was produced.
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuiteReturnCode {
    A, // A SuiteLink match was found and secondary information was added.
    #[serde(rename = "00")]
    _00, // A SuiteLink match could not be found and no secondary information was added.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsAutocompletion {
    pub id: String,
    pub suggestions: Vec<AutocompleteSuggestion>,
    object: object::UsAutocompletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsZipLookup {
    pub id: String,
    pub zip_code: String,
    #[serde(with = "none_as_empty_string")]
    pub zip_code_type: Option<ZipCodeType>,
    pub cities: Vec<City>,
    object: object::UsZipLookup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
    pub primary_line: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub city: String,
    pub state: String,
    pub county: String,
    pub county_fips: String,
    pub preferred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalVerification {
    pub id: String,
    pub recipient: String,
    pub primary_line: String,
    #[serde(with = "none_as_empty_string")]
    pub secondary_line: Option<String>,
    pub last_line: String,
    pub country: String,
    pub deliverability: Deliverability,
    pub components: InternationalAddressComponents,
    object: object::InternationalVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalAddressComponents {
    pub primary_object: String,
    pub street_name: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Postcard {
    pub id: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub to: Address,
    pub from: Option<Address>,
    pub url: String,
    pub front_template_id: Option<String>,
    pub back_template_id: Option<String>,
    pub front_template_version_id: Option<String>,
    pub back_template_version_id: Option<String>,
    pub carrier: String,
    pub tracking_events: Vec<TrackingEvent>,
    pub thumbnails: Vec<String>,
    pub merge_variables: Option<serde_json::Value>,
    pub size: PostcardSize,
    pub mail_type: MailType,
    pub expected_delivery_date: DateTime<Utc>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: bool,
    object: object::Postcard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Letter {
    pub id: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub to: Address,
    pub from: Option<Address>,
    pub color: bool,
    pub double_sided: bool,
    pub address_placement: LetterAddressPlacement,
    pub return_envelope: bool,
    pub perforated_page: Option<i32>,
    pub custom_envelope: Option<CustomEnvelope>,
    pub extra_service: Option<ExtraService>,
    pub mail_type: MailType,
    pub url: String,
    pub merge_variables: Option<serde_json::Value>,
    pub template_id: Option<String>,
    pub template_version_id: Option<String>,
    pub carrier: String,
    pub tracking_number: Option<String>,
    pub tracking_events: Vec<TrackingEvent>,
    pub thumbnails: Vec<String>,
    pub expected_delivery_date: DateTime<Utc>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: bool,
    object: object::Letter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub id: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub check_number: i32,
    pub memo: Option<String>,
    pub amount: f64, //TODO should we create a custom type for this? Or use a third-party crate (e.g. bigdecimal)?
    pub message: Option<String>,
    pub url: String,
    pub check_bottom_template_id: Option<String>,
    pub attachment_template_id: Option<String>,
    pub check_bottom_template_version_id: Option<String>,
    pub attachment_template_version_id: Option<String>,
    pub to: Address,
    pub from: Address,
    pub bank_account: BankAccount,
    pub carrier: String,
    pub tracking_number: Option<String>,
    pub tracking_events: Vec<TrackingEvent>,
    pub thumbnails: Vec<String>,
    pub merge_variables: Option<serde_json::Value>,
    pub expected_delivery_date: DateTime<Utc>,
    pub mail_type: MailType,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: bool,
    object: object::Check,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
    pub routing_number: String,
    pub account_number: String,
    pub account_type: AccountType,
    pub signatory: String,
    pub signature_url: Option<String>,
    pub bank_name: String,
    pub verified: bool,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub deleted: bool,
    object: object::BankAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEnvelope {
    pub id: String,
    pub url: String,
    object: object::Envelope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    pub id: String,
    pub name: String,
    pub location: Option<String>,
    pub time: DateTime<Utc>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    object: object::TrackingEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub body: Object,
    pub reference_id: String,
    pub event_type: EventType,
    pub date_created: DateTime<Utc>,
    object: object::Event,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventType {
    pub id: EventTypeId,
    pub enabled_for_test: bool,
    pub resource: Resource,
    object: object::EventType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventTypeId {
    //Postcards
    #[serde(rename = "postcard.created")]
    PostcardCreated, //    Occurs when a postcard is successfully created (Lob returns a 200 status code).
    #[serde(rename = "postcard.rendered_pdf")]
    PostcardRenderedPdf, //   Occurs when a postcard's PDF proof is successfully rendered.
    #[serde(rename = "postcard.rendered_thumbnails")]
    PostcardRenderedThumbnails, //    Occurs when a postcard's thumbnails are successfully rendered.
    #[serde(rename = "postcard.deleted")]
    PostcardDeleted, //    Occurs when a postcard is successfully canceled.
    #[serde(rename = "postcard.mailed")]
    PostcardMailed, //     Occurs when a postcard receives a "Mailed" tracking event. Only enabled for certain Print & Mail Editions. Only created in the Live Environment.
    #[serde(rename = "postcard.in_transit")]
    PostcardInTransit, //     Occurs when a postcard receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.in_local_area")]
    PostcardInLocalArea, //  Occurs when a postcard receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.processed_for_delivery")]
    PostcardProcessedForDelivery, //     Occurs when a postcard receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.re-routed")]
    PostcardReRouted, //  Occurs when a postcard receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.returned_to_sender")]
    PostcardReturnedToSender, //     Occurs when a postcard receives a "Returned to Sender" tracking event. Only created in the Live Environment.

    //Letters
    #[serde(rename = "letter.created")]
    LetterCreated, // Occurs when a letter is successfully created (Lob returns a 200 status code).
    #[serde(rename = "letter.rendered_pdf")]
    LetterRenderedPdf, //    Occurs when a letter's PDF proof is successfully rendered.
    #[serde(rename = "letter.rendered_thumbnails")]
    LetterRenderedThumbnails, // Occurs when a letter's thumbnails are successfully rendered.
    #[serde(rename = "letter.deleted")]
    LetterDeleted, // Occurs when a letter is successfully canceled.
    #[serde(rename = "letter.mailed")]
    LetterMailed, //  Occurs when a letter receives a "Mailed" tracking event. Only enabled for certain Print & Mail Editions. Only created in the Live Environment.
    #[serde(rename = "letter.in_transit")]
    LetterInTransit, //  Occurs when a letter receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.in_local_area")]
    LetterInLocalArea, //   Occurs when a letter receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.processed_for_delivery")]
    LetterProcessedForDelivery, //  Occurs when a letter receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.re-routed")]
    LetterReRouted, //   Occurs when a letter receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.returned_to_sender")]
    LetterReturnedToSender, //  Occurs when a letter receives a "Returned to Sender" tracking event. Only created in the Live Environment.

    // Checks
    #[serde(rename = "check.created")]
    CheckCreated, //  Occurs when a check is successfully created (Lob returns a 200 status code).
    #[serde(rename = "check.rendered_pdf")]
    CheckRenderedPdf, // Occurs when a check's PDF proof is successfully rendered.
    #[serde(rename = "check.rendered_thumbnails")]
    CheckRenderedThumbnails, //  Occurs when a check's thumbnails are successfully rendered.
    #[serde(rename = "check.deleted")]
    CheckDeleted, //  Occurs when a check is successfully canceled.
    #[serde(rename = "check.in_transit")]
    CheckInTransit, //   Occurs when a check receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.in_local_area")]
    CheckInLocalArea, //    Occurs when a check receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.processed_for_delivery")]
    CheckProcessedForDelivery, //   Occurs when a check receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.re-routed")]
    CheckReRouted, //    Occurs when a check receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.returned_to_sender")]
    CheckReturnedToSender, //   Occurs when a check receives a "Returned to Sender" tracking event. Only created in the Live Environment.

    // Addresses
    #[serde(rename = "address.created")]
    AddressCreated, // Occurs when an address is successfully created (Lob returns a 200 status code).
    #[serde(rename = "address.deleted")]
    AddressDeleted, // Occurs when an address is successfully deleted.

    // Bank Accounts
    #[serde(rename = "bank_account.created")]
    BankAccountCreated, //  Occurs when a bank account is successfully created (Lob returns a 200 status code).
    #[serde(rename = "bank_account.deleted")]
    BankAccountDeleted, //  Occurs when a bank account is successfully deleted.
    #[serde(rename = "bank_account.verified")]
    BankAccountVerified, // Occurs when a bank account is successfully verified.
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resource {
    Postcards,
    Letters,
    Checks,
    Addresses,
    BankAccounts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Object {
    Address(Address),
    Postcard(Postcard),
    Letter(Letter),
    Check(Check),
    BankAccount(BankAccount),
    Delete(Delete),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PostcardSize {
    #[serde(rename = "4x6")]
    FourBySix,
    #[serde(rename = "6x9")]
    SixByNine,
    #[serde(rename = "6x11")]
    SixByEleven,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MailType {
    UspsFirstClass,
    UspsStandard,
    UpsNextDayAir,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LetterAddressPlacement {
    TopFirstPage,
    InsertBlankPage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtraService {
    Certified,
    CertifiedReturnReceipt,
    Registered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Company,
    Individual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delete {
    pub id: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobError {
    pub message: String,
    pub status_code: i32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAddressesOptions {
    limit: Option<u32>,
    after: Option<String>,
    before: Option<String>,
    include: Option<Vec<ListAddressesIncludeOptions>>,
    metadata: Option<BTreeMap<String, String>>,
    date_created: Option<DateFilter>
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum ListAddressesIncludeOptions {
    TotalCount,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse<T> {
    data: Vec<T>,
    object: object::List,
    next_url: Option<String>,
    previous_url: Option<String>,
    count: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DateFilter {
    pub gt: Option<DateTime<Utc>>,
    pub gte: Option<DateTime<Utc>>,
    pub lt: Option<DateTime<Utc>>,
    pub lte: Option<DateTime<Utc>>,
}

impl fmt::Display for ListAddressesIncludeOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ListAddressesIncludeOptions::TotalCount => write!(f, "total_count"),
        }
    }
}

// {"Y", "N", ""} <=> {Some(true), Some(false), None}
mod yn_empty {
    use serde::de::{Deserialize, Error};

    pub fn serialize<S>(t: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match t {
            Some(true) => serde::Serialize::serialize("Y", serializer),
            Some(false) => serde::Serialize::serialize("N", serializer),
            None => serde::Serialize::serialize("", serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &'de str = Deserialize::deserialize(deserializer)?;
        match s {
            "Y" => Ok(Some(true)),
            "N" => Ok(Some(false)),
            "" => Ok(None),
            other => Err(D::Error::custom(format!(
                "Expected 'Y', 'N', or '', found '{}'",
                other
            ))),
        }
    }
}

mod none_as_empty_string {
    use serde::{
        de::IntoDeserializer,
        {Deserialize, Serialize},
    };

    pub fn serialize<S, T>(t: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: Serialize,
    {
        match t {
            Some(t) => serde::Serialize::serialize(t, serializer),
            None => serde::Serialize::serialize("", serializer),
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let s: &'de str = Deserialize::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            T::deserialize(s.into_deserializer()).map(Some)
        }
    }
}
