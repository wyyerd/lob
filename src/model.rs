use chrono::{DateTime, NaiveDate, Utc};
use std::fmt::Formatter;
use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub metadata: BTreeMap<String, String>,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub deleted: Option<bool>,
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
    pub metadata: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsVerification {
    pub id: String,
    #[serde(with = "none_as_empty_string")]
    pub recipient: Option<String>,
    pub primary_line: String,
    #[serde(with = "none_as_empty_string")]
    pub secondary_line: Option<String>,
    #[serde(with = "none_as_empty_string")]
    pub urbanization: Option<String>,
    pub last_line: String,
    pub deliverability: Deliverability,
    pub components: VerificationComponents,
    pub deliverability_analysis: DeliverabilityAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyAddressOptions {
    pub case: Option<Case>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Case {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressVerificationComponents {
    pub recipient: Option<String>,
    pub primary_line: String,
    pub secondary_line: Option<String>,
    pub urbanization: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AddressVerificationInput {
    Flat(String),
    Components(AddressVerificationComponents),
}

pub trait VerifyAddress {
    fn into_input(self) -> AddressVerificationInput;
}

impl VerifyAddress for AddressVerificationComponents {
    fn into_input(self) -> AddressVerificationInput {
        AddressVerificationInput::Components(self)
    }
}

impl VerifyAddress for String {
    fn into_input(self) -> AddressVerificationInput {
        AddressVerificationInput::Flat(self)
    }
}

impl VerifyAddress for &str {
    fn into_input(self) -> AddressVerificationInput {
        AddressVerificationInput::Flat(self.to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Deliverability {
    Deliverable,
    DeliverableUnnecessaryUnit,
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
    #[serde(with = "none_as_empty_string")]
    pub carrier_route_type: Option<CarrierRouteType>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverabilityAnalysis {
    /// `None` indicates undeliverable address
    #[serde(with = "none_as_empty_string")]
    pub dpv_confirmation: Option<DpvConfirmation>,
    #[serde(with = "yn_empty")]
    pub dpv_cmra: Option<bool>,
    #[serde(with = "yn_empty")]
    pub dpv_vacant: Option<bool>,
    #[serde(with = "yn_empty")]
    pub dpv_active: Option<bool>,
    pub dpv_footnotes: Vec<DpvCode>,
    pub ews_match: bool,
    #[serde(with = "yn_empty")]
    pub lacs_indicator: Option<bool>,
    #[serde(with = "none_as_empty_string")]
    pub lacs_return_code: Option<LacsReturnCode>,
    #[serde(with = "none_as_empty_string")]
    pub suite_return_code: Option<SuiteReturnCode>,
    //    object: object::UsVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZipCodeType {
    Standard,
    Military,
    Unique,
    PoBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AddressType {
    Residential,
    Commercial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordType {
    Street,
    Highrise,
    Firm,
    PoBox,
    RuralRoute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CarrierRouteType {
    CityDelivery,
    RuralRoute,
    HighwayContract,
    PoBox,
    GeneralDelivery,
    Contract, // `Contract` doesn't appear in their documentation, but has been returned
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpvConfirmation {
    /// The address is deliverable by the USPS.
    Y,
    /// The address is deliverable by removing the provided secondary unit designator. This
    /// information may be incorrect or unnecessary.
    S,
    /// The address is deliverable to the building's default address but is missing a secondary unit
    /// designator and/or number. There is a chance the mail will not reach the intended recipient.
    D,
    /// The address is not deliverable according to the USPS, but parts of the address are valid
    /// (such as the street and ZIP code).
    N,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DpvCode {
    /// Some parts of the address (such as the street and ZIP code) are valid.
    AA,
    /// The address is invalid based on given inputs.
    A1,
    /// The address is deliverable.
    BB,
    /// The address is deliverable by removing the provided secondary unit designator.
    CC,
    /// The address is deliverable but is missing a secondary information (apartment, unit, etc).
    N1,
    /// The address is a deliverable military address.
    F1,
    /// The address is a deliverable General Delivery address. General Delivery is a USPS service which allows individuals without permanent addresses to receive mail.
    G1,
    /// The address is a deliverable unique address. A unique ZIP code is assigned to a single organization (such as a government agency) that receives a large volume of mail.
    U1,
    /// The primary number is missing.
    M1,
    /// The primary number is invalid.
    M3,
    /// PO Box, Rural Route, or Highway Contract box number is missing.
    P1,
    /// PO Box, Rural Route, or Highway Contract box number is invalid.
    P3,
    /// The address matched to a CMRA and private mailbox information is not present.
    R1,
    /// The address matched to a Phantom Carrier Route (carrier_route of R777), which corresponds to physical addresses that are not eligible for delivery.
    R7,
    /// The address matched to a CMRA and private mailbox information is present.
    RR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LacsReturnCode {
    /// A new address was produced because a match was found in LACSLink.
    A,
    #[serde(rename = "92")]
    /// A LACSLink record was matched after dropping secondary information.
    _92,
    #[serde(rename = "14")]
    /// A match was found in LACSLink, but could not be converted to a deliverable address.
    _14,
    #[serde(rename = "00")]
    /// A match was not found in LACSLink, and no new address was produced.
    _00,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default)]
pub struct AutocompleteAddressOptions {
    pub city: Option<String>,
    pub state: Option<String>,
    pub geo_ip_sort: Option<IpAddr>,
    // Requires explicit permission from Lob to use
    pub only_valid_addresses: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct AutocompleteAddressOptionsQuery {
    pub address_prefix: String,
    pub city: Option<String>,
    pub state: Option<String>,
    pub geo_ip_sort: Option<bool>,
}

impl AutocompleteAddressOptionsQuery {
    pub fn new<S: Into<String>>(
        address_prefix: S,
        options: Option<AutocompleteAddressOptions>,
    ) -> Self {
        let options = options.unwrap_or_default();
        AutocompleteAddressOptionsQuery {
            address_prefix: address_prefix.into(),
            city: options.city,
            state: options.state,
            geo_ip_sort: options.geo_ip_sort.map(|_| true),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct UsZipLookupBody {
    pub zip_code: String,
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
pub struct InternationalVerificationInput {
    pub recipient: Option<String>,
    pub primary_line: String,
    pub secondary_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    /// Must be a 2 letter country short-name code (ISO 3166). Does not accept US, AS, PR, FM, GU,
    /// MH, MP, PW, or VI. For these addresses, please use the US verification API. Also does not
    /// accept PS, which is not currently supported.
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternationalAddressComponents {
    pub primary_object: Option<String>,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Postcard {
    pub id: String,
    pub description: Option<String>,
    pub metadata: BTreeMap<String, String>,
    pub to: Address,
    pub from: Option<Address>,
    pub url: String,
    pub front_template_id: Option<String>,
    pub back_template_id: Option<String>,
    pub front_template_version_id: Option<String>,
    pub back_template_version_id: Option<String>,
    pub carrier: String,
    pub tracking_events: Vec<TrackingEvent>,
    pub thumbnails: Vec<Thumbnails>,
    pub merge_variables: Option<BTreeMap<String, String>>,
    pub size: PostcardSize,
    pub mail_type: MailType,
    pub expected_delivery_date: NaiveDate,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: Option<bool>,
    object: object::Postcard,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewPostcard {
    pub description: Option<String>,
    pub to: SendAddress,
    pub from: Option<SendAddress>,
    #[serde(skip_serializing_if = "FileInput::is_file")]
    pub front: FileInput,
    #[serde(skip_serializing_if = "FileInput::is_file")]
    pub back: FileInput,
    pub merge_variables: Option<BTreeMap<String, String>>,
    pub size: Option<PostcardSize>,
    pub mail_type: Option<MailType>,
    pub send_date: Option<DateTime<Utc>>,
    pub metadata: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListPostcardOptions {
    /// An integer that designates how many results to return. Defaults to 10 and must be no more than 100.
    pub limit: Option<i32>,
    /// A reference to a list entry used for paginating to the previous set of entries. This field is pre-populated in the previous_url field in the return response.
    pub after: Option<String>,
    /// A reference to a list entry used for paginating to the next set of entries. This field is pre-populated in the next_url field in the return response.
    pub before: Option<String>,
    /// Request that the response include the total count by specifying include[]=total_count.
    pub include: Option<Vec<ListIncludeOptions>>,
    /// Filter by metadata key-value pair, e.g. metadata[customer_id]=987654.
    pub metadata: Option<BTreeMap<String, String>>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub date_created: Option<DateFilter>,
    /// The postcard sizes to be returned. Must be a non-empty string array of valid sizes. Acceptable values are 4x6, 6x9, and 6x11.
    pub size: Option<PostcardSize>,
    //Set scheduled to true to only return orders (past or future) where send_date is greater than date_created. Set scheduled to false to only return orders where send_date is equal to date_created.
    pub scheduled: Option<bool>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub send_date: Option<DateFilter>,
    pub mail_type: Option<MailType>,
    /// Sorts postcards in a desired order. sort_by accepts an object with the key being either date_created or send_date and the value being either asc or desc.
    pub sort_by: Option<SortBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Thumbnails {
    pub large: String,
    pub medium: String,
    pub small: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    DateCreated(Order),
    SendDate(Order),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SendAddress {
    AddressId(String),
    Components(SendAddressComponents),
}

impl Into<SendAddress> for String {
    fn into(self) -> SendAddress {
        SendAddress::AddressId(self)
    }
}

impl Into<SendAddress> for &str {
    fn into(self) -> SendAddress {
        SendAddress::AddressId(self.to_owned())
    }
}

impl Into<SendAddress> for SendAddressComponents {
    fn into(self) -> SendAddress {
        SendAddress::Components(self)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SendAddressComponents {
    pub name: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_city: String,
    pub address_state: String,
    pub address_zip: String,
    pub address_country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Letter {
    pub id: String,
    pub description: Option<String>,
    pub metadata: BTreeMap<String, String>,
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
    pub merge_variables: Option<BTreeMap<String, String>>,
    pub template_id: Option<String>,
    pub template_version_id: Option<String>,
    pub carrier: String,
    pub tracking_number: Option<String>,
    pub tracking_events: Vec<TrackingEvent>,
    pub thumbnails: Vec<Thumbnails>,
    pub expected_delivery_date: NaiveDate,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: Option<bool>,
    object: object::Letter,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewLetter {
    pub description: Option<String>,
    pub to: SendAddress,
    pub from: SendAddress,
    pub color: bool,
    #[serde(skip_serializing_if = "FileInput::is_file")]
    pub file: FileInput,
    pub merge_variables: Option<BTreeMap<String, String>>,
    pub double_sided: Option<bool>,
    pub address_placement: Option<LetterAddressPlacement>,
    pub return_envelope: Option<bool>,
    pub custom_envelope: Option<String>,
    pub mail_type: Option<MailType>,
    pub extra_service: Option<ExtraService>,
    pub send_date: Option<DateTime<Utc>>,
    pub perforated_page: Option<u32>,
    pub metadata: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListLetterOptions {
    /// An integer that designates how many results to return. Defaults to 10 and must be no more than 100.
    pub limit: Option<i32>,
    /// A reference to a list entry used for paginating to the previous set of entries. This field is pre-populated in the previous_url field in the return response.
    pub after: Option<String>,
    /// A reference to a list entry used for paginating to the next set of entries. This field is pre-populated in the next_url field in the return response.
    pub before: Option<String>,
    /// Request that the response include the total count by specifying include[]=total_count.
    pub include: Option<Vec<ListIncludeOptions>>,
    /// Filter by metadata key-value pair, e.g. metadata[customer_id]=987654.
    pub metadata: Option<BTreeMap<String, String>>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub date_created: Option<DateFilter>,
    //Set scheduled to true to only return orders (past or future) where send_date is greater than date_created. Set scheduled to false to only return orders where send_date is equal to date_created.
    pub scheduled: Option<bool>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub send_date: Option<DateFilter>,
    pub mail_type: Option<MailType>,
    pub color: Option<bool>,
    /// Sorts postcards in a desired order. sort_by accepts an object with the key being either date_created or send_date and the value being either asc or desc.
    pub sort_by: Option<SortBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Check {
    pub id: String,
    pub description: Option<String>,
    pub metadata: BTreeMap<String, String>,
    pub check_number: i32,
    pub memo: Option<String>,
    pub amount: CheckAmount, //TODO should we use a third-party crate (e.g. bigdecimal)?
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
    pub thumbnails: Vec<Thumbnails>,
    pub merge_variables: Option<BTreeMap<String, String>>,
    pub expected_delivery_date: DateTime<Utc>,
    pub mail_type: MailType,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub send_date: DateTime<Utc>,
    pub deleted: Option<bool>,
    object: object::Check,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewCheck {
    pub description: Option<String>,
    pub to: SendAddress,
    pub from: SendAddress,
    pub bank_account: String,
    pub amount: CheckAmount, //TODO should we create a custom type for this? Or use a third-party crate (e.g. bigdecimal)?
    pub memo: Option<String>,
    pub check_number: Option<i32>,
    /// must be URL or File
    #[serde(skip_serializing_if = "FileInput::is_maybe_file")]
    pub logo: Option<FileInput>,
    /// Either message or check_bottom, choose one. Max of 400 characters to be included at the bottom of the check page.
    pub message: Option<String>,
    /// Either message or check_bottom, choose one. The artwork to use on the bottom of the check page.
    /// Accepts an HTML string of under 10,000 characters, the ID of a saved HTML template, or a remote
    /// URL or a local upload of an HTML, PDF, PNG, or JPG file. HTML files passed as remote URLs or
    /// local file uploads have no character limit. HTML merge variables should not include delimiting
    /// whitespace. PDF, PNG, and JPGs must be sized at 8.5"x11" at 300 DPI, while supplied HTML will
    /// be rendered and trimmed to fit on a 8.5"x11" page. The check bottom will always be printed in
    /// black & white. You must follow template at https://s3-us-west-2.amazonaws.com/public.lob.com/assets/templates/check_bottom_template.pdf.
    /// See https://lob.com/docs#html-examples for HTML examples.
    #[serde(skip_serializing_if = "FileInput::is_maybe_file")]
    pub check_bottom: Option<FileInput>,
    #[serde(skip_serializing_if = "FileInput::is_maybe_file")]
    pub attachment: Option<FileInput>,
    /// Must be UspsFirstClass or UpsNextDayAir
    pub mail_type: Option<MailType>,
    pub send_date: Option<NaiveDate>,
    pub metadata: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct ListCheckOptions {
    /// An integer that designates how many results to return. Defaults to 10 and must be no more than 100.
    pub limit: Option<i32>,
    /// A reference to a list entry used for paginating to the previous set of entries. This field is pre-populated in the previous_url field in the return response.
    pub after: Option<String>,
    /// A reference to a list entry used for paginating to the next set of entries. This field is pre-populated in the next_url field in the return response.
    pub before: Option<String>,
    /// Request that the response include the total count by specifying include[]=total_count.
    pub include: Option<Vec<ListIncludeOptions>>,
    /// Filter by metadata key-value pair, e.g. metadata[customer_id]=987654.
    pub metadata: Option<BTreeMap<String, String>>,
    pub mail_type: Option<MailType>,
    /// Set scheduled to true to only return orders (past or future) where send_date is greater than date_created. Set scheduled to false to only return orders where send_date is equal to date_created.
    pub scheduled: Option<bool>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub date_created: Option<DateFilter>,
    /// Filter by ISO-8601 date or datetime, e.g. { gt: '2012-01-01', lt: '2012-01-31T12:34:56Z' } where gt is ›, lt is ‹, gte is ≥, and lte is ≤.
    pub send_date: Option<DateFilter>,
    /// Sorts postcards in a desired order. sort_by accepts an object with the key being either date_created or send_date and the value being either asc or desc.
    pub sort_by: Option<SortBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankAccount {
    pub id: String,
    pub description: Option<String>,
    pub metadata: BTreeMap<String, String>,
    pub routing_number: String,
    pub account_number: String,
    pub account_type: AccountType,
    pub signatory: String,
    pub signature_url: Option<String>,
    pub bank_name: String,
    pub verified: bool,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub deleted: Option<bool>,
    object: object::BankAccount,
}

#[derive(Debug, Clone, Serialize)]
pub struct NewBankAccount {
    pub description: Option<String>,
    pub routing_number: String,
    pub account_number: String,
    pub account_type: AccountType,
    pub signatory: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListBankAccountOptions {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub include: Option<Vec<ListIncludeOptions>>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub date_created: Option<DateFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CustomEnvelope {
    pub id: String,
    pub url: String,
    object: object::Envelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    // Postcards
    /// Occurs when a postcard is successfully created (Lob returns a 200 status code).
    #[serde(rename = "postcard.created")]
    PostcardCreated,
    /// Occurs when a postcard's PDF proof is successfully rendered.
    #[serde(rename = "postcard.rendered_pdf")]
    PostcardRenderedPdf,
    /// Occurs when a postcard's thumbnails are successfully rendered.
    #[serde(rename = "postcard.rendered_thumbnails")]
    PostcardRenderedThumbnails,
    /// Occurs when a postcard is successfully canceled.
    #[serde(rename = "postcard.deleted")]
    PostcardDeleted,
    /// Occurs when a postcard receives a "Mailed" tracking event. Only enabled for certain Print & Mail Editions. Only created in the Live Environment.
    #[serde(rename = "postcard.mailed")]
    PostcardMailed,
    /// Occurs when a postcard receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.in_transit")]
    PostcardInTransit,
    /// Occurs when a postcard receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.in_local_area")]
    PostcardInLocalArea,
    /// Occurs when a postcard receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.processed_for_delivery")]
    PostcardProcessedForDelivery,
    /// Occurs when a postcard receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.re-routed")]
    PostcardReRouted,
    /// Occurs when a postcard receives a "Returned to Sender" tracking event. Only created in the Live Environment.
    #[serde(rename = "postcard.returned_to_sender")]
    PostcardReturnedToSender,

    // Letters
    /// Occurs when a letter is successfully created (Lob returns a 200 status code).
    #[serde(rename = "letter.created")]
    LetterCreated,
    /// Occurs when a letter's PDF proof is successfully rendered.
    #[serde(rename = "letter.rendered_pdf")]
    LetterRenderedPdf,
    /// Occurs when a letter's thumbnails are successfully rendered.
    #[serde(rename = "letter.rendered_thumbnails")]
    LetterRenderedThumbnails,
    /// Occurs when a letter is successfully canceled.
    #[serde(rename = "letter.deleted")]
    LetterDeleted,
    /// Occurs when a letter receives a "Mailed" tracking event. Only enabled for certain Print & Mail Editions. Only created in the Live Environment.
    #[serde(rename = "letter.mailed")]
    LetterMailed,
    /// Occurs when a letter receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.in_transit")]
    LetterInTransit,
    /// Occurs when a letter receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.in_local_area")]
    LetterInLocalArea,
    /// Occurs when a letter receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.processed_for_delivery")]
    LetterProcessedForDelivery,
    /// Occurs when a letter receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.re-routed")]
    LetterReRouted,
    /// Occurs when a letter receives a "Returned to Sender" tracking event. Only created in the Live Environment.
    #[serde(rename = "letter.returned_to_sender")]
    LetterReturnedToSender,

    // Checks
    /// Occurs when a check is successfully created (Lob returns a 200 status code).
    #[serde(rename = "check.created")]
    CheckCreated,
    /// Occurs when a check's PDF proof is successfully rendered.
    #[serde(rename = "check.rendered_pdf")]
    CheckRenderedPdf,
    /// Occurs when a check's thumbnails are successfully rendered.
    #[serde(rename = "check.rendered_thumbnails")]
    CheckRenderedThumbnails,
    /// Occurs when a check is successfully canceled.
    #[serde(rename = "check.deleted")]
    CheckDeleted,
    /// Occurs when a check receives an "In Transit" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.in_transit")]
    CheckInTransit,
    /// Occurs when a check receives an "In Local Area" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.in_local_area")]
    CheckInLocalArea,
    /// Occurs when a check receives a "Processed for Delivery" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.processed_for_delivery")]
    CheckProcessedForDelivery,
    /// Occurs when a check receives a "Re-Routed" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.re-routed")]
    CheckReRouted,
    /// Occurs when a check receives a "Returned to Sender" tracking event. Only created in the Live Environment.
    #[serde(rename = "check.returned_to_sender")]
    CheckReturnedToSender,

    // Addresses
    /// Occurs when an address is successfully created (Lob returns a 200 status code).
    #[serde(rename = "address.created")]
    AddressCreated,
    /// Occurs when an address is successfully deleted.
    #[serde(rename = "address.deleted")]
    AddressDeleted,

    // Bank Accounts
    /// Occurs when a bank account is successfully created (Lob returns a 200 status code).
    #[serde(rename = "bank_account.created")]
    BankAccountCreated,
    /// Occurs when a bank account is successfully deleted.
    #[serde(rename = "bank_account.deleted")]
    BankAccountDeleted,
    /// Occurs when a bank account is successfully verified.
    #[serde(rename = "bank_account.verified")]
    BankAccountVerified,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PostcardSize {
    #[serde(rename = "4x6")]
    FourBySix,
    #[serde(rename = "6x9")]
    SixByNine,
    #[serde(rename = "6x11")]
    SixByEleven,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MailType {
    UspsFirstClass,
    UspsStandard,
    UpsNextDayAir,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LetterAddressPlacement {
    TopFirstPage,
    InsertBlankPage,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExtraService {
    Certified,
    CertifiedReturnReceipt,
    Registered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
pub struct LobErrorResponse {
    pub error: LobError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobError {
    pub message: String,
    pub status_code: i32,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListAddressesOptions {
    pub limit: Option<u32>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub include: Option<Vec<ListIncludeOptions>>,
    pub metadata: Option<BTreeMap<String, String>>,
    pub date_created: Option<DateFilter>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ListIncludeOptions {
    TotalCount,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub object: object::List,
    pub next_url: Option<String>,
    pub previous_url: Option<String>,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DateFilter {
    pub gt: Option<DateTime<Utc>>,
    pub gte: Option<DateTime<Utc>>,
    pub lt: Option<DateTime<Utc>>,
    pub lte: Option<DateTime<Utc>>,
}

impl fmt::Display for ListIncludeOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ListIncludeOptions::TotalCount => write!(f, "total_count"),
        }
    }
}

// TODO should this should handle files via AsyncRead or w/e?
#[derive(Debug, Clone)]
pub enum FileInput {
    TemplateId(String),
    Url(String),
    Html(String),
    File { filename: String, data: Vec<u8> },
}

impl FileInput {
    pub fn is_file(&self) -> bool {
        match self {
            FileInput::File { .. } => true,
            _ => false,
        }
    }

    pub fn is_maybe_file(file: &Option<FileInput>) -> bool {
        file.as_ref().map(|f| f.is_file()).unwrap_or(false)
    }

    pub fn is_url(&self) -> bool {
        match self {
            FileInput::Url { .. } => true,
            _ => false,
        }
    }
}

// TODO determine whether to instead integrate with an existing decimal crate
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CheckAmount(u64);

impl CheckAmount {
    pub fn new(dollars: u64, cents: u64) -> CheckAmount {
        CheckAmount(dollars * 100 + cents)
    }

    pub fn cents(cents: u64) -> CheckAmount {
        CheckAmount(cents)
    }

    pub fn to_dollars_and_cents(&self) -> (u64, u64) {
        (self.0 / 100, self.0 % 100)
    }

    pub fn to_cents(&self) -> u64 {
        self.0
    }
}

impl From<f64> for CheckAmount {
    fn from(f: f64) -> Self {
        CheckAmount((f * 100.0) as u64)
    }
}

#[derive(Debug, Clone)]
pub struct ParseMoneyError(String);
impl std::error::Error for ParseMoneyError {}
impl fmt::Display for ParseMoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for CheckAmount {
    type Err = ParseMoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(".");
        let dollars = split
            .next()
            .and_then(|d| u64::from_str(d).ok())
            .ok_or_else(|| ParseMoneyError(format!("Unable to parse {} as money", s)))?;
        let cents = split
            .next()
            .map(|c| {
                u64::from_str(c)
                    .map_err(|_| ParseMoneyError(format!("Unable to parse {} as money", s)))
            })
            .transpose()?
            .unwrap_or(0);
        Ok(CheckAmount::cents(dollars * 100 + cents))
    }
}

impl fmt::Display for CheckAmount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (dollars, cents) = self.to_dollars_and_cents();
        write!(f, "{}.{}", dollars, cents)
    }
}

impl Serialize for CheckAmount {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        // TODO is there a better way to do this without the allocation?
        let n: serde_json::Number = self.to_string().parse().unwrap();
        n.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CheckAmount {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let n = serde_json::Number::deserialize(deserializer)?;
        n.to_string()
            .parse()
            .map_err(|e: ParseMoneyError| D::Error::custom(e.to_string()))
    }
}

impl Serialize for FileInput {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            FileInput::TemplateId(s) | FileInput::Url(s) | FileInput::Html(s) => {
                String::serialize(s, serializer)
            }
            FileInput::File { .. } => {
                return Err(S::Error::custom(
                    "BUG! field must be skipped if variant is File",
                ))
            }
        }
    }
}

pub mod object {
    macro_rules! object_name {
        ($name:ident, $value:expr) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
