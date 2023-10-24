mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddBounceBody {
        ///Valid email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///Error code (optional, default: 550)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        ///Timestamp of a bounce event in [RFC2822](https://documentation.mailgun.com/en/latest/api-intro.html#date-format) format (optional, default: current time)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        ///Error description (optional, default: empty string)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
    }

    impl From<&AddBounceBody> for AddBounceBody {
        fn from(value: &AddBounceBody) -> Self {
            value.clone()
        }
    }

    impl AddBounceBody {
        pub fn builder() -> builder::AddBounceBody {
            builder::AddBounceBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddBounceResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&AddBounceResponse> for AddBounceResponse {
        fn from(value: &AddBounceResponse) -> Self {
            value.clone()
        }
    }

    impl AddBounceResponse {
        pub fn builder() -> builder::AddBounceResponse {
            builder::AddBounceResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddBouncesRequest(pub Vec<BounceMail>);
    impl std::ops::Deref for AddBouncesRequest {
        type Target = Vec<BounceMail>;
        fn deref(&self) -> &Vec<BounceMail> {
            &self.0
        }
    }

    impl From<AddBouncesRequest> for Vec<BounceMail> {
        fn from(value: AddBouncesRequest) -> Self {
            value.0
        }
    }

    impl From<&AddBouncesRequest> for AddBouncesRequest {
        fn from(value: &AddBouncesRequest) -> Self {
            value.clone()
        }
    }

    impl From<Vec<BounceMail>> for AddBouncesRequest {
        fn from(value: Vec<BounceMail>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddComplaintBody {
        ///Valid email address
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///Timestamp of a complaint event in RFC2822 format (optional, default:
        /// current time)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
    }

    impl From<&AddComplaintBody> for AddComplaintBody {
        fn from(value: &AddComplaintBody) -> Self {
            value.clone()
        }
    }

    impl AddComplaintBody {
        pub fn builder() -> builder::AddComplaintBody {
            builder::AddComplaintBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddListMemberBody {
        ///Valid email address specification, e.g. Alice <alice@example.com> or
        /// just alice@example.com
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///Optional member name
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///yes to add as subscribed (default), no as unsubscribed
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscribed: Option<String>,
        ///yes to update member if present, no to
        ///raise error in case of a duplicate member (default)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upsert: Option<String>,
        ///JSON-encoded dictionary string with arbitrary parameters, e.g.
        /// {"gender":"female","age":27}

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vars: Option<String>,
    }

    impl From<&AddListMemberBody> for AddListMemberBody {
        fn from(value: &AddListMemberBody) -> Self {
            value.clone()
        }
    }

    impl AddListMemberBody {
        pub fn builder() -> builder::AddListMemberBody {
            builder::AddListMemberBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddListMembersBody {
        ///JSON-encoded array. Elements can be either addresses, e.g.
        /// ["bob@example.com", "alice@example.com"], or JSON objects, e.g.
        /// [{"address": "bob@example.com", "name": "Bob", "subscribed": false},
        /// {"address": "alice@example.com", "name": "Alice"}] . Custom
        /// variables can be provided, see examples.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub members: Option<String>,
        ///yes to update existing members, no (default) to ignore duplicates
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub upsert: Option<String>,
    }

    impl From<&AddListMembersBody> for AddListMembersBody {
        fn from(value: &AddListMembersBody) -> Self {
            value.clone()
        }
    }

    impl AddListMembersBody {
        pub fn builder() -> builder::AddListMembersBody {
            builder::AddListMembersBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AssignIpToDomainBody {
        ///IP address that should be assigned to the domain pool.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ip: Option<String>,
    }

    impl From<&AssignIpToDomainBody> for AssignIpToDomainBody {
        fn from(value: &AssignIpToDomainBody) -> Self {
            value.clone()
        }
    }

    impl AssignIpToDomainBody {
        pub fn builder() -> builder::AssignIpToDomainBody {
            builder::AssignIpToDomainBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Attachment {
        ///The content type of the attachment.
        #[serde(
            rename = "content-type",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub content_type: Option<String>,
        ///The name of the attachment.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Indicates the size of the attachment in bytes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
        ///Contains the URL where the attachment can be found. This does not
        /// support DELETE.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    impl From<&Attachment> for Attachment {
        fn from(value: &Attachment) -> Self {
            value.clone()
        }
    }

    impl Attachment {
        pub fn builder() -> builder::Attachment {
            builder::Attachment::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct BounceMail {
        pub address: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
    }

    impl From<&BounceMail> for BounceMail {
        fn from(value: &BounceMail) -> Self {
            value.clone()
        }
    }

    impl BounceMail {
        pub fn builder() -> builder::BounceMail {
            builder::BounceMail::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Complaint {
        pub address: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    }

    impl From<&Complaint> for Complaint {
        fn from(value: &Complaint) -> Self {
            value.clone()
        }
    }

    impl Complaint {
        pub fn builder() -> builder::Complaint {
            builder::Complaint::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComplaintRequest(pub Vec<Complaint>);
    impl std::ops::Deref for ComplaintRequest {
        type Target = Vec<Complaint>;
        fn deref(&self) -> &Vec<Complaint> {
            &self.0
        }
    }

    impl From<ComplaintRequest> for Vec<Complaint> {
        fn from(value: ComplaintRequest) -> Self {
            value.0
        }
    }

    impl From<&ComplaintRequest> for ComplaintRequest {
        fn from(value: &ComplaintRequest) -> Self {
            value.clone()
        }
    }

    impl From<Vec<Complaint>> for ComplaintRequest {
        fn from(value: Vec<Complaint>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConnectionUpdate {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub connection: Option<ConnectionUpdateConnection>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&ConnectionUpdate> for ConnectionUpdate {
        fn from(value: &ConnectionUpdate) -> Self {
            value.clone()
        }
    }

    impl ConnectionUpdate {
        pub fn builder() -> builder::ConnectionUpdate {
            builder::ConnectionUpdate::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConnectionUpdateConnection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_tls: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_verification: Option<bool>,
    }

    impl From<&ConnectionUpdateConnection> for ConnectionUpdateConnection {
        fn from(value: &ConnectionUpdateConnection) -> Self {
            value.clone()
        }
    }

    impl ConnectionUpdateConnection {
        pub fn builder() -> builder::ConnectionUpdateConnection {
            builder::ConnectionUpdateConnection::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateDomainBody {
        ///true or false
        ///
        ///If set to true, the domain will be the DKIM authority for itself
        /// even if the root domain is registered on the same mailgun account
        ///
        ///If set to false, the domain will have the same DKIM authority as the
        /// root domain registered on the same mailgun account
        ///
        ///The default is false.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub force_dkim_authority: Option<String>,
        ///Name of the domain (ex. domain.com)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Password for SMTP authentication
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub smtp_password: Option<String>,
        ///disabled, block, or tag
        ///
        ///If disabled, no spam filtering will occur for inbound messages.
        ///
        ///If block, inbound spam messages will not be delivered.
        ///
        ///If tag, inbound messages will be tagged with a spam header. See Spam
        /// Filter.
        ///
        ///The default is disabled.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spam_action: Option<String>,
        ///true or false Determines whether the domain will accept email for
        /// sub-domains.
        ///
        ///The default is false.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wildcard: Option<String>,
    }

    impl From<&CreateDomainBody> for CreateDomainBody {
        fn from(value: &CreateDomainBody) -> Self {
            value.clone()
        }
    }

    impl CreateDomainBody {
        pub fn builder() -> builder::CreateDomainBody {
            builder::CreateDomainBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateListBody {
        ///List access level, one of: readonly (default), members, everyone
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub access_level: Option<String>,
        ///A valid email address for the mailing list, e.g.
        /// developers@mailgun.net, or Developers <devs@mg.net>
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///A description (optional)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///Mailing list name, e.g. Developers (optional)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&CreateListBody> for CreateListBody {
        fn from(value: &CreateListBody) -> Self {
            value.clone()
        }
    }

    impl CreateListBody {
        pub fn builder() -> builder::CreateListBody {
            builder::CreateListBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateRouteBody {
        ///Route action. This action is executed when the expression evaluates
        /// to True. Example: `forward("alice@example.com")` You can pass
        /// multiple `action` parameters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,
        ///An arbitrary string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///A filter expression like `match_recipient('.*@gmail.com')`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expression: Option<String>,
        ///Integer: smaller number indicates higher priority. Higher priority
        /// routes are handled first. Defaults to 0.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,
    }

    impl From<&CreateRouteBody> for CreateRouteBody {
        fn from(value: &CreateRouteBody) -> Self {
            value.clone()
        }
    }

    impl CreateRouteBody {
        pub fn builder() -> builder::CreateRouteBody {
            builder::CreateRouteBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateSmtpCredentialsBody {
        ///The user name, for example bob.bar
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub login: Option<String>,
        ///A password for the SMTP credentials. (Length Min 5, Max 32)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
    }

    impl From<&CreateSmtpCredentialsBody> for CreateSmtpCredentialsBody {
        fn from(value: &CreateSmtpCredentialsBody) -> Self {
            value.clone()
        }
    }

    impl CreateSmtpCredentialsBody {
        pub fn builder() -> builder::CreateSmtpCredentialsBody {
            builder::CreateSmtpCredentialsBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CreateWebhookBody {
        ///Name of the domain
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,
        ///Name of the webhook
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        ///URL for the webhook event. May be repeated up to 3 times.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
    }

    impl From<&CreateWebhookBody> for CreateWebhookBody {
        fn from(value: &CreateWebhookBody) -> Self {
            value.clone()
        }
    }

    impl CreateWebhookBody {
        pub fn builder() -> builder::CreateWebhookBody {
            builder::CreateWebhookBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Credential {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub login: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mailbox: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size_bytes: Option<i64>,
    }

    impl From<&Credential> for Credential {
        fn from(value: &Credential) -> Self {
            value.clone()
        }
    }

    impl Credential {
        pub fn builder() -> builder::Credential {
            builder::Credential::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteCredentialsResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&DeleteCredentialsResponse> for DeleteCredentialsResponse {
        fn from(value: &DeleteCredentialsResponse) -> Self {
            value.clone()
        }
    }

    impl DeleteCredentialsResponse {
        pub fn builder() -> builder::DeleteCredentialsResponse {
            builder::DeleteCredentialsResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteListMember {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub member: Option<Member>,
        ///A message indicating the result of the delete operation
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&DeleteListMember> for DeleteListMember {
        fn from(value: &DeleteListMember) -> Self {
            value.clone()
        }
    }

    impl DeleteListMember {
        pub fn builder() -> builder::DeleteListMember {
            builder::DeleteListMember::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DeleteTag {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&DeleteTag> for DeleteTag {
        fn from(value: &DeleteTag) -> Self {
            value.clone()
        }
    }

    impl DeleteTag {
        pub fn builder() -> builder::DeleteTag {
            builder::DeleteTag::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Domain {
        ///The date and time when the domain was created.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        ///The unique identifier for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        ///Indicates whether the domain is disabled.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub is_disabled: Option<bool>,
        ///The name of the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Indicates whether TLS is required for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_tls: Option<bool>,
        ///Indicates whether verification is skipped for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_verification: Option<bool>,
        ///The SMTP login for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub smtp_login: Option<String>,
        ///The action taken when spam is detected for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub spam_action: Option<String>,
        ///The current state of the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        ///The type of the domain.
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        ///The web prefix for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_prefix: Option<String>,
        ///The web scheme for the domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub web_scheme: Option<String>,
        ///Indicates whether the domain is a wildcard domain.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wildcard: Option<bool>,
    }

    impl From<&Domain> for Domain {
        fn from(value: &Domain) -> Self {
            value.clone()
        }
    }

    impl Domain {
        pub fn builder() -> builder::Domain {
            builder::Domain::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DomainResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<Domain>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub receiving_dns_records: Vec<ReceivingDnsRecord>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub sending_dns_records: Vec<SendingDnsRecord>,
    }

    impl From<&DomainResponse> for DomainResponse {
        fn from(value: &DomainResponse) -> Self {
            value.clone()
        }
    }

    impl DomainResponse {
        pub fn builder() -> builder::DomainResponse {
            builder::DomainResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetConnectionResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub connection: Option<GetConnectionResponseConnection>,
    }

    impl From<&GetConnectionResponse> for GetConnectionResponse {
        fn from(value: &GetConnectionResponse) -> Self {
            value.clone()
        }
    }

    impl GetConnectionResponse {
        pub fn builder() -> builder::GetConnectionResponse {
            builder::GetConnectionResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetConnectionResponseConnection {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_tls: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_verification: Option<bool>,
    }

    impl From<&GetConnectionResponseConnection> for GetConnectionResponseConnection {
        fn from(value: &GetConnectionResponseConnection) -> Self {
            value.clone()
        }
    }

    impl GetConnectionResponseConnection {
        pub fn builder() -> builder::GetConnectionResponseConnection {
            builder::GetConnectionResponseConnection::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetCountryResponse {
        ///A hashmap of provider domains and their respective event counts.
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub provider: std::collections::HashMap<String, ProviderCountryEvents>,
        ///The tag for which the events are being retrieved.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&GetCountryResponse> for GetCountryResponse {
        fn from(value: &GetCountryResponse) -> Self {
            value.clone()
        }
    }

    impl GetCountryResponse {
        pub fn builder() -> builder::GetCountryResponse {
            builder::GetCountryResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetCredentialsResponse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub items: Vec<Credential>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total_count: Option<f64>,
    }

    impl From<&GetCredentialsResponse> for GetCredentialsResponse {
        fn from(value: &GetCredentialsResponse) -> Self {
            value.clone()
        }
    }

    impl GetCredentialsResponse {
        pub fn builder() -> builder::GetCredentialsResponse {
            builder::GetCredentialsResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetDeviceResponse {
        ///A hashmap of device types and their respective event counts.
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub device: std::collections::HashMap<String, ProviderDeviceEvents>,
        ///The tag for which the events are being retrieved.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&GetDeviceResponse> for GetDeviceResponse {
        fn from(value: &GetDeviceResponse) -> Self {
            value.clone()
        }
    }

    impl GetDeviceResponse {
        pub fn builder() -> builder::GetDeviceResponse {
            builder::GetDeviceResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetDomainTags(pub Vec<GetDomainTagsItem>);
    impl std::ops::Deref for GetDomainTags {
        type Target = Vec<GetDomainTagsItem>;
        fn deref(&self) -> &Vec<GetDomainTagsItem> {
            &self.0
        }
    }

    impl From<GetDomainTags> for Vec<GetDomainTagsItem> {
        fn from(value: GetDomainTags) -> Self {
            value.0
        }
    }

    impl From<&GetDomainTags> for GetDomainTags {
        fn from(value: &GetDomainTags) -> Self {
            value.clone()
        }
    }

    impl From<Vec<GetDomainTagsItem>> for GetDomainTags {
        fn from(value: Vec<GetDomainTagsItem>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetDomainTagsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&GetDomainTagsItem> for GetDomainTagsItem {
        fn from(value: &GetDomainTagsItem) -> Self {
            value.clone()
        }
    }

    impl GetDomainTagsItem {
        pub fn builder() -> builder::GetDomainTagsItem {
            builder::GetDomainTagsItem::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetDomainsResponse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub items: Vec<Domain>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub total_count: Option<f64>,
    }

    impl From<&GetDomainsResponse> for GetDomainsResponse {
        fn from(value: &GetDomainsResponse) -> Self {
            value.clone()
        }
    }

    impl GetDomainsResponse {
        pub fn builder() -> builder::GetDomainsResponse {
            builder::GetDomainsResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetProvidersEvents {
        ///A hashmap of provider domains and their respective event counts.
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub provider: std::collections::HashMap<String, ProviderEvents>,
        ///The tag for which the events are being retrieved.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&GetProvidersEvents> for GetProvidersEvents {
        fn from(value: &GetProvidersEvents) -> Self {
            value.clone()
        }
    }

    impl GetProvidersEvents {
        pub fn builder() -> builder::GetProvidersEvents {
            builder::GetProvidersEvents::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTrackingResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub click: Option<GetTrackingResponseClick>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub open: Option<GetTrackingResponseOpen>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unsubscribe: Option<GetTrackingResponseUnsubscribe>,
    }

    impl From<&GetTrackingResponse> for GetTrackingResponse {
        fn from(value: &GetTrackingResponse) -> Self {
            value.clone()
        }
    }

    impl GetTrackingResponse {
        pub fn builder() -> builder::GetTrackingResponse {
            builder::GetTrackingResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTrackingResponseClick {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
    }

    impl From<&GetTrackingResponseClick> for GetTrackingResponseClick {
        fn from(value: &GetTrackingResponseClick) -> Self {
            value.clone()
        }
    }

    impl GetTrackingResponseClick {
        pub fn builder() -> builder::GetTrackingResponseClick {
            builder::GetTrackingResponseClick::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTrackingResponseOpen {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
    }

    impl From<&GetTrackingResponseOpen> for GetTrackingResponseOpen {
        fn from(value: &GetTrackingResponseOpen) -> Self {
            value.clone()
        }
    }

    impl GetTrackingResponseOpen {
        pub fn builder() -> builder::GetTrackingResponseOpen {
            builder::GetTrackingResponseOpen::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GetTrackingResponseUnsubscribe {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<bool>,
    }

    impl From<&GetTrackingResponseUnsubscribe> for GetTrackingResponseUnsubscribe {
        fn from(value: &GetTrackingResponseUnsubscribe) -> Self {
            value.clone()
        }
    }

    impl GetTrackingResponseUnsubscribe {
        pub fn builder() -> builder::GetTrackingResponseUnsubscribe {
            builder::GetTrackingResponseUnsubscribe::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListMemberAddressResponse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub items: Vec<Member>,
    }

    impl From<&ListMemberAddressResponse> for ListMemberAddressResponse {
        fn from(value: &ListMemberAddressResponse) -> Self {
            value.clone()
        }
    }

    impl ListMemberAddressResponse {
        pub fn builder() -> builder::ListMemberAddressResponse {
            builder::ListMemberAddressResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListMembersResponse {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub items: Vec<Member>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub paging: Option<ListMembersResponsePaging>,
    }

    impl From<&ListMembersResponse> for ListMembersResponse {
        fn from(value: &ListMembersResponse) -> Self {
            value.clone()
        }
    }

    impl ListMembersResponse {
        pub fn builder() -> builder::ListMembersResponse {
            builder::ListMembersResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ListMembersResponsePaging {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub first: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub last: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub next: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub previous: Option<String>,
    }

    impl From<&ListMembersResponsePaging> for ListMembersResponsePaging {
        fn from(value: &ListMembersResponsePaging) -> Self {
            value.clone()
        }
    }

    impl ListMembersResponsePaging {
        pub fn builder() -> builder::ListMembersResponsePaging {
            builder::ListMembersResponsePaging::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Member {
        ///The email address of the member.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///The name of the member.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///Indicates whether the member is subscribed to the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscribed: Option<bool>,
        ///A collection of custom variables stored for the member.
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub vars: std::collections::HashMap<String, String>,
    }

    impl From<&Member> for Member {
        fn from(value: &Member) -> Self {
            value.clone()
        }
    }

    impl Member {
        pub fn builder() -> builder::Member {
            builder::Member::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProviderCountryEvents {
        ///The number of accepted emails.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accepted: Option<i64>,
        ///The number of clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clicked: Option<i64>,
        ///The number of complaints (i.e. spam reports) received.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub complained: Option<i64>,
        ///The number of emails that were successfully delivered.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delivered: Option<i64>,
        ///The number of times the email was opened.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub opened: Option<i64>,
        ///The number of unique clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_clicked: Option<i64>,
        ///The number of unique opens of the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_opened: Option<i64>,
        ///The number of unsubscribes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<i64>,
    }

    impl From<&ProviderCountryEvents> for ProviderCountryEvents {
        fn from(value: &ProviderCountryEvents) -> Self {
            value.clone()
        }
    }

    impl ProviderCountryEvents {
        pub fn builder() -> builder::ProviderCountryEvents {
            builder::ProviderCountryEvents::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProviderDeviceEvents {
        ///The number of clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clicked: Option<i64>,
        ///The number of complaints (i.e. spam reports) received.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub complained: Option<i64>,
        ///The number of times the email was opened.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub opened: Option<i64>,
        ///The number of unique clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_clicked: Option<i64>,
        ///The number of unique opens of the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_opened: Option<i64>,
        ///The number of unsubscribes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<i64>,
    }

    impl From<&ProviderDeviceEvents> for ProviderDeviceEvents {
        fn from(value: &ProviderDeviceEvents) -> Self {
            value.clone()
        }
    }

    impl ProviderDeviceEvents {
        pub fn builder() -> builder::ProviderDeviceEvents {
            builder::ProviderDeviceEvents::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProviderEvents {
        ///The number of accepted emails.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub accepted: Option<i64>,
        ///The number of clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub clicked: Option<i64>,
        ///The number of complaints (i.e. spam reports) received.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub complained: Option<i64>,
        ///The number of emails that were successfully delivered.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delivered: Option<i64>,
        ///The number of times the email was opened.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub opened: Option<i64>,
        ///The number of unique clicks on links within the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_clicked: Option<i64>,
        ///The number of unique opens of the email.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unique_opened: Option<i64>,
        ///The number of unsubscribes.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unsubscribed: Option<i64>,
    }

    impl From<&ProviderEvents> for ProviderEvents {
        fn from(value: &ProviderEvents) -> Self {
            value.clone()
        }
    }

    impl ProviderEvents {
        pub fn builder() -> builder::ProviderEvents {
            builder::ProviderEvents::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ReceivingDnsRecord {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub cached: Vec<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub record_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&ReceivingDnsRecord> for ReceivingDnsRecord {
        fn from(value: &ReceivingDnsRecord) -> Self {
            value.clone()
        }
    }

    impl ReceivingDnsRecord {
        pub fn builder() -> builder::ReceivingDnsRecord {
            builder::ReceivingDnsRecord::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SendMessageBody {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attachment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bcc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cc: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        #[serde(
            rename = "h:X-My-Header",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub h_x_my_header: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub html: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub inline: Option<String>,
        #[serde(
            rename = "o:deliverytime",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_deliverytime: Option<String>,
        #[serde(
            rename = "o:deliverytime-optimize-period",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_deliverytime_optimize_period: Option<String>,
        #[serde(rename = "o:dkim", default, skip_serializing_if = "Option::is_none")]
        pub o_dkim: Option<String>,
        #[serde(
            rename = "o:require-tls",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_require_tls: Option<String>,
        #[serde(
            rename = "o:skip-verification",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_skip_verification: Option<String>,
        #[serde(rename = "o:tag", default, skip_serializing_if = "Option::is_none")]
        pub o_tag: Option<String>,
        #[serde(
            rename = "o:testmode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_testmode: Option<String>,
        #[serde(
            rename = "o:time-zone-localize",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_time_zone_localize: Option<String>,
        #[serde(
            rename = "o:tracking",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_tracking: Option<String>,
        #[serde(
            rename = "o:tracking-clicks",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_tracking_clicks: Option<String>,
        #[serde(
            rename = "o:tracking-opens",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_tracking_opens: Option<String>,
        #[serde(
            rename = "o:tracking-pixel-location-top",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_tracking_pixel_location_top: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subject: Option<String>,
        #[serde(rename = "t:text", default, skip_serializing_if = "Option::is_none")]
        pub t_text: Option<bool>,
        #[serde(
            rename = "t:variables",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub t_variables: Option<String>,
        #[serde(rename = "t:version", default, skip_serializing_if = "Option::is_none")]
        pub t_version: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub template: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
        #[serde(rename = "v:my-var", default, skip_serializing_if = "Option::is_none")]
        pub v_my_var: Option<String>,
    }

    impl From<&SendMessageBody> for SendMessageBody {
        fn from(value: &SendMessageBody) -> Self {
            value.clone()
        }
    }

    impl SendMessageBody {
        pub fn builder() -> builder::SendMessageBody {
            builder::SendMessageBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SendingDnsRecord {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub cached: Vec<serde_json::Value>,
        ///The name of the DNS record.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///The type of DNS record.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub record_type: Option<String>,
        ///The validation status of the DNS record.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<String>,
        ///The value of the DNS record.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&SendingDnsRecord> for SendingDnsRecord {
        fn from(value: &SendingDnsRecord) -> Self {
            value.clone()
        }
    }

    impl SendingDnsRecord {
        pub fn builder() -> builder::SendingDnsRecord {
            builder::SendingDnsRecord::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct StoredMessage {
        ///Contains a json list of metadata objects, one for each attachment,
        /// see below.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attachments: Vec<Attachment>,
        ///HTML version of the message, if message was multipart. Note that all
        /// parts of the message will be posted, not just text/html. For
        /// instance if a message arrives with “foo” part it will be posted as
        /// “body-foo”.
        #[serde(rename = "body-html", default, skip_serializing_if = "Option::is_none")]
        pub body_html: Option<String>,
        ///Text version of the email. This field is always present. If the
        /// incoming message only has HTML body, Mailgun will create a text
        /// representation for you.
        #[serde(
            rename = "body-plain",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub body_plain: Option<String>,
        ///JSON-encoded dictionary which maps Content-ID (CID) of each
        /// attachment to the corresponding attachment-x parameter. This allows
        /// you to map posted attachments to tags like <img src='cid'> in the
        /// message body.
        #[serde(
            rename = "content-id-map",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub content_id_map: serde_json::Map<String, serde_json::Value>,
        ///Sender of the message as reported by From message header, for
        /// example “Bob Lee <blee@mailgun.net>”.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        ///List of all MIME headers dumped to a json string (order of headers
        /// preserved).
        #[serde(
            rename = "message-headers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub message_headers: Vec<String>,
        ///Recipient of the message as reported by MAIL TO during SMTP chat.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub recipients: Option<String>,
        ///Sender of the message as reported by MAIL FROM during SMTP chat.
        /// Note: this value may differ from From MIME header.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sender: Option<String>,
        ///HTML version of the message, without quoted parts.
        #[serde(
            rename = "stripped-html",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stripped_html: Option<String>,
        ///The signature block stripped from the plain text message (if found).
        #[serde(
            rename = "stripped-signature",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stripped_signature: Option<String>,
        ///Text version of the message without quoted parts and signature block
        /// (if found).
        #[serde(
            rename = "stripped-text",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub stripped_text: Option<String>,
        ///Subject string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subject: Option<String>,
    }

    impl From<&StoredMessage> for StoredMessage {
        fn from(value: &StoredMessage) -> Self {
            value.clone()
        }
    }

    impl StoredMessage {
        pub fn builder() -> builder::StoredMessage {
            builder::StoredMessage::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SuccessResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&SuccessResponse> for SuccessResponse {
        fn from(value: &SuccessResponse) -> Self {
            value.clone()
        }
    }

    impl SuccessResponse {
        pub fn builder() -> builder::SuccessResponse {
            builder::SuccessResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Unsubscribe {
        pub address: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tags: Vec<String>,
    }

    impl From<&Unsubscribe> for Unsubscribe {
        fn from(value: &Unsubscribe) -> Self {
            value.clone()
        }
    }

    impl Unsubscribe {
        pub fn builder() -> builder::Unsubscribe {
            builder::Unsubscribe::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UnsubscribeRequest(pub Vec<Unsubscribe>);
    impl std::ops::Deref for UnsubscribeRequest {
        type Target = Vec<Unsubscribe>;
        fn deref(&self) -> &Vec<Unsubscribe> {
            &self.0
        }
    }

    impl From<UnsubscribeRequest> for Vec<Unsubscribe> {
        fn from(value: UnsubscribeRequest) -> Self {
            value.0
        }
    }

    impl From<&UnsubscribeRequest> for UnsubscribeRequest {
        fn from(value: &UnsubscribeRequest) -> Self {
            value.clone()
        }
    }

    impl From<Vec<Unsubscribe>> for UnsubscribeRequest {
        fn from(value: Vec<Unsubscribe>) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDkimAuthorityBody {
        ///_true_ or _false_
        ///
        ///Change the DKIM authority for a domain.
        ///
        ///If set to _true_, the domain will be the DKIM authority for itself
        /// even if the root domain is registered on the same mailgun account
        ///
        ///If set to _false_, the domain will have the same DKIM authority as
        /// the root domain registered on the same mailgun account
        #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
        pub self_: Option<String>,
    }

    impl From<&UpdateDkimAuthorityBody> for UpdateDkimAuthorityBody {
        fn from(value: &UpdateDkimAuthorityBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDkimAuthorityBody {
        pub fn builder() -> builder::UpdateDkimAuthorityBody {
            builder::UpdateDkimAuthorityBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDomainClickTrackingBody {
        ///yes, no, or htmlonly
        ///
        ///If set to yes, links will be overwritten and pointed to our servers
        /// so we can track clicks.
        ///
        ///If set to htmlonly, links will only be rewritten in the HTML part of
        /// a message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<String>,
    }

    impl From<&UpdateDomainClickTrackingBody> for UpdateDomainClickTrackingBody {
        fn from(value: &UpdateDomainClickTrackingBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDomainClickTrackingBody {
        pub fn builder() -> builder::UpdateDomainClickTrackingBody {
            builder::UpdateDomainClickTrackingBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDomainConnectionBody {
        ///true or false
        ///
        ///If set to true, this requires the message only be sent over a TLS
        /// connection. If a TLS connection can not be established, Mailgun will
        /// not deliver the message.
        ///
        ///If set to false, Mailgun will still try and upgrade the connection,
        /// but if Mailgun cannot, the message will be delivered over a
        /// plaintext SMTP connection.
        ///
        ///The default is false.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_tls: Option<bool>,
        ///true or false
        ///
        ///If set to true, the certificate and hostname will not be verified
        /// when trying to establish a TLS connection and Mailgun will accept
        /// any certificate during delivery.
        ///
        ///If set to false, Mailgun will verify the certificate and hostname.
        /// If either one can not be verified, a TLS connection will not be
        /// established.
        ///
        ///The default is false.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skip_verification: Option<bool>,
    }

    impl From<&UpdateDomainConnectionBody> for UpdateDomainConnectionBody {
        fn from(value: &UpdateDomainConnectionBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDomainConnectionBody {
        pub fn builder() -> builder::UpdateDomainConnectionBody {
            builder::UpdateDomainConnectionBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDomainTagBody {
        ///Optional description of a tag
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///Name of the domain
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,
        ///Name of the tag
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: Option<String>,
    }

    impl From<&UpdateDomainTagBody> for UpdateDomainTagBody {
        fn from(value: &UpdateDomainTagBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDomainTagBody {
        pub fn builder() -> builder::UpdateDomainTagBody {
            builder::UpdateDomainTagBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDomainTrackingBody {
        ///`yes` or `no`
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<String>,
    }

    impl From<&UpdateDomainTrackingBody> for UpdateDomainTrackingBody {
        fn from(value: &UpdateDomainTrackingBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDomainTrackingBody {
        pub fn builder() -> builder::UpdateDomainTrackingBody {
            builder::UpdateDomainTrackingBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateDomainUnsubscribeTrackingBody {
        ///true or false.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub active: Option<String>,
        ///Custom HTML version of unsubscribe footer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub html_footer: Option<String>,
        ///Custom text version of unsubscribe footer.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub text_footer: Option<String>,
    }

    impl From<&UpdateDomainUnsubscribeTrackingBody> for UpdateDomainUnsubscribeTrackingBody {
        fn from(value: &UpdateDomainUnsubscribeTrackingBody) -> Self {
            value.clone()
        }
    }

    impl UpdateDomainUnsubscribeTrackingBody {
        pub fn builder() -> builder::UpdateDomainUnsubscribeTrackingBody {
            builder::UpdateDomainUnsubscribeTrackingBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateListBody {
        ///List access level, one of: readonly (default), members, everyone
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub access_level: Option<String>,
        ///New mailing list address, e.g. devs@mg.net (optional)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///Description string (optional)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///New name, e.g. My newsletter (optional)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&UpdateListBody> for UpdateListBody {
        fn from(value: &UpdateListBody) -> Self {
            value.clone()
        }
    }

    impl UpdateListBody {
        pub fn builder() -> builder::UpdateListBody {
            builder::UpdateListBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateListMemberBody {
        ///Valid email address specification, e.g. Alice <alice@example.com> or
        /// just alice@example.com
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///Recipient name, e.g. Alice
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        ///no to set unsubscribed, yes as subscribed
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subscribed: Option<String>,
        ///JSON-encoded dictionary string with arbitrary parameters, e.g.
        /// {"gender":"female","age":27}

        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub vars: Option<String>,
    }

    impl From<&UpdateListMemberBody> for UpdateListMemberBody {
        fn from(value: &UpdateListMemberBody) -> Self {
            value.clone()
        }
    }

    impl UpdateListMemberBody {
        pub fn builder() -> builder::UpdateListMemberBody {
            builder::UpdateListMemberBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateMailingList {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub list: Option<UpdateMailingListList>,
        ///A message indicating that the mailing list has been updated.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }

    impl From<&UpdateMailingList> for UpdateMailingList {
        fn from(value: &UpdateMailingList) -> Self {
            value.clone()
        }
    }

    impl UpdateMailingList {
        pub fn builder() -> builder::UpdateMailingList {
            builder::UpdateMailingList::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateMailingListList {
        ///The access level of the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub access_level: Option<String>,
        ///The email address of the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<String>,
        ///The date and time when the mailing list was created.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        ///The updated description of the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///The number of members in the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub members_count: Option<i64>,
        ///The updated name of the mailing list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&UpdateMailingListList> for UpdateMailingListList {
        fn from(value: &UpdateMailingListList) -> Self {
            value.clone()
        }
    }

    impl UpdateMailingListList {
        pub fn builder() -> builder::UpdateMailingListList {
            builder::UpdateMailingListList::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateRouteBody {
        ///Route action. This action is executed when the expression evaluates
        /// to True. Example: forward("alice@example.com") You can pass multiple
        /// action parameters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub action: Option<String>,
        ///An arbitrary string
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        ///A filter expression like match_recipient('.*@gmail.com')
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expression: Option<String>,
        ///ID of the route
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        ///Integer: smaller number indicates higher priority. Higher priority
        /// routes are handled first.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<String>,
    }

    impl From<&UpdateRouteBody> for UpdateRouteBody {
        fn from(value: &UpdateRouteBody) -> Self {
            value.clone()
        }
    }

    impl UpdateRouteBody {
        pub fn builder() -> builder::UpdateRouteBody {
            builder::UpdateRouteBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateSmtpCredentialsBody {
        ///A password for the SMTP credentials. (Length Min 5, Max 32)
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
    }

    impl From<&UpdateSmtpCredentialsBody> for UpdateSmtpCredentialsBody {
        fn from(value: &UpdateSmtpCredentialsBody) -> Self {
            value.clone()
        }
    }

    impl UpdateSmtpCredentialsBody {
        pub fn builder() -> builder::UpdateSmtpCredentialsBody {
            builder::UpdateSmtpCredentialsBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateWebhookBody {
        ///Name of the webhook
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub domain: Option<String>,
        ///URL for the webhook event. May be repeated up to 3 times.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        ///Name of the webhook
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub webhookname: Option<String>,
    }

    impl From<&UpdateWebhookBody> for UpdateWebhookBody {
        fn from(value: &UpdateWebhookBody) -> Self {
            value.clone()
        }
    }

    impl UpdateWebhookBody {
        pub fn builder() -> builder::UpdateWebhookBody {
            builder::UpdateWebhookBody::default()
        }
    }

    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AddBounceBody {
            address: Result<Option<String>, String>,
            code: Result<Option<String>, String>,
            created_at: Result<Option<String>, String>,
            error: Result<Option<String>, String>,
        }

        impl Default for AddBounceBody {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    code: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    error: Ok(Default::default()),
                }
            }
        }

        impl AddBounceBody {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn error<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddBounceBody> for super::AddBounceBody {
            type Error = String;
            fn try_from(value: AddBounceBody) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    code: value.code?,
                    created_at: value.created_at?,
                    error: value.error?,
                })
            }
        }

        impl From<super::AddBounceBody> for AddBounceBody {
            fn from(value: super::AddBounceBody) -> Self {
                Self {
                    address: Ok(value.address),
                    code: Ok(value.code),
                    created_at: Ok(value.created_at),
                    error: Ok(value.error),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddBounceResponse {
            address: Result<Option<String>, String>,
            message: Result<Option<String>, String>,
        }

        impl Default for AddBounceResponse {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl AddBounceResponse {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddBounceResponse> for super::AddBounceResponse {
            type Error = String;
            fn try_from(value: AddBounceResponse) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    message: value.message?,
                })
            }
        }

        impl From<super::AddBounceResponse> for AddBounceResponse {
            fn from(value: super::AddBounceResponse) -> Self {
                Self {
                    address: Ok(value.address),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddComplaintBody {
            address: Result<Option<String>, String>,
            created_at: Result<Option<String>, String>,
        }

        impl Default for AddComplaintBody {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                }
            }
        }

        impl AddComplaintBody {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddComplaintBody> for super::AddComplaintBody {
            type Error = String;
            fn try_from(value: AddComplaintBody) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    created_at: value.created_at?,
                })
            }
        }

        impl From<super::AddComplaintBody> for AddComplaintBody {
            fn from(value: super::AddComplaintBody) -> Self {
                Self {
                    address: Ok(value.address),
                    created_at: Ok(value.created_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddListMemberBody {
            address: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            subscribed: Result<Option<String>, String>,
            upsert: Result<Option<String>, String>,
            vars: Result<Option<String>, String>,
        }

        impl Default for AddListMemberBody {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    name: Ok(Default::default()),
                    subscribed: Ok(Default::default()),
                    upsert: Ok(Default::default()),
                    vars: Ok(Default::default()),
                }
            }
        }

        impl AddListMemberBody {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn subscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.subscribed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subscribed: {}", e));
                self
            }
            pub fn upsert<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.upsert = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upsert: {}", e));
                self
            }
            pub fn vars<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.vars = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vars: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddListMemberBody> for super::AddListMemberBody {
            type Error = String;
            fn try_from(value: AddListMemberBody) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    name: value.name?,
                    subscribed: value.subscribed?,
                    upsert: value.upsert?,
                    vars: value.vars?,
                })
            }
        }

        impl From<super::AddListMemberBody> for AddListMemberBody {
            fn from(value: super::AddListMemberBody) -> Self {
                Self {
                    address: Ok(value.address),
                    name: Ok(value.name),
                    subscribed: Ok(value.subscribed),
                    upsert: Ok(value.upsert),
                    vars: Ok(value.vars),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddListMembersBody {
            members: Result<Option<String>, String>,
            upsert: Result<Option<String>, String>,
        }

        impl Default for AddListMembersBody {
            fn default() -> Self {
                Self {
                    members: Ok(Default::default()),
                    upsert: Ok(Default::default()),
                }
            }
        }

        impl AddListMembersBody {
            pub fn members<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.members = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for members: {}", e));
                self
            }
            pub fn upsert<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.upsert = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for upsert: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddListMembersBody> for super::AddListMembersBody {
            type Error = String;
            fn try_from(value: AddListMembersBody) -> Result<Self, String> {
                Ok(Self {
                    members: value.members?,
                    upsert: value.upsert?,
                })
            }
        }

        impl From<super::AddListMembersBody> for AddListMembersBody {
            fn from(value: super::AddListMembersBody) -> Self {
                Self {
                    members: Ok(value.members),
                    upsert: Ok(value.upsert),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AssignIpToDomainBody {
            ip: Result<Option<String>, String>,
        }

        impl Default for AssignIpToDomainBody {
            fn default() -> Self {
                Self {
                    ip: Ok(Default::default()),
                }
            }
        }

        impl AssignIpToDomainBody {
            pub fn ip<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.ip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ip: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AssignIpToDomainBody> for super::AssignIpToDomainBody {
            type Error = String;
            fn try_from(value: AssignIpToDomainBody) -> Result<Self, String> {
                Ok(Self { ip: value.ip? })
            }
        }

        impl From<super::AssignIpToDomainBody> for AssignIpToDomainBody {
            fn from(value: super::AssignIpToDomainBody) -> Self {
                Self { ip: Ok(value.ip) }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Attachment {
            content_type: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            size: Result<Option<i64>, String>,
            url: Result<Option<String>, String>,
        }

        impl Default for Attachment {
            fn default() -> Self {
                Self {
                    content_type: Ok(Default::default()),
                    name: Ok(Default::default()),
                    size: Ok(Default::default()),
                    url: Ok(Default::default()),
                }
            }
        }

        impl Attachment {
            pub fn content_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.content_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for content_type: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn size<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Attachment> for super::Attachment {
            type Error = String;
            fn try_from(value: Attachment) -> Result<Self, String> {
                Ok(Self {
                    content_type: value.content_type?,
                    name: value.name?,
                    size: value.size?,
                    url: value.url?,
                })
            }
        }

        impl From<super::Attachment> for Attachment {
            fn from(value: super::Attachment) -> Self {
                Self {
                    content_type: Ok(value.content_type),
                    name: Ok(value.name),
                    size: Ok(value.size),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct BounceMail {
            address: Result<String, String>,
            code: Result<Option<String>, String>,
            created_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            error: Result<Option<String>, String>,
        }

        impl Default for BounceMail {
            fn default() -> Self {
                Self {
                    address: Err("no value supplied for address".to_string()),
                    code: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    error: Ok(Default::default()),
                }
            }
        }

        impl BounceMail {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn error<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<BounceMail> for super::BounceMail {
            type Error = String;
            fn try_from(value: BounceMail) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    code: value.code?,
                    created_at: value.created_at?,
                    error: value.error?,
                })
            }
        }

        impl From<super::BounceMail> for BounceMail {
            fn from(value: super::BounceMail) -> Self {
                Self {
                    address: Ok(value.address),
                    code: Ok(value.code),
                    created_at: Ok(value.created_at),
                    error: Ok(value.error),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Complaint {
            address: Result<String, String>,
            created_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        }

        impl Default for Complaint {
            fn default() -> Self {
                Self {
                    address: Err("no value supplied for address".to_string()),
                    created_at: Ok(Default::default()),
                }
            }
        }

        impl Complaint {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Complaint> for super::Complaint {
            type Error = String;
            fn try_from(value: Complaint) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    created_at: value.created_at?,
                })
            }
        }

        impl From<super::Complaint> for Complaint {
            fn from(value: super::Complaint) -> Self {
                Self {
                    address: Ok(value.address),
                    created_at: Ok(value.created_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ConnectionUpdate {
            connection: Result<Option<super::ConnectionUpdateConnection>, String>,
            message: Result<Option<String>, String>,
        }

        impl Default for ConnectionUpdate {
            fn default() -> Self {
                Self {
                    connection: Ok(Default::default()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl ConnectionUpdate {
            pub fn connection<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::ConnectionUpdateConnection>>,
                T::Error: std::fmt::Display,
            {
                self.connection = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for connection: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ConnectionUpdate> for super::ConnectionUpdate {
            type Error = String;
            fn try_from(value: ConnectionUpdate) -> Result<Self, String> {
                Ok(Self {
                    connection: value.connection?,
                    message: value.message?,
                })
            }
        }

        impl From<super::ConnectionUpdate> for ConnectionUpdate {
            fn from(value: super::ConnectionUpdate) -> Self {
                Self {
                    connection: Ok(value.connection),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ConnectionUpdateConnection {
            require_tls: Result<Option<bool>, String>,
            skip_verification: Result<Option<bool>, String>,
        }

        impl Default for ConnectionUpdateConnection {
            fn default() -> Self {
                Self {
                    require_tls: Ok(Default::default()),
                    skip_verification: Ok(Default::default()),
                }
            }
        }

        impl ConnectionUpdateConnection {
            pub fn require_tls<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.require_tls = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for require_tls: {}", e));
                self
            }
            pub fn skip_verification<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_verification = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_verification: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<ConnectionUpdateConnection> for super::ConnectionUpdateConnection {
            type Error = String;
            fn try_from(value: ConnectionUpdateConnection) -> Result<Self, String> {
                Ok(Self {
                    require_tls: value.require_tls?,
                    skip_verification: value.skip_verification?,
                })
            }
        }

        impl From<super::ConnectionUpdateConnection> for ConnectionUpdateConnection {
            fn from(value: super::ConnectionUpdateConnection) -> Self {
                Self {
                    require_tls: Ok(value.require_tls),
                    skip_verification: Ok(value.skip_verification),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateDomainBody {
            force_dkim_authority: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            smtp_password: Result<Option<String>, String>,
            spam_action: Result<Option<String>, String>,
            wildcard: Result<Option<String>, String>,
        }

        impl Default for CreateDomainBody {
            fn default() -> Self {
                Self {
                    force_dkim_authority: Ok(Default::default()),
                    name: Ok(Default::default()),
                    smtp_password: Ok(Default::default()),
                    spam_action: Ok(Default::default()),
                    wildcard: Ok(Default::default()),
                }
            }
        }

        impl CreateDomainBody {
            pub fn force_dkim_authority<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.force_dkim_authority = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for force_dkim_authority: {}",
                        e
                    )
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn smtp_password<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.smtp_password = value.try_into().map_err(|e| {
                    format!("error converting supplied value for smtp_password: {}", e)
                });
                self
            }
            pub fn spam_action<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.spam_action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for spam_action: {}", e));
                self
            }
            pub fn wildcard<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.wildcard = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for wildcard: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateDomainBody> for super::CreateDomainBody {
            type Error = String;
            fn try_from(value: CreateDomainBody) -> Result<Self, String> {
                Ok(Self {
                    force_dkim_authority: value.force_dkim_authority?,
                    name: value.name?,
                    smtp_password: value.smtp_password?,
                    spam_action: value.spam_action?,
                    wildcard: value.wildcard?,
                })
            }
        }

        impl From<super::CreateDomainBody> for CreateDomainBody {
            fn from(value: super::CreateDomainBody) -> Self {
                Self {
                    force_dkim_authority: Ok(value.force_dkim_authority),
                    name: Ok(value.name),
                    smtp_password: Ok(value.smtp_password),
                    spam_action: Ok(value.spam_action),
                    wildcard: Ok(value.wildcard),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateListBody {
            access_level: Result<Option<String>, String>,
            address: Result<Option<String>, String>,
            description: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
        }

        impl Default for CreateListBody {
            fn default() -> Self {
                Self {
                    access_level: Ok(Default::default()),
                    address: Ok(Default::default()),
                    description: Ok(Default::default()),
                    name: Ok(Default::default()),
                }
            }
        }

        impl CreateListBody {
            pub fn access_level<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.access_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_level: {}", e)
                });
                self
            }
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateListBody> for super::CreateListBody {
            type Error = String;
            fn try_from(value: CreateListBody) -> Result<Self, String> {
                Ok(Self {
                    access_level: value.access_level?,
                    address: value.address?,
                    description: value.description?,
                    name: value.name?,
                })
            }
        }

        impl From<super::CreateListBody> for CreateListBody {
            fn from(value: super::CreateListBody) -> Self {
                Self {
                    access_level: Ok(value.access_level),
                    address: Ok(value.address),
                    description: Ok(value.description),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateRouteBody {
            action: Result<Option<String>, String>,
            description: Result<Option<String>, String>,
            expression: Result<Option<String>, String>,
            priority: Result<Option<String>, String>,
        }

        impl Default for CreateRouteBody {
            fn default() -> Self {
                Self {
                    action: Ok(Default::default()),
                    description: Ok(Default::default()),
                    expression: Ok(Default::default()),
                    priority: Ok(Default::default()),
                }
            }
        }

        impl CreateRouteBody {
            pub fn action<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn expression<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.expression = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expression: {}", e));
                self
            }
            pub fn priority<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.priority = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for priority: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateRouteBody> for super::CreateRouteBody {
            type Error = String;
            fn try_from(value: CreateRouteBody) -> Result<Self, String> {
                Ok(Self {
                    action: value.action?,
                    description: value.description?,
                    expression: value.expression?,
                    priority: value.priority?,
                })
            }
        }

        impl From<super::CreateRouteBody> for CreateRouteBody {
            fn from(value: super::CreateRouteBody) -> Self {
                Self {
                    action: Ok(value.action),
                    description: Ok(value.description),
                    expression: Ok(value.expression),
                    priority: Ok(value.priority),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateSmtpCredentialsBody {
            login: Result<Option<String>, String>,
            password: Result<Option<String>, String>,
        }

        impl Default for CreateSmtpCredentialsBody {
            fn default() -> Self {
                Self {
                    login: Ok(Default::default()),
                    password: Ok(Default::default()),
                }
            }
        }

        impl CreateSmtpCredentialsBody {
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
            pub fn password<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.password = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for password: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateSmtpCredentialsBody> for super::CreateSmtpCredentialsBody {
            type Error = String;
            fn try_from(value: CreateSmtpCredentialsBody) -> Result<Self, String> {
                Ok(Self {
                    login: value.login?,
                    password: value.password?,
                })
            }
        }

        impl From<super::CreateSmtpCredentialsBody> for CreateSmtpCredentialsBody {
            fn from(value: super::CreateSmtpCredentialsBody) -> Self {
                Self {
                    login: Ok(value.login),
                    password: Ok(value.password),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateWebhookBody {
            domain: Result<Option<String>, String>,
            id: Result<Option<String>, String>,
            url: Result<Option<String>, String>,
        }

        impl Default for CreateWebhookBody {
            fn default() -> Self {
                Self {
                    domain: Ok(Default::default()),
                    id: Ok(Default::default()),
                    url: Ok(Default::default()),
                }
            }
        }

        impl CreateWebhookBody {
            pub fn domain<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.domain = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for domain: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateWebhookBody> for super::CreateWebhookBody {
            type Error = String;
            fn try_from(value: CreateWebhookBody) -> Result<Self, String> {
                Ok(Self {
                    domain: value.domain?,
                    id: value.id?,
                    url: value.url?,
                })
            }
        }

        impl From<super::CreateWebhookBody> for CreateWebhookBody {
            fn from(value: super::CreateWebhookBody) -> Self {
                Self {
                    domain: Ok(value.domain),
                    id: Ok(value.id),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Credential {
            created_at: Result<Option<String>, String>,
            login: Result<Option<String>, String>,
            mailbox: Result<Option<String>, String>,
            size_bytes: Result<Option<i64>, String>,
        }

        impl Default for Credential {
            fn default() -> Self {
                Self {
                    created_at: Ok(Default::default()),
                    login: Ok(Default::default()),
                    mailbox: Ok(Default::default()),
                    size_bytes: Ok(Default::default()),
                }
            }
        }

        impl Credential {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
            pub fn mailbox<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.mailbox = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mailbox: {}", e));
                self
            }
            pub fn size_bytes<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.size_bytes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for size_bytes: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Credential> for super::Credential {
            type Error = String;
            fn try_from(value: Credential) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    login: value.login?,
                    mailbox: value.mailbox?,
                    size_bytes: value.size_bytes?,
                })
            }
        }

        impl From<super::Credential> for Credential {
            fn from(value: super::Credential) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    login: Ok(value.login),
                    mailbox: Ok(value.mailbox),
                    size_bytes: Ok(value.size_bytes),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteCredentialsResponse {
            message: Result<Option<String>, String>,
        }

        impl Default for DeleteCredentialsResponse {
            fn default() -> Self {
                Self {
                    message: Ok(Default::default()),
                }
            }
        }

        impl DeleteCredentialsResponse {
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DeleteCredentialsResponse> for super::DeleteCredentialsResponse {
            type Error = String;
            fn try_from(value: DeleteCredentialsResponse) -> Result<Self, String> {
                Ok(Self {
                    message: value.message?,
                })
            }
        }

        impl From<super::DeleteCredentialsResponse> for DeleteCredentialsResponse {
            fn from(value: super::DeleteCredentialsResponse) -> Self {
                Self {
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteListMember {
            member: Result<Option<super::Member>, String>,
            message: Result<Option<String>, String>,
        }

        impl Default for DeleteListMember {
            fn default() -> Self {
                Self {
                    member: Ok(Default::default()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl DeleteListMember {
            pub fn member<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Member>>,
                T::Error: std::fmt::Display,
            {
                self.member = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for member: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DeleteListMember> for super::DeleteListMember {
            type Error = String;
            fn try_from(value: DeleteListMember) -> Result<Self, String> {
                Ok(Self {
                    member: value.member?,
                    message: value.message?,
                })
            }
        }

        impl From<super::DeleteListMember> for DeleteListMember {
            fn from(value: super::DeleteListMember) -> Self {
                Self {
                    member: Ok(value.member),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DeleteTag {
            message: Result<Option<String>, String>,
        }

        impl Default for DeleteTag {
            fn default() -> Self {
                Self {
                    message: Ok(Default::default()),
                }
            }
        }

        impl DeleteTag {
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DeleteTag> for super::DeleteTag {
            type Error = String;
            fn try_from(value: DeleteTag) -> Result<Self, String> {
                Ok(Self {
                    message: value.message?,
                })
            }
        }

        impl From<super::DeleteTag> for DeleteTag {
            fn from(value: super::DeleteTag) -> Self {
                Self {
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Domain {
            created_at: Result<Option<String>, String>,
            id: Result<Option<String>, String>,
            is_disabled: Result<Option<bool>, String>,
            name: Result<Option<String>, String>,
            require_tls: Result<Option<bool>, String>,
            skip_verification: Result<Option<bool>, String>,
            smtp_login: Result<Option<String>, String>,
            spam_action: Result<Option<String>, String>,
            state: Result<Option<String>, String>,
            type_: Result<Option<String>, String>,
            web_prefix: Result<Option<String>, String>,
            web_scheme: Result<Option<String>, String>,
            wildcard: Result<Option<bool>, String>,
        }

        impl Default for Domain {
            fn default() -> Self {
                Self {
                    created_at: Ok(Default::default()),
                    id: Ok(Default::default()),
                    is_disabled: Ok(Default::default()),
                    name: Ok(Default::default()),
                    require_tls: Ok(Default::default()),
                    skip_verification: Ok(Default::default()),
                    smtp_login: Ok(Default::default()),
                    spam_action: Ok(Default::default()),
                    state: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    web_prefix: Ok(Default::default()),
                    web_scheme: Ok(Default::default()),
                    wildcard: Ok(Default::default()),
                }
            }
        }

        impl Domain {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn is_disabled<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.is_disabled = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_disabled: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn require_tls<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.require_tls = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for require_tls: {}", e));
                self
            }
            pub fn skip_verification<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_verification = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_verification: {}",
                        e
                    )
                });
                self
            }
            pub fn smtp_login<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.smtp_login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for smtp_login: {}", e));
                self
            }
            pub fn spam_action<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.spam_action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for spam_action: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
            pub fn web_prefix<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.web_prefix = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_prefix: {}", e));
                self
            }
            pub fn web_scheme<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.web_scheme = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for web_scheme: {}", e));
                self
            }
            pub fn wildcard<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.wildcard = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for wildcard: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Domain> for super::Domain {
            type Error = String;
            fn try_from(value: Domain) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    is_disabled: value.is_disabled?,
                    name: value.name?,
                    require_tls: value.require_tls?,
                    skip_verification: value.skip_verification?,
                    smtp_login: value.smtp_login?,
                    spam_action: value.spam_action?,
                    state: value.state?,
                    type_: value.type_?,
                    web_prefix: value.web_prefix?,
                    web_scheme: value.web_scheme?,
                    wildcard: value.wildcard?,
                })
            }
        }

        impl From<super::Domain> for Domain {
            fn from(value: super::Domain) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    is_disabled: Ok(value.is_disabled),
                    name: Ok(value.name),
                    require_tls: Ok(value.require_tls),
                    skip_verification: Ok(value.skip_verification),
                    smtp_login: Ok(value.smtp_login),
                    spam_action: Ok(value.spam_action),
                    state: Ok(value.state),
                    type_: Ok(value.type_),
                    web_prefix: Ok(value.web_prefix),
                    web_scheme: Ok(value.web_scheme),
                    wildcard: Ok(value.wildcard),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DomainResponse {
            domain: Result<Option<super::Domain>, String>,
            message: Result<Option<String>, String>,
            receiving_dns_records: Result<Vec<super::ReceivingDnsRecord>, String>,
            sending_dns_records: Result<Vec<super::SendingDnsRecord>, String>,
        }

        impl Default for DomainResponse {
            fn default() -> Self {
                Self {
                    domain: Ok(Default::default()),
                    message: Ok(Default::default()),
                    receiving_dns_records: Ok(Default::default()),
                    sending_dns_records: Ok(Default::default()),
                }
            }
        }

        impl DomainResponse {
            pub fn domain<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Domain>>,
                T::Error: std::fmt::Display,
            {
                self.domain = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for domain: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn receiving_dns_records<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::ReceivingDnsRecord>>,
                T::Error: std::fmt::Display,
            {
                self.receiving_dns_records = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for receiving_dns_records: {}",
                        e
                    )
                });
                self
            }
            pub fn sending_dns_records<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::SendingDnsRecord>>,
                T::Error: std::fmt::Display,
            {
                self.sending_dns_records = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for sending_dns_records: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<DomainResponse> for super::DomainResponse {
            type Error = String;
            fn try_from(value: DomainResponse) -> Result<Self, String> {
                Ok(Self {
                    domain: value.domain?,
                    message: value.message?,
                    receiving_dns_records: value.receiving_dns_records?,
                    sending_dns_records: value.sending_dns_records?,
                })
            }
        }

        impl From<super::DomainResponse> for DomainResponse {
            fn from(value: super::DomainResponse) -> Self {
                Self {
                    domain: Ok(value.domain),
                    message: Ok(value.message),
                    receiving_dns_records: Ok(value.receiving_dns_records),
                    sending_dns_records: Ok(value.sending_dns_records),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetConnectionResponse {
            connection: Result<Option<super::GetConnectionResponseConnection>, String>,
        }

        impl Default for GetConnectionResponse {
            fn default() -> Self {
                Self {
                    connection: Ok(Default::default()),
                }
            }
        }

        impl GetConnectionResponse {
            pub fn connection<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::GetConnectionResponseConnection>>,
                T::Error: std::fmt::Display,
            {
                self.connection = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for connection: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetConnectionResponse> for super::GetConnectionResponse {
            type Error = String;
            fn try_from(value: GetConnectionResponse) -> Result<Self, String> {
                Ok(Self {
                    connection: value.connection?,
                })
            }
        }

        impl From<super::GetConnectionResponse> for GetConnectionResponse {
            fn from(value: super::GetConnectionResponse) -> Self {
                Self {
                    connection: Ok(value.connection),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetConnectionResponseConnection {
            require_tls: Result<Option<bool>, String>,
            skip_verification: Result<Option<bool>, String>,
        }

        impl Default for GetConnectionResponseConnection {
            fn default() -> Self {
                Self {
                    require_tls: Ok(Default::default()),
                    skip_verification: Ok(Default::default()),
                }
            }
        }

        impl GetConnectionResponseConnection {
            pub fn require_tls<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.require_tls = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for require_tls: {}", e));
                self
            }
            pub fn skip_verification<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_verification = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_verification: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<GetConnectionResponseConnection>
            for super::GetConnectionResponseConnection
        {
            type Error = String;
            fn try_from(value: GetConnectionResponseConnection) -> Result<Self, String> {
                Ok(Self {
                    require_tls: value.require_tls?,
                    skip_verification: value.skip_verification?,
                })
            }
        }

        impl From<super::GetConnectionResponseConnection> for GetConnectionResponseConnection {
            fn from(value: super::GetConnectionResponseConnection) -> Self {
                Self {
                    require_tls: Ok(value.require_tls),
                    skip_verification: Ok(value.skip_verification),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetCountryResponse {
            provider:
                Result<std::collections::HashMap<String, super::ProviderCountryEvents>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for GetCountryResponse {
            fn default() -> Self {
                Self {
                    provider: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl GetCountryResponse {
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    std::collections::HashMap<String, super::ProviderCountryEvents>,
                >,
                T::Error: std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetCountryResponse> for super::GetCountryResponse {
            type Error = String;
            fn try_from(value: GetCountryResponse) -> Result<Self, String> {
                Ok(Self {
                    provider: value.provider?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::GetCountryResponse> for GetCountryResponse {
            fn from(value: super::GetCountryResponse) -> Self {
                Self {
                    provider: Ok(value.provider),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetCredentialsResponse {
            items: Result<Vec<super::Credential>, String>,
            total_count: Result<Option<f64>, String>,
        }

        impl Default for GetCredentialsResponse {
            fn default() -> Self {
                Self {
                    items: Ok(Default::default()),
                    total_count: Ok(Default::default()),
                }
            }
        }

        impl GetCredentialsResponse {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Credential>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn total_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.total_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_count: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetCredentialsResponse> for super::GetCredentialsResponse {
            type Error = String;
            fn try_from(value: GetCredentialsResponse) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    total_count: value.total_count?,
                })
            }
        }

        impl From<super::GetCredentialsResponse> for GetCredentialsResponse {
            fn from(value: super::GetCredentialsResponse) -> Self {
                Self {
                    items: Ok(value.items),
                    total_count: Ok(value.total_count),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetDeviceResponse {
            device: Result<std::collections::HashMap<String, super::ProviderDeviceEvents>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for GetDeviceResponse {
            fn default() -> Self {
                Self {
                    device: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl GetDeviceResponse {
            pub fn device<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<
                    std::collections::HashMap<String, super::ProviderDeviceEvents>,
                >,
                T::Error: std::fmt::Display,
            {
                self.device = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for device: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetDeviceResponse> for super::GetDeviceResponse {
            type Error = String;
            fn try_from(value: GetDeviceResponse) -> Result<Self, String> {
                Ok(Self {
                    device: value.device?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::GetDeviceResponse> for GetDeviceResponse {
            fn from(value: super::GetDeviceResponse) -> Self {
                Self {
                    device: Ok(value.device),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetDomainTagsItem {
            description: Result<Option<String>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for GetDomainTagsItem {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl GetDomainTagsItem {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetDomainTagsItem> for super::GetDomainTagsItem {
            type Error = String;
            fn try_from(value: GetDomainTagsItem) -> Result<Self, String> {
                Ok(Self {
                    description: value.description?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::GetDomainTagsItem> for GetDomainTagsItem {
            fn from(value: super::GetDomainTagsItem) -> Self {
                Self {
                    description: Ok(value.description),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetDomainsResponse {
            items: Result<Vec<super::Domain>, String>,
            total_count: Result<Option<f64>, String>,
        }

        impl Default for GetDomainsResponse {
            fn default() -> Self {
                Self {
                    items: Ok(Default::default()),
                    total_count: Ok(Default::default()),
                }
            }
        }

        impl GetDomainsResponse {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Domain>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn total_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<f64>>,
                T::Error: std::fmt::Display,
            {
                self.total_count = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_count: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetDomainsResponse> for super::GetDomainsResponse {
            type Error = String;
            fn try_from(value: GetDomainsResponse) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    total_count: value.total_count?,
                })
            }
        }

        impl From<super::GetDomainsResponse> for GetDomainsResponse {
            fn from(value: super::GetDomainsResponse) -> Self {
                Self {
                    items: Ok(value.items),
                    total_count: Ok(value.total_count),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetProvidersEvents {
            provider: Result<std::collections::HashMap<String, super::ProviderEvents>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for GetProvidersEvents {
            fn default() -> Self {
                Self {
                    provider: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl GetProvidersEvents {
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, super::ProviderEvents>>,
                T::Error: std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetProvidersEvents> for super::GetProvidersEvents {
            type Error = String;
            fn try_from(value: GetProvidersEvents) -> Result<Self, String> {
                Ok(Self {
                    provider: value.provider?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::GetProvidersEvents> for GetProvidersEvents {
            fn from(value: super::GetProvidersEvents) -> Self {
                Self {
                    provider: Ok(value.provider),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetTrackingResponse {
            click: Result<Option<super::GetTrackingResponseClick>, String>,
            open: Result<Option<super::GetTrackingResponseOpen>, String>,
            unsubscribe: Result<Option<super::GetTrackingResponseUnsubscribe>, String>,
        }

        impl Default for GetTrackingResponse {
            fn default() -> Self {
                Self {
                    click: Ok(Default::default()),
                    open: Ok(Default::default()),
                    unsubscribe: Ok(Default::default()),
                }
            }
        }

        impl GetTrackingResponse {
            pub fn click<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::GetTrackingResponseClick>>,
                T::Error: std::fmt::Display,
            {
                self.click = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for click: {}", e));
                self
            }
            pub fn open<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::GetTrackingResponseOpen>>,
                T::Error: std::fmt::Display,
            {
                self.open = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open: {}", e));
                self
            }
            pub fn unsubscribe<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::GetTrackingResponseUnsubscribe>>,
                T::Error: std::fmt::Display,
            {
                self.unsubscribe = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for unsubscribe: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetTrackingResponse> for super::GetTrackingResponse {
            type Error = String;
            fn try_from(value: GetTrackingResponse) -> Result<Self, String> {
                Ok(Self {
                    click: value.click?,
                    open: value.open?,
                    unsubscribe: value.unsubscribe?,
                })
            }
        }

        impl From<super::GetTrackingResponse> for GetTrackingResponse {
            fn from(value: super::GetTrackingResponse) -> Self {
                Self {
                    click: Ok(value.click),
                    open: Ok(value.open),
                    unsubscribe: Ok(value.unsubscribe),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetTrackingResponseClick {
            active: Result<Option<bool>, String>,
        }

        impl Default for GetTrackingResponseClick {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                }
            }
        }

        impl GetTrackingResponseClick {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetTrackingResponseClick> for super::GetTrackingResponseClick {
            type Error = String;
            fn try_from(value: GetTrackingResponseClick) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                })
            }
        }

        impl From<super::GetTrackingResponseClick> for GetTrackingResponseClick {
            fn from(value: super::GetTrackingResponseClick) -> Self {
                Self {
                    active: Ok(value.active),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetTrackingResponseOpen {
            active: Result<Option<bool>, String>,
        }

        impl Default for GetTrackingResponseOpen {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                }
            }
        }

        impl GetTrackingResponseOpen {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetTrackingResponseOpen> for super::GetTrackingResponseOpen {
            type Error = String;
            fn try_from(value: GetTrackingResponseOpen) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                })
            }
        }

        impl From<super::GetTrackingResponseOpen> for GetTrackingResponseOpen {
            fn from(value: super::GetTrackingResponseOpen) -> Self {
                Self {
                    active: Ok(value.active),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetTrackingResponseUnsubscribe {
            active: Result<Option<bool>, String>,
        }

        impl Default for GetTrackingResponseUnsubscribe {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                }
            }
        }

        impl GetTrackingResponseUnsubscribe {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetTrackingResponseUnsubscribe>
            for super::GetTrackingResponseUnsubscribe
        {
            type Error = String;
            fn try_from(value: GetTrackingResponseUnsubscribe) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                })
            }
        }

        impl From<super::GetTrackingResponseUnsubscribe> for GetTrackingResponseUnsubscribe {
            fn from(value: super::GetTrackingResponseUnsubscribe) -> Self {
                Self {
                    active: Ok(value.active),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListMemberAddressResponse {
            items: Result<Vec<super::Member>, String>,
        }

        impl Default for ListMemberAddressResponse {
            fn default() -> Self {
                Self {
                    items: Ok(Default::default()),
                }
            }
        }

        impl ListMemberAddressResponse {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Member>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListMemberAddressResponse> for super::ListMemberAddressResponse {
            type Error = String;
            fn try_from(value: ListMemberAddressResponse) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                })
            }
        }

        impl From<super::ListMemberAddressResponse> for ListMemberAddressResponse {
            fn from(value: super::ListMemberAddressResponse) -> Self {
                Self {
                    items: Ok(value.items),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListMembersResponse {
            items: Result<Vec<super::Member>, String>,
            paging: Result<Option<super::ListMembersResponsePaging>, String>,
        }

        impl Default for ListMembersResponse {
            fn default() -> Self {
                Self {
                    items: Ok(Default::default()),
                    paging: Ok(Default::default()),
                }
            }
        }

        impl ListMembersResponse {
            pub fn items<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Member>>,
                T::Error: std::fmt::Display,
            {
                self.items = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for items: {}", e));
                self
            }
            pub fn paging<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::ListMembersResponsePaging>>,
                T::Error: std::fmt::Display,
            {
                self.paging = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for paging: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListMembersResponse> for super::ListMembersResponse {
            type Error = String;
            fn try_from(value: ListMembersResponse) -> Result<Self, String> {
                Ok(Self {
                    items: value.items?,
                    paging: value.paging?,
                })
            }
        }

        impl From<super::ListMembersResponse> for ListMembersResponse {
            fn from(value: super::ListMembersResponse) -> Self {
                Self {
                    items: Ok(value.items),
                    paging: Ok(value.paging),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListMembersResponsePaging {
            first: Result<Option<String>, String>,
            last: Result<Option<String>, String>,
            next: Result<Option<String>, String>,
            previous: Result<Option<String>, String>,
        }

        impl Default for ListMembersResponsePaging {
            fn default() -> Self {
                Self {
                    first: Ok(Default::default()),
                    last: Ok(Default::default()),
                    next: Ok(Default::default()),
                    previous: Ok(Default::default()),
                }
            }
        }

        impl ListMembersResponsePaging {
            pub fn first<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.first = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first: {}", e));
                self
            }
            pub fn last<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.last = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last: {}", e));
                self
            }
            pub fn next<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.next = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for next: {}", e));
                self
            }
            pub fn previous<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.previous = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for previous: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListMembersResponsePaging> for super::ListMembersResponsePaging {
            type Error = String;
            fn try_from(value: ListMembersResponsePaging) -> Result<Self, String> {
                Ok(Self {
                    first: value.first?,
                    last: value.last?,
                    next: value.next?,
                    previous: value.previous?,
                })
            }
        }

        impl From<super::ListMembersResponsePaging> for ListMembersResponsePaging {
            fn from(value: super::ListMembersResponsePaging) -> Self {
                Self {
                    first: Ok(value.first),
                    last: Ok(value.last),
                    next: Ok(value.next),
                    previous: Ok(value.previous),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Member {
            address: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            subscribed: Result<Option<bool>, String>,
            vars: Result<std::collections::HashMap<String, String>, String>,
        }

        impl Default for Member {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    name: Ok(Default::default()),
                    subscribed: Ok(Default::default()),
                    vars: Ok(Default::default()),
                }
            }
        }

        impl Member {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn subscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.subscribed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subscribed: {}", e));
                self
            }
            pub fn vars<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<std::collections::HashMap<String, String>>,
                T::Error: std::fmt::Display,
            {
                self.vars = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vars: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Member> for super::Member {
            type Error = String;
            fn try_from(value: Member) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    name: value.name?,
                    subscribed: value.subscribed?,
                    vars: value.vars?,
                })
            }
        }

        impl From<super::Member> for Member {
            fn from(value: super::Member) -> Self {
                Self {
                    address: Ok(value.address),
                    name: Ok(value.name),
                    subscribed: Ok(value.subscribed),
                    vars: Ok(value.vars),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ProviderCountryEvents {
            accepted: Result<Option<i64>, String>,
            clicked: Result<Option<i64>, String>,
            complained: Result<Option<i64>, String>,
            delivered: Result<Option<i64>, String>,
            opened: Result<Option<i64>, String>,
            unique_clicked: Result<Option<i64>, String>,
            unique_opened: Result<Option<i64>, String>,
            unsubscribed: Result<Option<i64>, String>,
        }

        impl Default for ProviderCountryEvents {
            fn default() -> Self {
                Self {
                    accepted: Ok(Default::default()),
                    clicked: Ok(Default::default()),
                    complained: Ok(Default::default()),
                    delivered: Ok(Default::default()),
                    opened: Ok(Default::default()),
                    unique_clicked: Ok(Default::default()),
                    unique_opened: Ok(Default::default()),
                    unsubscribed: Ok(Default::default()),
                }
            }
        }

        impl ProviderCountryEvents {
            pub fn accepted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.accepted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for accepted: {}", e));
                self
            }
            pub fn clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.clicked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for clicked: {}", e));
                self
            }
            pub fn complained<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.complained = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for complained: {}", e));
                self
            }
            pub fn delivered<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.delivered = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for delivered: {}", e));
                self
            }
            pub fn opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.opened = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opened: {}", e));
                self
            }
            pub fn unique_clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_clicked = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_clicked: {}", e)
                });
                self
            }
            pub fn unique_opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_opened = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_opened: {}", e)
                });
                self
            }
            pub fn unsubscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unsubscribed = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsubscribed: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<ProviderCountryEvents> for super::ProviderCountryEvents {
            type Error = String;
            fn try_from(value: ProviderCountryEvents) -> Result<Self, String> {
                Ok(Self {
                    accepted: value.accepted?,
                    clicked: value.clicked?,
                    complained: value.complained?,
                    delivered: value.delivered?,
                    opened: value.opened?,
                    unique_clicked: value.unique_clicked?,
                    unique_opened: value.unique_opened?,
                    unsubscribed: value.unsubscribed?,
                })
            }
        }

        impl From<super::ProviderCountryEvents> for ProviderCountryEvents {
            fn from(value: super::ProviderCountryEvents) -> Self {
                Self {
                    accepted: Ok(value.accepted),
                    clicked: Ok(value.clicked),
                    complained: Ok(value.complained),
                    delivered: Ok(value.delivered),
                    opened: Ok(value.opened),
                    unique_clicked: Ok(value.unique_clicked),
                    unique_opened: Ok(value.unique_opened),
                    unsubscribed: Ok(value.unsubscribed),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ProviderDeviceEvents {
            clicked: Result<Option<i64>, String>,
            complained: Result<Option<i64>, String>,
            opened: Result<Option<i64>, String>,
            unique_clicked: Result<Option<i64>, String>,
            unique_opened: Result<Option<i64>, String>,
            unsubscribed: Result<Option<i64>, String>,
        }

        impl Default for ProviderDeviceEvents {
            fn default() -> Self {
                Self {
                    clicked: Ok(Default::default()),
                    complained: Ok(Default::default()),
                    opened: Ok(Default::default()),
                    unique_clicked: Ok(Default::default()),
                    unique_opened: Ok(Default::default()),
                    unsubscribed: Ok(Default::default()),
                }
            }
        }

        impl ProviderDeviceEvents {
            pub fn clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.clicked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for clicked: {}", e));
                self
            }
            pub fn complained<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.complained = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for complained: {}", e));
                self
            }
            pub fn opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.opened = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opened: {}", e));
                self
            }
            pub fn unique_clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_clicked = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_clicked: {}", e)
                });
                self
            }
            pub fn unique_opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_opened = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_opened: {}", e)
                });
                self
            }
            pub fn unsubscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unsubscribed = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsubscribed: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<ProviderDeviceEvents> for super::ProviderDeviceEvents {
            type Error = String;
            fn try_from(value: ProviderDeviceEvents) -> Result<Self, String> {
                Ok(Self {
                    clicked: value.clicked?,
                    complained: value.complained?,
                    opened: value.opened?,
                    unique_clicked: value.unique_clicked?,
                    unique_opened: value.unique_opened?,
                    unsubscribed: value.unsubscribed?,
                })
            }
        }

        impl From<super::ProviderDeviceEvents> for ProviderDeviceEvents {
            fn from(value: super::ProviderDeviceEvents) -> Self {
                Self {
                    clicked: Ok(value.clicked),
                    complained: Ok(value.complained),
                    opened: Ok(value.opened),
                    unique_clicked: Ok(value.unique_clicked),
                    unique_opened: Ok(value.unique_opened),
                    unsubscribed: Ok(value.unsubscribed),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ProviderEvents {
            accepted: Result<Option<i64>, String>,
            clicked: Result<Option<i64>, String>,
            complained: Result<Option<i64>, String>,
            delivered: Result<Option<i64>, String>,
            opened: Result<Option<i64>, String>,
            unique_clicked: Result<Option<i64>, String>,
            unique_opened: Result<Option<i64>, String>,
            unsubscribed: Result<Option<i64>, String>,
        }

        impl Default for ProviderEvents {
            fn default() -> Self {
                Self {
                    accepted: Ok(Default::default()),
                    clicked: Ok(Default::default()),
                    complained: Ok(Default::default()),
                    delivered: Ok(Default::default()),
                    opened: Ok(Default::default()),
                    unique_clicked: Ok(Default::default()),
                    unique_opened: Ok(Default::default()),
                    unsubscribed: Ok(Default::default()),
                }
            }
        }

        impl ProviderEvents {
            pub fn accepted<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.accepted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for accepted: {}", e));
                self
            }
            pub fn clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.clicked = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for clicked: {}", e));
                self
            }
            pub fn complained<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.complained = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for complained: {}", e));
                self
            }
            pub fn delivered<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.delivered = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for delivered: {}", e));
                self
            }
            pub fn opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.opened = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for opened: {}", e));
                self
            }
            pub fn unique_clicked<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_clicked = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_clicked: {}", e)
                });
                self
            }
            pub fn unique_opened<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unique_opened = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unique_opened: {}", e)
                });
                self
            }
            pub fn unsubscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.unsubscribed = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsubscribed: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<ProviderEvents> for super::ProviderEvents {
            type Error = String;
            fn try_from(value: ProviderEvents) -> Result<Self, String> {
                Ok(Self {
                    accepted: value.accepted?,
                    clicked: value.clicked?,
                    complained: value.complained?,
                    delivered: value.delivered?,
                    opened: value.opened?,
                    unique_clicked: value.unique_clicked?,
                    unique_opened: value.unique_opened?,
                    unsubscribed: value.unsubscribed?,
                })
            }
        }

        impl From<super::ProviderEvents> for ProviderEvents {
            fn from(value: super::ProviderEvents) -> Self {
                Self {
                    accepted: Ok(value.accepted),
                    clicked: Ok(value.clicked),
                    complained: Ok(value.complained),
                    delivered: Ok(value.delivered),
                    opened: Ok(value.opened),
                    unique_clicked: Ok(value.unique_clicked),
                    unique_opened: Ok(value.unique_opened),
                    unsubscribed: Ok(value.unsubscribed),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReceivingDnsRecord {
            cached: Result<Vec<serde_json::Value>, String>,
            priority: Result<Option<String>, String>,
            record_type: Result<Option<String>, String>,
            valid: Result<Option<String>, String>,
            value: Result<Option<String>, String>,
        }

        impl Default for ReceivingDnsRecord {
            fn default() -> Self {
                Self {
                    cached: Ok(Default::default()),
                    priority: Ok(Default::default()),
                    record_type: Ok(Default::default()),
                    valid: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl ReceivingDnsRecord {
            pub fn cached<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.cached = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cached: {}", e));
                self
            }
            pub fn priority<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.priority = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for priority: {}", e));
                self
            }
            pub fn record_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.record_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for record_type: {}", e));
                self
            }
            pub fn valid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.valid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for valid: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ReceivingDnsRecord> for super::ReceivingDnsRecord {
            type Error = String;
            fn try_from(value: ReceivingDnsRecord) -> Result<Self, String> {
                Ok(Self {
                    cached: value.cached?,
                    priority: value.priority?,
                    record_type: value.record_type?,
                    valid: value.valid?,
                    value: value.value?,
                })
            }
        }

        impl From<super::ReceivingDnsRecord> for ReceivingDnsRecord {
            fn from(value: super::ReceivingDnsRecord) -> Self {
                Self {
                    cached: Ok(value.cached),
                    priority: Ok(value.priority),
                    record_type: Ok(value.record_type),
                    valid: Ok(value.valid),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SendMessageBody {
            attachment: Result<Option<String>, String>,
            bcc: Result<Option<String>, String>,
            cc: Result<Option<String>, String>,
            from: Result<Option<String>, String>,
            h_x_my_header: Result<Option<String>, String>,
            html: Result<Option<String>, String>,
            inline: Result<Option<String>, String>,
            o_deliverytime: Result<Option<String>, String>,
            o_deliverytime_optimize_period: Result<Option<String>, String>,
            o_dkim: Result<Option<String>, String>,
            o_require_tls: Result<Option<String>, String>,
            o_skip_verification: Result<Option<String>, String>,
            o_tag: Result<Option<String>, String>,
            o_testmode: Result<Option<String>, String>,
            o_time_zone_localize: Result<Option<String>, String>,
            o_tracking: Result<Option<String>, String>,
            o_tracking_clicks: Result<Option<String>, String>,
            o_tracking_opens: Result<Option<String>, String>,
            o_tracking_pixel_location_top: Result<Option<String>, String>,
            subject: Result<Option<String>, String>,
            t_text: Result<Option<bool>, String>,
            t_variables: Result<Option<String>, String>,
            t_version: Result<Option<String>, String>,
            template: Result<Option<String>, String>,
            text: Result<Option<String>, String>,
            to: Result<Option<String>, String>,
            v_my_var: Result<Option<String>, String>,
        }

        impl Default for SendMessageBody {
            fn default() -> Self {
                Self {
                    attachment: Ok(Default::default()),
                    bcc: Ok(Default::default()),
                    cc: Ok(Default::default()),
                    from: Ok(Default::default()),
                    h_x_my_header: Ok(Default::default()),
                    html: Ok(Default::default()),
                    inline: Ok(Default::default()),
                    o_deliverytime: Ok(Default::default()),
                    o_deliverytime_optimize_period: Ok(Default::default()),
                    o_dkim: Ok(Default::default()),
                    o_require_tls: Ok(Default::default()),
                    o_skip_verification: Ok(Default::default()),
                    o_tag: Ok(Default::default()),
                    o_testmode: Ok(Default::default()),
                    o_time_zone_localize: Ok(Default::default()),
                    o_tracking: Ok(Default::default()),
                    o_tracking_clicks: Ok(Default::default()),
                    o_tracking_opens: Ok(Default::default()),
                    o_tracking_pixel_location_top: Ok(Default::default()),
                    subject: Ok(Default::default()),
                    t_text: Ok(Default::default()),
                    t_variables: Ok(Default::default()),
                    t_version: Ok(Default::default()),
                    template: Ok(Default::default()),
                    text: Ok(Default::default()),
                    to: Ok(Default::default()),
                    v_my_var: Ok(Default::default()),
                }
            }
        }

        impl SendMessageBody {
            pub fn attachment<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.attachment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attachment: {}", e));
                self
            }
            pub fn bcc<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.bcc = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bcc: {}", e));
                self
            }
            pub fn cc<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.cc = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cc: {}", e));
                self
            }
            pub fn from<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.from = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for from: {}", e));
                self
            }
            pub fn h_x_my_header<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.h_x_my_header = value.try_into().map_err(|e| {
                    format!("error converting supplied value for h_x_my_header: {}", e)
                });
                self
            }
            pub fn html<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.html = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for html: {}", e));
                self
            }
            pub fn inline<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.inline = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for inline: {}", e));
                self
            }
            pub fn o_deliverytime<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_deliverytime = value.try_into().map_err(|e| {
                    format!("error converting supplied value for o_deliverytime: {}", e)
                });
                self
            }
            pub fn o_deliverytime_optimize_period<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_deliverytime_optimize_period = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_deliverytime_optimize_period: {}",
                        e
                    )
                });
                self
            }
            pub fn o_dkim<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_dkim = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for o_dkim: {}", e));
                self
            }
            pub fn o_require_tls<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_require_tls = value.try_into().map_err(|e| {
                    format!("error converting supplied value for o_require_tls: {}", e)
                });
                self
            }
            pub fn o_skip_verification<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_skip_verification = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_skip_verification: {}",
                        e
                    )
                });
                self
            }
            pub fn o_tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for o_tag: {}", e));
                self
            }
            pub fn o_testmode<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_testmode = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for o_testmode: {}", e));
                self
            }
            pub fn o_time_zone_localize<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_time_zone_localize = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_time_zone_localize: {}",
                        e
                    )
                });
                self
            }
            pub fn o_tracking<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_tracking = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for o_tracking: {}", e));
                self
            }
            pub fn o_tracking_clicks<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_tracking_clicks = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_tracking_clicks: {}",
                        e
                    )
                });
                self
            }
            pub fn o_tracking_opens<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_tracking_opens = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_tracking_opens: {}",
                        e
                    )
                });
                self
            }
            pub fn o_tracking_pixel_location_top<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.o_tracking_pixel_location_top = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for o_tracking_pixel_location_top: {}",
                        e
                    )
                });
                self
            }
            pub fn subject<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.subject = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subject: {}", e));
                self
            }
            pub fn t_text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.t_text = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for t_text: {}", e));
                self
            }
            pub fn t_variables<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.t_variables = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for t_variables: {}", e));
                self
            }
            pub fn t_version<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.t_version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for t_version: {}", e));
                self
            }
            pub fn template<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.template = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for template: {}", e));
                self
            }
            pub fn text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.text = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for text: {}", e));
                self
            }
            pub fn to<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.to = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for to: {}", e));
                self
            }
            pub fn v_my_var<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.v_my_var = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for v_my_var: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<SendMessageBody> for super::SendMessageBody {
            type Error = String;
            fn try_from(value: SendMessageBody) -> Result<Self, String> {
                Ok(Self {
                    attachment: value.attachment?,
                    bcc: value.bcc?,
                    cc: value.cc?,
                    from: value.from?,
                    h_x_my_header: value.h_x_my_header?,
                    html: value.html?,
                    inline: value.inline?,
                    o_deliverytime: value.o_deliverytime?,
                    o_deliverytime_optimize_period: value.o_deliverytime_optimize_period?,
                    o_dkim: value.o_dkim?,
                    o_require_tls: value.o_require_tls?,
                    o_skip_verification: value.o_skip_verification?,
                    o_tag: value.o_tag?,
                    o_testmode: value.o_testmode?,
                    o_time_zone_localize: value.o_time_zone_localize?,
                    o_tracking: value.o_tracking?,
                    o_tracking_clicks: value.o_tracking_clicks?,
                    o_tracking_opens: value.o_tracking_opens?,
                    o_tracking_pixel_location_top: value.o_tracking_pixel_location_top?,
                    subject: value.subject?,
                    t_text: value.t_text?,
                    t_variables: value.t_variables?,
                    t_version: value.t_version?,
                    template: value.template?,
                    text: value.text?,
                    to: value.to?,
                    v_my_var: value.v_my_var?,
                })
            }
        }

        impl From<super::SendMessageBody> for SendMessageBody {
            fn from(value: super::SendMessageBody) -> Self {
                Self {
                    attachment: Ok(value.attachment),
                    bcc: Ok(value.bcc),
                    cc: Ok(value.cc),
                    from: Ok(value.from),
                    h_x_my_header: Ok(value.h_x_my_header),
                    html: Ok(value.html),
                    inline: Ok(value.inline),
                    o_deliverytime: Ok(value.o_deliverytime),
                    o_deliverytime_optimize_period: Ok(value.o_deliverytime_optimize_period),
                    o_dkim: Ok(value.o_dkim),
                    o_require_tls: Ok(value.o_require_tls),
                    o_skip_verification: Ok(value.o_skip_verification),
                    o_tag: Ok(value.o_tag),
                    o_testmode: Ok(value.o_testmode),
                    o_time_zone_localize: Ok(value.o_time_zone_localize),
                    o_tracking: Ok(value.o_tracking),
                    o_tracking_clicks: Ok(value.o_tracking_clicks),
                    o_tracking_opens: Ok(value.o_tracking_opens),
                    o_tracking_pixel_location_top: Ok(value.o_tracking_pixel_location_top),
                    subject: Ok(value.subject),
                    t_text: Ok(value.t_text),
                    t_variables: Ok(value.t_variables),
                    t_version: Ok(value.t_version),
                    template: Ok(value.template),
                    text: Ok(value.text),
                    to: Ok(value.to),
                    v_my_var: Ok(value.v_my_var),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SendingDnsRecord {
            cached: Result<Vec<serde_json::Value>, String>,
            name: Result<Option<String>, String>,
            record_type: Result<Option<String>, String>,
            valid: Result<Option<String>, String>,
            value: Result<Option<String>, String>,
        }

        impl Default for SendingDnsRecord {
            fn default() -> Self {
                Self {
                    cached: Ok(Default::default()),
                    name: Ok(Default::default()),
                    record_type: Ok(Default::default()),
                    valid: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl SendingDnsRecord {
            pub fn cached<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.cached = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cached: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn record_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.record_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for record_type: {}", e));
                self
            }
            pub fn valid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.valid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for valid: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<SendingDnsRecord> for super::SendingDnsRecord {
            type Error = String;
            fn try_from(value: SendingDnsRecord) -> Result<Self, String> {
                Ok(Self {
                    cached: value.cached?,
                    name: value.name?,
                    record_type: value.record_type?,
                    valid: value.valid?,
                    value: value.value?,
                })
            }
        }

        impl From<super::SendingDnsRecord> for SendingDnsRecord {
            fn from(value: super::SendingDnsRecord) -> Self {
                Self {
                    cached: Ok(value.cached),
                    name: Ok(value.name),
                    record_type: Ok(value.record_type),
                    valid: Ok(value.valid),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct StoredMessage {
            attachments: Result<Vec<super::Attachment>, String>,
            body_html: Result<Option<String>, String>,
            body_plain: Result<Option<String>, String>,
            content_id_map: Result<serde_json::Map<String, serde_json::Value>, String>,
            from: Result<Option<String>, String>,
            message_headers: Result<Vec<String>, String>,
            recipients: Result<Option<String>, String>,
            sender: Result<Option<String>, String>,
            stripped_html: Result<Option<String>, String>,
            stripped_signature: Result<Option<String>, String>,
            stripped_text: Result<Option<String>, String>,
            subject: Result<Option<String>, String>,
        }

        impl Default for StoredMessage {
            fn default() -> Self {
                Self {
                    attachments: Ok(Default::default()),
                    body_html: Ok(Default::default()),
                    body_plain: Ok(Default::default()),
                    content_id_map: Ok(Default::default()),
                    from: Ok(Default::default()),
                    message_headers: Ok(Default::default()),
                    recipients: Ok(Default::default()),
                    sender: Ok(Default::default()),
                    stripped_html: Ok(Default::default()),
                    stripped_signature: Ok(Default::default()),
                    stripped_text: Ok(Default::default()),
                    subject: Ok(Default::default()),
                }
            }
        }

        impl StoredMessage {
            pub fn attachments<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Attachment>>,
                T::Error: std::fmt::Display,
            {
                self.attachments = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attachments: {}", e));
                self
            }
            pub fn body_html<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.body_html = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for body_html: {}", e));
                self
            }
            pub fn body_plain<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.body_plain = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for body_plain: {}", e));
                self
            }
            pub fn content_id_map<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
                T::Error: std::fmt::Display,
            {
                self.content_id_map = value.try_into().map_err(|e| {
                    format!("error converting supplied value for content_id_map: {}", e)
                });
                self
            }
            pub fn from<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.from = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for from: {}", e));
                self
            }
            pub fn message_headers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.message_headers = value.try_into().map_err(|e| {
                    format!("error converting supplied value for message_headers: {}", e)
                });
                self
            }
            pub fn recipients<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.recipients = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recipients: {}", e));
                self
            }
            pub fn sender<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.sender = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sender: {}", e));
                self
            }
            pub fn stripped_html<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.stripped_html = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stripped_html: {}", e)
                });
                self
            }
            pub fn stripped_signature<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.stripped_signature = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for stripped_signature: {}",
                        e
                    )
                });
                self
            }
            pub fn stripped_text<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.stripped_text = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stripped_text: {}", e)
                });
                self
            }
            pub fn subject<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.subject = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subject: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<StoredMessage> for super::StoredMessage {
            type Error = String;
            fn try_from(value: StoredMessage) -> Result<Self, String> {
                Ok(Self {
                    attachments: value.attachments?,
                    body_html: value.body_html?,
                    body_plain: value.body_plain?,
                    content_id_map: value.content_id_map?,
                    from: value.from?,
                    message_headers: value.message_headers?,
                    recipients: value.recipients?,
                    sender: value.sender?,
                    stripped_html: value.stripped_html?,
                    stripped_signature: value.stripped_signature?,
                    stripped_text: value.stripped_text?,
                    subject: value.subject?,
                })
            }
        }

        impl From<super::StoredMessage> for StoredMessage {
            fn from(value: super::StoredMessage) -> Self {
                Self {
                    attachments: Ok(value.attachments),
                    body_html: Ok(value.body_html),
                    body_plain: Ok(value.body_plain),
                    content_id_map: Ok(value.content_id_map),
                    from: Ok(value.from),
                    message_headers: Ok(value.message_headers),
                    recipients: Ok(value.recipients),
                    sender: Ok(value.sender),
                    stripped_html: Ok(value.stripped_html),
                    stripped_signature: Ok(value.stripped_signature),
                    stripped_text: Ok(value.stripped_text),
                    subject: Ok(value.subject),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SuccessResponse {
            message: Result<Option<String>, String>,
        }

        impl Default for SuccessResponse {
            fn default() -> Self {
                Self {
                    message: Ok(Default::default()),
                }
            }
        }

        impl SuccessResponse {
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<SuccessResponse> for super::SuccessResponse {
            type Error = String;
            fn try_from(value: SuccessResponse) -> Result<Self, String> {
                Ok(Self {
                    message: value.message?,
                })
            }
        }

        impl From<super::SuccessResponse> for SuccessResponse {
            fn from(value: super::SuccessResponse) -> Self {
                Self {
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Unsubscribe {
            address: Result<String, String>,
            created_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            tags: Result<Vec<String>, String>,
        }

        impl Default for Unsubscribe {
            fn default() -> Self {
                Self {
                    address: Err("no value supplied for address".to_string()),
                    created_at: Ok(Default::default()),
                    tags: Ok(Default::default()),
                }
            }
        }

        impl Unsubscribe {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn tags<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.tags = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tags: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Unsubscribe> for super::Unsubscribe {
            type Error = String;
            fn try_from(value: Unsubscribe) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    created_at: value.created_at?,
                    tags: value.tags?,
                })
            }
        }

        impl From<super::Unsubscribe> for Unsubscribe {
            fn from(value: super::Unsubscribe) -> Self {
                Self {
                    address: Ok(value.address),
                    created_at: Ok(value.created_at),
                    tags: Ok(value.tags),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDkimAuthorityBody {
            self_: Result<Option<String>, String>,
        }

        impl Default for UpdateDkimAuthorityBody {
            fn default() -> Self {
                Self {
                    self_: Ok(Default::default()),
                }
            }
        }

        impl UpdateDkimAuthorityBody {
            pub fn self_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.self_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for self_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateDkimAuthorityBody> for super::UpdateDkimAuthorityBody {
            type Error = String;
            fn try_from(value: UpdateDkimAuthorityBody) -> Result<Self, String> {
                Ok(Self {
                    self_: value.self_?,
                })
            }
        }

        impl From<super::UpdateDkimAuthorityBody> for UpdateDkimAuthorityBody {
            fn from(value: super::UpdateDkimAuthorityBody) -> Self {
                Self {
                    self_: Ok(value.self_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDomainClickTrackingBody {
            active: Result<Option<String>, String>,
        }

        impl Default for UpdateDomainClickTrackingBody {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                }
            }
        }

        impl UpdateDomainClickTrackingBody {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateDomainClickTrackingBody> for super::UpdateDomainClickTrackingBody {
            type Error = String;
            fn try_from(value: UpdateDomainClickTrackingBody) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                })
            }
        }

        impl From<super::UpdateDomainClickTrackingBody> for UpdateDomainClickTrackingBody {
            fn from(value: super::UpdateDomainClickTrackingBody) -> Self {
                Self {
                    active: Ok(value.active),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDomainConnectionBody {
            require_tls: Result<Option<bool>, String>,
            skip_verification: Result<Option<bool>, String>,
        }

        impl Default for UpdateDomainConnectionBody {
            fn default() -> Self {
                Self {
                    require_tls: Ok(Default::default()),
                    skip_verification: Ok(Default::default()),
                }
            }
        }

        impl UpdateDomainConnectionBody {
            pub fn require_tls<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.require_tls = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for require_tls: {}", e));
                self
            }
            pub fn skip_verification<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.skip_verification = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for skip_verification: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<UpdateDomainConnectionBody> for super::UpdateDomainConnectionBody {
            type Error = String;
            fn try_from(value: UpdateDomainConnectionBody) -> Result<Self, String> {
                Ok(Self {
                    require_tls: value.require_tls?,
                    skip_verification: value.skip_verification?,
                })
            }
        }

        impl From<super::UpdateDomainConnectionBody> for UpdateDomainConnectionBody {
            fn from(value: super::UpdateDomainConnectionBody) -> Self {
                Self {
                    require_tls: Ok(value.require_tls),
                    skip_verification: Ok(value.skip_verification),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDomainTagBody {
            description: Result<Option<String>, String>,
            domain: Result<Option<String>, String>,
            tag: Result<Option<String>, String>,
        }

        impl Default for UpdateDomainTagBody {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    domain: Ok(Default::default()),
                    tag: Ok(Default::default()),
                }
            }
        }

        impl UpdateDomainTagBody {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn domain<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.domain = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for domain: {}", e));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateDomainTagBody> for super::UpdateDomainTagBody {
            type Error = String;
            fn try_from(value: UpdateDomainTagBody) -> Result<Self, String> {
                Ok(Self {
                    description: value.description?,
                    domain: value.domain?,
                    tag: value.tag?,
                })
            }
        }

        impl From<super::UpdateDomainTagBody> for UpdateDomainTagBody {
            fn from(value: super::UpdateDomainTagBody) -> Self {
                Self {
                    description: Ok(value.description),
                    domain: Ok(value.domain),
                    tag: Ok(value.tag),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDomainTrackingBody {
            active: Result<Option<String>, String>,
        }

        impl Default for UpdateDomainTrackingBody {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                }
            }
        }

        impl UpdateDomainTrackingBody {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateDomainTrackingBody> for super::UpdateDomainTrackingBody {
            type Error = String;
            fn try_from(value: UpdateDomainTrackingBody) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                })
            }
        }

        impl From<super::UpdateDomainTrackingBody> for UpdateDomainTrackingBody {
            fn from(value: super::UpdateDomainTrackingBody) -> Self {
                Self {
                    active: Ok(value.active),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateDomainUnsubscribeTrackingBody {
            active: Result<Option<String>, String>,
            html_footer: Result<Option<String>, String>,
            text_footer: Result<Option<String>, String>,
        }

        impl Default for UpdateDomainUnsubscribeTrackingBody {
            fn default() -> Self {
                Self {
                    active: Ok(Default::default()),
                    html_footer: Ok(Default::default()),
                    text_footer: Ok(Default::default()),
                }
            }
        }

        impl UpdateDomainUnsubscribeTrackingBody {
            pub fn active<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.active = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for active: {}", e));
                self
            }
            pub fn html_footer<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.html_footer = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for html_footer: {}", e));
                self
            }
            pub fn text_footer<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.text_footer = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for text_footer: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateDomainUnsubscribeTrackingBody>
            for super::UpdateDomainUnsubscribeTrackingBody
        {
            type Error = String;
            fn try_from(value: UpdateDomainUnsubscribeTrackingBody) -> Result<Self, String> {
                Ok(Self {
                    active: value.active?,
                    html_footer: value.html_footer?,
                    text_footer: value.text_footer?,
                })
            }
        }

        impl From<super::UpdateDomainUnsubscribeTrackingBody> for UpdateDomainUnsubscribeTrackingBody {
            fn from(value: super::UpdateDomainUnsubscribeTrackingBody) -> Self {
                Self {
                    active: Ok(value.active),
                    html_footer: Ok(value.html_footer),
                    text_footer: Ok(value.text_footer),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateListBody {
            access_level: Result<Option<String>, String>,
            address: Result<Option<String>, String>,
            description: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
        }

        impl Default for UpdateListBody {
            fn default() -> Self {
                Self {
                    access_level: Ok(Default::default()),
                    address: Ok(Default::default()),
                    description: Ok(Default::default()),
                    name: Ok(Default::default()),
                }
            }
        }

        impl UpdateListBody {
            pub fn access_level<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.access_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_level: {}", e)
                });
                self
            }
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateListBody> for super::UpdateListBody {
            type Error = String;
            fn try_from(value: UpdateListBody) -> Result<Self, String> {
                Ok(Self {
                    access_level: value.access_level?,
                    address: value.address?,
                    description: value.description?,
                    name: value.name?,
                })
            }
        }

        impl From<super::UpdateListBody> for UpdateListBody {
            fn from(value: super::UpdateListBody) -> Self {
                Self {
                    access_level: Ok(value.access_level),
                    address: Ok(value.address),
                    description: Ok(value.description),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateListMemberBody {
            address: Result<Option<String>, String>,
            name: Result<Option<String>, String>,
            subscribed: Result<Option<String>, String>,
            vars: Result<Option<String>, String>,
        }

        impl Default for UpdateListMemberBody {
            fn default() -> Self {
                Self {
                    address: Ok(Default::default()),
                    name: Ok(Default::default()),
                    subscribed: Ok(Default::default()),
                    vars: Ok(Default::default()),
                }
            }
        }

        impl UpdateListMemberBody {
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn subscribed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.subscribed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for subscribed: {}", e));
                self
            }
            pub fn vars<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.vars = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vars: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateListMemberBody> for super::UpdateListMemberBody {
            type Error = String;
            fn try_from(value: UpdateListMemberBody) -> Result<Self, String> {
                Ok(Self {
                    address: value.address?,
                    name: value.name?,
                    subscribed: value.subscribed?,
                    vars: value.vars?,
                })
            }
        }

        impl From<super::UpdateListMemberBody> for UpdateListMemberBody {
            fn from(value: super::UpdateListMemberBody) -> Self {
                Self {
                    address: Ok(value.address),
                    name: Ok(value.name),
                    subscribed: Ok(value.subscribed),
                    vars: Ok(value.vars),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateMailingList {
            list: Result<Option<super::UpdateMailingListList>, String>,
            message: Result<Option<String>, String>,
        }

        impl Default for UpdateMailingList {
            fn default() -> Self {
                Self {
                    list: Ok(Default::default()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl UpdateMailingList {
            pub fn list<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::UpdateMailingListList>>,
                T::Error: std::fmt::Display,
            {
                self.list = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for list: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateMailingList> for super::UpdateMailingList {
            type Error = String;
            fn try_from(value: UpdateMailingList) -> Result<Self, String> {
                Ok(Self {
                    list: value.list?,
                    message: value.message?,
                })
            }
        }

        impl From<super::UpdateMailingList> for UpdateMailingList {
            fn from(value: super::UpdateMailingList) -> Self {
                Self {
                    list: Ok(value.list),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateMailingListList {
            access_level: Result<Option<String>, String>,
            address: Result<Option<String>, String>,
            created_at: Result<Option<String>, String>,
            description: Result<Option<String>, String>,
            members_count: Result<Option<i64>, String>,
            name: Result<Option<String>, String>,
        }

        impl Default for UpdateMailingListList {
            fn default() -> Self {
                Self {
                    access_level: Ok(Default::default()),
                    address: Ok(Default::default()),
                    created_at: Ok(Default::default()),
                    description: Ok(Default::default()),
                    members_count: Ok(Default::default()),
                    name: Ok(Default::default()),
                }
            }
        }

        impl UpdateMailingListList {
            pub fn access_level<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.access_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_level: {}", e)
                });
                self
            }
            pub fn address<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.address = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for address: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn members_count<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i64>>,
                T::Error: std::fmt::Display,
            {
                self.members_count = value.try_into().map_err(|e| {
                    format!("error converting supplied value for members_count: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateMailingListList> for super::UpdateMailingListList {
            type Error = String;
            fn try_from(value: UpdateMailingListList) -> Result<Self, String> {
                Ok(Self {
                    access_level: value.access_level?,
                    address: value.address?,
                    created_at: value.created_at?,
                    description: value.description?,
                    members_count: value.members_count?,
                    name: value.name?,
                })
            }
        }

        impl From<super::UpdateMailingListList> for UpdateMailingListList {
            fn from(value: super::UpdateMailingListList) -> Self {
                Self {
                    access_level: Ok(value.access_level),
                    address: Ok(value.address),
                    created_at: Ok(value.created_at),
                    description: Ok(value.description),
                    members_count: Ok(value.members_count),
                    name: Ok(value.name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateRouteBody {
            action: Result<Option<String>, String>,
            description: Result<Option<String>, String>,
            expression: Result<Option<String>, String>,
            id: Result<Option<String>, String>,
            priority: Result<Option<String>, String>,
        }

        impl Default for UpdateRouteBody {
            fn default() -> Self {
                Self {
                    action: Ok(Default::default()),
                    description: Ok(Default::default()),
                    expression: Ok(Default::default()),
                    id: Ok(Default::default()),
                    priority: Ok(Default::default()),
                }
            }
        }

        impl UpdateRouteBody {
            pub fn action<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.action = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for action: {}", e));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {}", e));
                self
            }
            pub fn expression<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.expression = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expression: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn priority<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.priority = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for priority: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateRouteBody> for super::UpdateRouteBody {
            type Error = String;
            fn try_from(value: UpdateRouteBody) -> Result<Self, String> {
                Ok(Self {
                    action: value.action?,
                    description: value.description?,
                    expression: value.expression?,
                    id: value.id?,
                    priority: value.priority?,
                })
            }
        }

        impl From<super::UpdateRouteBody> for UpdateRouteBody {
            fn from(value: super::UpdateRouteBody) -> Self {
                Self {
                    action: Ok(value.action),
                    description: Ok(value.description),
                    expression: Ok(value.expression),
                    id: Ok(value.id),
                    priority: Ok(value.priority),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateSmtpCredentialsBody {
            password: Result<Option<String>, String>,
        }

        impl Default for UpdateSmtpCredentialsBody {
            fn default() -> Self {
                Self {
                    password: Ok(Default::default()),
                }
            }
        }

        impl UpdateSmtpCredentialsBody {
            pub fn password<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.password = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for password: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateSmtpCredentialsBody> for super::UpdateSmtpCredentialsBody {
            type Error = String;
            fn try_from(value: UpdateSmtpCredentialsBody) -> Result<Self, String> {
                Ok(Self {
                    password: value.password?,
                })
            }
        }

        impl From<super::UpdateSmtpCredentialsBody> for UpdateSmtpCredentialsBody {
            fn from(value: super::UpdateSmtpCredentialsBody) -> Self {
                Self {
                    password: Ok(value.password),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UpdateWebhookBody {
            domain: Result<Option<String>, String>,
            url: Result<Option<String>, String>,
            webhookname: Result<Option<String>, String>,
        }

        impl Default for UpdateWebhookBody {
            fn default() -> Self {
                Self {
                    domain: Ok(Default::default()),
                    url: Ok(Default::default()),
                    webhookname: Ok(Default::default()),
                }
            }
        }

        impl UpdateWebhookBody {
            pub fn domain<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.domain = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for domain: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
            pub fn webhookname<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.webhookname = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for webhookname: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<UpdateWebhookBody> for super::UpdateWebhookBody {
            type Error = String;
            fn try_from(value: UpdateWebhookBody) -> Result<Self, String> {
                Ok(Self {
                    domain: value.domain?,
                    url: value.url?,
                    webhookname: value.webhookname?,
                })
            }
        }

        impl From<super::UpdateWebhookBody> for UpdateWebhookBody {
            fn from(value: super::UpdateWebhookBody) -> Self {
                Self {
                    domain: Ok(value.domain),
                    url: Ok(value.url),
                    webhookname: Ok(value.webhookname),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Mailgun API
///
///Introduction
///=======
///
///The Mailgun API is built on HTTP. Our API is RESTful and it:
///
/// * Uses predictable, resource-oriented URLs.
/// * Uses built-in HTTP capabilities for passing parameters and authentication.
/// * Responds with standard HTTP response codes to indicate errors.
/// * Returns JSON.
///
///Mailgun has published [Libraries](https://documentation.mailgun.com/en/latest/libraries.html#libraries) for various languages. You may use our libraries, or your favorite HTTP/REST library available for your programming language, to make HTTP calls to Mailgun. Visit our Libraries page to see HTTP REST libraries we recommend.
///
///To give you an idea of how to use the API, we have annotated our
/// documentation with code samples written in several popular programming
/// languages. Use the language selector at the top to switch between them.
///
///Our samples from [quickstart](https://documentation.mailgun.com/en/latest/quickstart.html), [User Manual](https://documentation.mailgun.com/en/latest/user_manual.html#user-manual), and [API Reference](https://documentation.mailgun.com/en/latest/api_reference.html#api-reference) provide examples that will function. You’re welcome to copy/paste and run the script to see the API in action.
///
///## Base URL
///
///All API URLs referenced in this documentation start with the following base
/// part:
/// > https://api.mailgun.net/v3
///
///Your Mailgun account may contain several email domains. To avoid passing the
/// domain name as a query parameter, most API URLs must include the name of the
/// domain you’re interested in:
/// > https://api.mailgun.net/v3/domain.com
///
///## Authentication
///
///When you sign up for an account, you are given an API key. You authenticate
/// to the Mailgun API by providing your API key in the request. You can manage
/// your API key in the “Security” tab under the Account section of the Control
/// Panel.
///
///Authentication to the API occurs via [HTTP Basic Auth](http://en.wikipedia.org/wiki/Basic_access_authentication). Use `api` as the user name and your API key is the password. Here is how you use basic HTTP auth with curl:
/// > curl --user 'api:key-3ax6xnjp29jd6fds4gc373sgvjxteol0'
///
///Or you can try the following API call right from your browser:
/// > https://api:key-3ax6xnjp29jd6fds4gc373sgvjxteol0@api.mailgun.net/v3/samples.mailgun.org/log
///
///**Warning:** Keep your API key secret!
///
///## Date Format
///
///Mailgun returns JSON for all API calls. JSON does not have a built-in date type, dates are passed as strings encoded according to [RFC 2822#page-14](https://tools.ietf.org/html/rfc2822.html#page-14). This format is native to JavaScript and is also supported by most programming languages out of the box:
///
/// > 'Thu, 13 Oct 2011 18:02:00 GMT'
///
///## Errors
///
///| Code               | Description                                               |
///|--------------------|-----------------------------------------------------------|
///| 200                | Everything worked as expected                             |
///| 400                | Bad Request - Often missing a required parameter          |
///| 401                | Unauthorized - No valid API key provided                  |
///| 402                | Request Failed - Parameters were valid but request failed |
///| 404                | Not Found - The requested item doesn’t exist              |
///| 500, 502, 503, 504 | Server Errors - something is wrong on Mailgun’s end       |
///
///## Webhooks
///
///Mailgun can also POST data to your application when events (opens, clicks, bounces, etc.) occur or when you use Routes. You can read more about webhooks and [Routes](https://documentation.mailgun.com/en/latest/user_manual.html#um-routes) in the [User Manual](https://documentation.mailgun.com/en/latest/user_manual.html#user-manual).
///
///## Mailgun Regions
///
///Using a single account and billing plan, you can choose to provision new
/// sending domains in the EU environment. Message data never leaves the region
/// in which it is processed. Only a limited amount of account data is
/// replicated globally, giving you a single account from which to manage
/// domains in both the US and the EU. Here are the specifics on the type of
/// data that is replicated globally versus what is region-bound.
///
///| Global                                                                                                  | Region-Bound (US / EU)                                                                                                             |
///|---------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|
///| Account Information, User Accounts, Billing Details (invoices/plan information), API Keys, Domain Names | Domain Metadata (e.g. SMTP credentials), Messages, Event Logs, Suppressions, Mailing Lists, Tags, Statistics, Routes, IP Addresses |
///
///The endpoints you will use for sending/receiving/tracking messages in the EU
/// are below:
///
///| Service                      | US Endpoint      | EU Endpoint         |
///|------------------------------|------------------|---------------------|
///| REST API                     | api.mailgun.net  | api.eu.mailgun.net  |
///| Outgoing SMTP Server         | smtp.mailgun.org | smtp.eu.mailgun.org |
///| Inbound SMTP Server (Routes) | mxa.mailgun.org  | mxa.eu.mailgun.org  |
///| Inbound SMTP Server (Routes) | mxb.mailgun.org  | mxb.eu.mailgun.org  |
///| Open/Click Tracking Endpoint | mailgun.org      | eu.mailgun.org      |
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}

pub trait ClientDomainsExt {
    ///Get domains
    ///
    ///Returns a list of domains under your account in JSON. See examples.
    ///
    ///| Parameter | Description                                           |
    ///|-----------|-------------------------------------------------------|
    ///| limit     | Maximum number of records to return. (100 by default) |
    ///| skip      | Number of records to skip. (0 by default)             |
    ///
    ///Sends a `GET` request to `/domains`
    ///
    ///```ignore
    /// let response = client.get_domains()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domains(&self) -> builder::GetDomains;
    ///Create new domain
    ///
    ///Create a new domain.
    ///
    ///| Parameter            | Description                                                                                                                                                                                                                                                                                       |
    ///|----------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| name                 | Name of the domain (ex. domain.com)                                                                                                                                                                                                                                                               |
    ///| smtp_password        | Password for SMTP authentication                                                                                                                                                                                                                                                                  |
    ///| spam_action          | `disabled`, `block`, or `tag`  If disabled, no spam filtering will occur for inbound messages.  If block, inbound spam messages will not be delivered.  If tag, inbound messages will be tagged with a spam header. See [Spam Filter](https://documentation.mailgun.com/en/latest/user_manual.html#um-spam-filter).  The default is `disabled`.                                          |
    ///| wildcard             | true or false Determines whether the domain will accept email for sub-domains.  The default is false.                                                                                                                                                                                             |
    ///| force_dkim_authority | true or false  If set to true, the domain will be the DKIM authority for itself even if the root domain is registered on the same mailgun account  If set to false, the domain will have the same DKIM authority as the root domain registered on the same mailgun account  The default is false. |
    ///
    ///Sends a `POST` request to `/domains`
    ///
    ///```ignore
    /// let response = client.create_domain()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_domain(&self) -> builder::CreateDomain;
    ///Get single domain
    ///
    ///Returns a single domain, including credentials and DNS records.
    ///
    ///Sends a `GET` request to `/domains/{domain}`
    ///
    ///```ignore
    /// let response = client.get_domain()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain(&self) -> builder::GetDomain;
    ///Delete domain
    ///
    ///Deletes a domain from your account
    ///
    ///Sends a `DELETE` request to `/domains/{domain}`
    ///
    ///```ignore
    /// let response = client.delete_domain()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_domain(&self) -> builder::DeleteDomain;
    ///Verify domain
    ///
    ///Verifies and returns a single domain, including credentials and DNS records. If the domain is successfully verified the message should be the following: _Domain DNS records have been updated_. For more information on verifying domains, visit the Mailgun [User Manual](https://documentation.mailgun.com/en/latest/user_manual.html#verifying-your-domain).
    ///
    ///Sends a `PUT` request to `/domains/{domain}/verify`
    ///
    ///```ignore
    /// let response = client.verify_domain()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn verify_domain(&self) -> builder::VerifyDomain;
    ///Get SMTP credentials for a domain
    ///
    ///Returns a list of SMTP credentials for the defined domain.
    ///
    ///| Parameter | Description                                           |
    ///|-----------|-------------------------------------------------------|
    ///| limit     | Maximum number of records to return. (100 by default) |
    ///| skip      | Number of records to skip. (0 by default)             |
    ///
    ///Sends a `GET` request to `/domains/{domain}/credentials`
    ///
    ///Arguments:
    /// - `domain`
    /// - `limit`: Maximum number of records to return; 100 by default
    /// - `skip`: Number of records to skip; 0 by default.
    /// - `authorization`
    ///```ignore
    /// let response = client.get_credentials()
    ///    .domain(domain)
    ///    .limit(limit)
    ///    .skip(skip)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_credentials(&self) -> builder::GetCredentials;
    ///Create new SMTP credentials
    ///
    ///Creates a new set of SMTP credentials for the defined domain.
    ///
    ///| Parameter | Description                                                 |
    ///|-----------|-------------------------------------------------------------|
    ///| login     | The user name, for example `bob.bar`                          |
    ///| password  | A password for the SMTP credentials. (Length Min 5, Max 32) |
    ///
    ///Sends a `POST` request to `/domains/{domain}/credentials`
    ///
    ///```ignore
    /// let response = client.create_smtp_credentials()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_smtp_credentials(&self) -> builder::CreateSmtpCredentials;
    ///Update SMTP credentials
    ///
    ///Updates the specified SMTP credentials. Currently only the password can
    /// be changed.
    ///
    ///| Parameter | Description                                                 |
    ///|-----------|-------------------------------------------------------------|
    ///| password  | A password for the SMTP credentials. (Length Min 5, Max 32) |
    ///
    ///Sends a `PUT` request to `/domains/{domain}/credentials/{login}`
    ///
    ///```ignore
    /// let response = client.update_smtp_credentials()
    ///    .domain(domain)
    ///    .login(login)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_smtp_credentials(&self) -> builder::UpdateSmtpCredentials;
    ///Delete SMTP credentials
    ///
    ///Deletes the defined SMTP credentials.
    ///
    /// > Note: Mailgun imposes a rate limit for the Domains API endpoint. Users
    /// > may issue no more than 300 requests per minute, per account. See the
    /// > resultant rate limit response below.
    ///
    ///Sends a `DELETE` request to `/domains/{domain}/credentials/{login}`
    ///
    ///```ignore
    /// let response = client.delete_smtp_credentials()
    ///    .domain(domain)
    ///    .login(login)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_smtp_credentials(&self) -> builder::DeleteSmtpCredentials;
    ///Get domain connection settings
    ///
    ///Returns delivery connection settings for the defined domain.
    ///
    ///Sends a `GET` request to `/domains/{domain}/connection`
    ///
    ///```ignore
    /// let response = client.get_domain_connection()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_connection(&self) -> builder::GetDomainConnection;
    ///Update domain delivery connection settings
    ///
    ///Updates the specified delivery connection settings for the defined
    /// domain.
    ///
    ///| Parameter         | Description                                                                                                                                                                                                                                                                                                                                                       |
    ///|-------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| require_tls       | true or false  If set to true, this requires the message only be sent over a TLS connection. If a TLS connection can not be established, Mailgun will not deliver the message.  If set to false, Mailgun will still try and upgrade the connection, but if Mailgun cannot, the message will be delivered over a plaintext SMTP connection.  The default is false. |
    ///| skip_verification | true or false  If set to true, the certificate and hostname will not be verified when trying to establish a TLS connection and Mailgun will accept any certificate during delivery.   If set to false, Mailgun will verify the certificate and hostname. If either one can not be verified, a TLS connection will not be established.   The default is false.     |
    ///
    ///Sends a `PUT` request to `/domains/{domain}/connection`
    ///
    ///```ignore
    /// let response = client.update_domain_connection()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_domain_connection(&self) -> builder::UpdateDomainConnection;
    ///Get domain tracking settings
    ///
    ///Returns tracking settings for a domain.
    ///
    ///Sends a `GET` request to `/domains/{domain}/tracking`
    ///
    ///```ignore
    /// let response = client.get_domain_tracking()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_tracking(&self) -> builder::GetDomainTracking;
    ///Update domain tracking settings for OPENS
    ///
    ///Updates the open tracking settings for a domain.
    ///
    ///| Parameter | Description |
    ///|-----------|-------------|
    ///| active    | `yes` or `no`   |
    ///
    ///Sends a `PUT` request to `/domains/{domain}/tracking/open`
    ///
    ///```ignore
    /// let response = client.update_domain_tracking()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_domain_tracking(&self) -> builder::UpdateDomainTracking;
    ///Update domain tracking settings for CLICKS
    ///
    ///| Parameter | Description                                                                                                                                                                                        |
    ///|-----------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| active    | yes, no, or htmlonly  If set to yes, links will be overwritten and pointed to our servers so we can track clicks.  If set to htmlonly, links will only be rewritten in the HTML part of a message. |
    ///
    ///Sends a `PUT` request to `/domains/{domain}/tracking/click`
    ///
    ///```ignore
    /// let response = client.update_domain_click_tracking()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_domain_click_tracking(&self) -> builder::UpdateDomainClickTracking;
    ///Update domain tracking settings for UNSUBSCRIBES
    ///
    ///| Parameter   | Description                                                                                                                                                                                                                                                                                                            |
    ///|-------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| active      | true or false.                                                                                                                                                                                                                                                                                                         |
    ///| html_footer | Custom HTML version of unsubscribe footer.                                                                                                                                                                                                                                                                             |
    ///| text_footer | Custom text version of unsubscribe footer.Mailgun can automatically provide an unsubscribe footer in each email you send and also provides you with several unsubscribe variables. You can customize your unsubscribe footer by editing the settings in the Control Panel. See Tracking Unsubscribes for more details. |
    ///
    ///Sends a `PUT` request to `/domains/{domain}/tracking/unsubscribe`
    ///
    ///```ignore
    /// let response = client.update_domain_unsubscribe_tracking()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_domain_unsubscribe_tracking(&self) -> builder::UpdateDomainUnsubscribeTracking;
    ///Change DKIM Authority
    ///
    ///| Parameter | Description                                                                                                                                                                                                                                                                                                         |
    ///|-----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| self      | true or false  Change the DKIM authority for a domain.  If set to true, the domain will be the DKIM authority for itself even if the root domain is registered on the same mailgun account  If set to false, the domain will have the same DKIM authority as the root domain registered on the same mailgun account |
    ///
    ///
    /// > Note: Use with caution: Do not forget to change the corresponding DNS
    /// > record. It can take 24-48 hours for DNS changes to propagate. Changing
    /// > the DKIM autority of an active domain affects its current
    /// > deliveriability.
    ///
    ///Sends a `PUT` request to `/domains/{domain}/dkim_authority`
    ///
    ///```ignore
    /// let response = client.update_dkim_authority()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_dkim_authority(&self) -> builder::UpdateDkimAuthority;
}

impl ClientDomainsExt for Client {
    fn get_domains(&self) -> builder::GetDomains {
        builder::GetDomains::new(self)
    }

    fn create_domain(&self) -> builder::CreateDomain {
        builder::CreateDomain::new(self)
    }

    fn get_domain(&self) -> builder::GetDomain {
        builder::GetDomain::new(self)
    }

    fn delete_domain(&self) -> builder::DeleteDomain {
        builder::DeleteDomain::new(self)
    }

    fn verify_domain(&self) -> builder::VerifyDomain {
        builder::VerifyDomain::new(self)
    }

    fn get_credentials(&self) -> builder::GetCredentials {
        builder::GetCredentials::new(self)
    }

    fn create_smtp_credentials(&self) -> builder::CreateSmtpCredentials {
        builder::CreateSmtpCredentials::new(self)
    }

    fn update_smtp_credentials(&self) -> builder::UpdateSmtpCredentials {
        builder::UpdateSmtpCredentials::new(self)
    }

    fn delete_smtp_credentials(&self) -> builder::DeleteSmtpCredentials {
        builder::DeleteSmtpCredentials::new(self)
    }

    fn get_domain_connection(&self) -> builder::GetDomainConnection {
        builder::GetDomainConnection::new(self)
    }

    fn update_domain_connection(&self) -> builder::UpdateDomainConnection {
        builder::UpdateDomainConnection::new(self)
    }

    fn get_domain_tracking(&self) -> builder::GetDomainTracking {
        builder::GetDomainTracking::new(self)
    }

    fn update_domain_tracking(&self) -> builder::UpdateDomainTracking {
        builder::UpdateDomainTracking::new(self)
    }

    fn update_domain_click_tracking(&self) -> builder::UpdateDomainClickTracking {
        builder::UpdateDomainClickTracking::new(self)
    }

    fn update_domain_unsubscribe_tracking(&self) -> builder::UpdateDomainUnsubscribeTracking {
        builder::UpdateDomainUnsubscribeTracking::new(self)
    }

    fn update_dkim_authority(&self) -> builder::UpdateDkimAuthority {
        builder::UpdateDkimAuthority::new(self)
    }
}

pub trait ClientEmailValidationExt {
    ///Validate address
    ///
    ///| Field Explanation:    |         |                                                                                                                         |
    ///|-----------------------|---------|-------------------------------------------------------------------------------------------------------------------------|
    ///|                       |         |                                                                                                                         |
    ///| Parameter             | Type    | Description                                                                                                             |
    ///| address               | string  | Email address being validated                                                                                           |
    ///| did_you_mean          | string  | Null if nothing, however if a potential typo is made, the closest suggestion is provided                                |
    ///| is_disposable_address | boolean | If the domain is in a list of disposable email addresses, this will be appropriately categorized                        |
    ///| is_role_address       | boolean | Checks the mailbox portion of the email if it matches a specific role type (‘admin’, ‘sales’, ‘webmaster’)              |
    ///| is_valid              | boolean | Runs the email segments across a valid known provider rule list. If a violation occurs this value is false              |
    ///| mailbox_verification  | string  | If the mail_verification flag is enabled, a call is made to the ESP to return existence. (true, false, unknown or null) |
    ///| parts                 | string  | (display_name, domain, local_part): Parsed segments of the provided email address                                       |
    ///
    ///Sends a `GET` request to `/address/validate`
    ///
    ///```ignore
    /// let response = client.validate_address()
    ///    .address(address)
    ///    .api_key(api_key)
    ///    .mailbox_verification(mailbox_verification)
    ///    .send()
    ///    .await;
    /// ```
    fn validate_address(&self) -> builder::ValidateAddress;
    ///Parse email address list
    ///
    ///Sends a `GET` request to `/address/parse`
    ///
    ///```ignore
    /// let response = client.parse_list()
    ///    .send()
    ///    .await;
    /// ```
    fn parse_list(&self) -> builder::ParseList;
    ///Validate address (private)
    ///
    ///Sends a `GET` request to `/address/private/validate`
    ///
    ///```ignore
    /// let response = client.validate_address_private()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn validate_address_private(&self) -> builder::ValidateAddressPrivate;
    ///Parse email address list (private)
    ///
    ///Sends a `GET` request to `/address/private/varse`
    ///
    ///```ignore
    /// let response = client.parse_list_private()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn parse_list_private(&self) -> builder::ParseListPrivate;
}

impl ClientEmailValidationExt for Client {
    fn validate_address(&self) -> builder::ValidateAddress {
        builder::ValidateAddress::new(self)
    }

    fn parse_list(&self) -> builder::ParseList {
        builder::ParseList::new(self)
    }

    fn validate_address_private(&self) -> builder::ValidateAddressPrivate {
        builder::ValidateAddressPrivate::new(self)
    }

    fn parse_list_private(&self) -> builder::ParseListPrivate {
        builder::ParseListPrivate::new(self)
    }
}

pub trait ClientEventsExt {
    ///Get events
    ///
    ///Get events for a domain.
    ///
    ///Sends a `GET` request to `/v3/{domain}/events`
    ///
    ///```ignore
    /// let response = client.get_events()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_events(&self) -> builder::GetEvents;
}

impl ClientEventsExt for Client {
    fn get_events(&self) -> builder::GetEvents {
        builder::GetEvents::new(self)
    }
}

pub trait ClientIPsExt {
    ///Get all IPs for an account
    ///
    ///| Parameter | Description                                                             |
    ///|-----------|-------------------------------------------------------------------------|
    ///| dedicated | Return only dedicated IPs if set to true. (all are returned by default) |
    ///
    ///Sends a `GET` request to `/ips`
    ///
    ///```ignore
    /// let response = client.get_ips()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_ips(&self) -> builder::GetIps;
    ///Get specific IP
    ///
    ///Returns information about the specified IP.
    ///
    ///Sends a `GET` request to `/ips/{ip}`
    ///
    ///```ignore
    /// let response = client.get_ip()
    ///    .ip(ip)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_ip(&self) -> builder::GetIp;
    ///Get list of IPs for a domain
    ///
    ///Returns a list of IPs currently assigned to the specified domain.
    ///
    ///Sends a `GET` request to `/v3/domains/{domain}/ips`
    ///
    ///```ignore
    /// let response = client.get_ip_for_domain()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_ip_for_domain(&self) -> builder::GetIpForDomain;
    ///Assign IP to domain
    ///
    ///Assign a dedicated IP to the domain specified.
    ///
    /// > **Note**: Only dedicated IPs can be assigned to a domain.
    ///
    ///Sends a `POST` request to `/domains/{domain}/ips`
    ///
    ///```ignore
    /// let response = client.assign_ip_to_domain()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn assign_ip_to_domain(&self) -> builder::AssignIpToDomain;
    ///Unassign IP from a domain
    ///
    ///Unassign an IP from the domain specified.
    ///
    ///Sends a `DELETE` request to `/domains/{domain}/ips/{ip}`
    ///
    ///```ignore
    /// let response = client.unassign_ip_from_domain()
    ///    .domain(domain)
    ///    .ip(ip)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn unassign_ip_from_domain(&self) -> builder::UnassignIpFromDomain;
}

impl ClientIPsExt for Client {
    fn get_ips(&self) -> builder::GetIps {
        builder::GetIps::new(self)
    }

    fn get_ip(&self) -> builder::GetIp {
        builder::GetIp::new(self)
    }

    fn get_ip_for_domain(&self) -> builder::GetIpForDomain {
        builder::GetIpForDomain::new(self)
    }

    fn assign_ip_to_domain(&self) -> builder::AssignIpToDomain {
        builder::AssignIpToDomain::new(self)
    }

    fn unassign_ip_from_domain(&self) -> builder::UnassignIpFromDomain {
        builder::UnassignIpFromDomain::new(self)
    }
}

pub trait ClientMailingListsExt {
    ///Get mailing lists
    ///
    ///Sends a `GET` request to `/lists/pages`
    ///
    ///Arguments:
    /// - `limit`: Maximum number of records to return (optional: 100 by
    ///   default)
    /// - `authorization`
    ///```ignore
    /// let response = client.get_lists()
    ///    .limit(limit)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_lists(&self) -> builder::GetLists;
    ///Get single mailing list by address
    ///
    ///Sends a `GET` request to `/lists/{address}`
    ///
    ///```ignore
    /// let response = client.get_list()
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_list(&self) -> builder::GetList;
    ///Update mailing list
    ///
    ///Sends a `PUT` request to `/lists/{address}`
    ///
    ///```ignore
    /// let response = client.update_list()
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_list(&self) -> builder::UpdateList;
    ///Delete mailing list
    ///
    ///Deletes a mailing list.
    ///
    ///Sends a `DELETE` request to `/lists/{address}`
    ///
    ///```ignore
    /// let response = client.delete_list()
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_list(&self) -> builder::DeleteList;
    ///Create mailing list
    ///
    ///Sends a `POST` request to `/lists`
    ///
    ///```ignore
    /// let response = client.create_list()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_list(&self) -> builder::CreateList;
    ///Get mailing list members
    ///
    ///Paginate over list members in the given mailing list
    ///
    ///Sends a `GET` request to `/lists/{address}/members/pages`
    ///
    ///Arguments:
    /// - `address`
    /// - `limit`: Maximum number of records to return (optional: 100 by
    ///   default)
    /// - `subscribed`: yes to lists subscribed, no for unsubscribed. list all
    ///   if not set
    /// - `authorization`
    ///```ignore
    /// let response = client.get_list_members()
    ///    .address(address)
    ///    .limit(limit)
    ///    .subscribed(subscribed)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_list_members(&self) -> builder::GetListMembers;
    ///Get mailing list member
    ///
    ///Retrieves a mailing list member.
    ///
    ///Sends a `GET` request to `/lists/{address}/members/{memberaddress}`
    ///
    ///```ignore
    /// let response = client.get_list_member()
    ///    .address(address)
    ///    .memberaddress(memberaddress)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_list_member(&self) -> builder::GetListMember;
    ///Update mailing list member
    ///
    ///Updates a mailing list member with given properties. Won’t touch the
    /// property if it’s not passed in.
    ///
    ///Sends a `PUT` request to `/lists/{address}/members/{memberaddress}`
    ///
    ///```ignore
    /// let response = client.update_list_member()
    ///    .address(address)
    ///    .memberaddress(memberaddress)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_list_member(&self) -> builder::UpdateListMember;
    ///Delete member from mailing list
    ///
    ///Delete a mailing list member.
    ///
    ///Sends a `DELETE` request to `/lists/{address}/members/{memberaddress}`
    ///
    ///```ignore
    /// let response = client.delete_list_member()
    ///    .address(address)
    ///    .memberaddress(memberaddress)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_list_member(&self) -> builder::DeleteListMember;
    ///Add member to mailing list
    ///
    ///Adds a member to the mailing list.
    ///
    ///Sends a `POST` request to `/lists/{address}/members`
    ///
    ///```ignore
    /// let response = client.add_list_member()
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_list_member(&self) -> builder::AddListMember;
    ///Add multiple members to mailing list
    ///
    ///Adds multiple members, up to 1,000 per call, to a Mailing List.
    ///
    ///Sends a `POST` request to `/lists/{address}/members.json`
    ///
    ///```ignore
    /// let response = client.add_list_members()
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_list_members(&self) -> builder::AddListMembers;
}

impl ClientMailingListsExt for Client {
    fn get_lists(&self) -> builder::GetLists {
        builder::GetLists::new(self)
    }

    fn get_list(&self) -> builder::GetList {
        builder::GetList::new(self)
    }

    fn update_list(&self) -> builder::UpdateList {
        builder::UpdateList::new(self)
    }

    fn delete_list(&self) -> builder::DeleteList {
        builder::DeleteList::new(self)
    }

    fn create_list(&self) -> builder::CreateList {
        builder::CreateList::new(self)
    }

    fn get_list_members(&self) -> builder::GetListMembers {
        builder::GetListMembers::new(self)
    }

    fn get_list_member(&self) -> builder::GetListMember {
        builder::GetListMember::new(self)
    }

    fn update_list_member(&self) -> builder::UpdateListMember {
        builder::UpdateListMember::new(self)
    }

    fn delete_list_member(&self) -> builder::DeleteListMember {
        builder::DeleteListMember::new(self)
    }

    fn add_list_member(&self) -> builder::AddListMember {
        builder::AddListMember::new(self)
    }

    fn add_list_members(&self) -> builder::AddListMembers {
        builder::AddListMembers::new(self)
    }
}

pub trait ClientMessagesExt {
    ///Sending a message with HTML and text parts, and attachments
    ///
    ///sends a message
    ///
    ///Sends a `POST` request to `/{domain}/messages`
    ///
    ///```ignore
    /// let response = client.send_message()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn send_message(&self) -> builder::SendMessage;
    ///Retrieving Stored Messages
    ///
    ///To retrieve an inbound message that has been stored via the `store()` action, use the URL found in the stored event (which you can find through the Events API, or in the notify webhook set when creating the store action (`store(notify="http://domain.com/callback")`).
    ///
    /// * By default the message will be returned in JSON form with parsed
    ///   parts. Links to the attachments will be included.
    /// * You can also retrieve the full raw mime message (attachments and all)
    ///   if you make the request to the URL with the `Accept` header set to
    ///   `message/rfc2822`.
    /// * Stored messages are encoded with [Quoted-printable](https://en.wikipedia.org/wiki/Quoted-printable)
    ///   encoding. Decoding samples are available in the examples section
    ///   below.
    ///
    ///
    ///These are the parameters of the JSON returned from a GET request to a
    /// stored message url.
    ///
    ///| Parameter          | Type   | Description                                                                                                                                                                                                      |
    ///|--------------------|--------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    ///| recipients         | string | recipient of the message as reported by `MAIL` `TO` during SMTP chat.                                                                                                                                                |
    ///| sender             | string | sender of the message as reported by `MAIL` `FROM` during SMTP chat. Note: this value may differ from `From` MIME header.                                                                                              |
    ///| from               | string | sender of the message as reported by `From` message header, for example “Bob Lee <blee@mailgun.net>”.                                                                                                              |
    ///| subject            | string | subject string.                                                                                                                                                                                                  |
    ///| body-plain         | string | text version of the email. This field is always present. If the incoming message only has HTML body, Mailgun will create a text representation for you.                                                          |
    ///| stripped-text      | string | text version of the message without quoted parts and signature block (if found).                                                                                                                                 |
    ///| stripped-signature | string | the signature block stripped from the plain text message (if found).                                                                                                                                             |
    ///| body-html          | string | HTML version of the message, if message was multipart. Note that all parts of the message will be posted, not just text/html. For instance if a message arrives with “foo” part it will be posted as “body-foo”. |
    ///| stripped-html      | string | HTML version of the message, without quoted parts.                                                                                                                                                               |
    ///| attachments        | string | contains a json list of metadata objects, one for each attachment, see below.                                                                                                                                    |
    ///| message-headers    | string | list of all MIME headers dumped to a json string (order of headers preserved).                                                                                                                                   |
    ///| content-id-map     | string | JSON-encoded dictionary which maps Content-ID (CID) of each attachment to the corresponding `attachment-x` parameter. This allows you to map posted attachments to tags like `<img src='cid'>` in the message body.  |
    ///
    ///
    /// > Note Do not rely on the `body-plain`, `stripped-text`, and
    /// > `stripped-signature` fields for HTML sanitization. These fields merely
    /// > provide content from the text/plain portion of an incoming message.
    /// > This content may contain unescaped HTML.
    ///
    ///The attachments JSON contains the following items.
    ///
    ///| Parameter    | Type    | Description                                                                       |
    ///|--------------|---------|-----------------------------------------------------------------------------------|
    ///| size         | integer | indicates the size of the attachment in bytes.                                    |
    ///| url          | string  | contains the url where the attachment can be found. This does not support DELETE. |
    ///| name         | string  | the name of the attachment                                                        |
    ///| content-type | string  | the content type of the attachment                                                |
    ///
    ///
    ///These are the parameters when the `Accept` header is set to
    /// `message/rfc2822`
    ///
    ///| Parameter | Type   | Description                                                                                    |
    ///|-----------|--------|------------------------------------------------------------------------------------------------|
    ///| recipient | string | recipient of the message.                                                                      |
    ///| sender    | string | sender of the message as reported by SMTP MAIL FROM.                                           |
    ///| from      | string | sender of the message as reported by `From` message header, for example “Bob <bob@example.com>”. |
    ///| subject   | string | subject string.                                                                                |
    ///| body-mime | string | full MIME envelope. You will need a MIME parsing library to process this data.                 |
    ///
    ///Sends a `GET` request to `/{domain}/messages/{message_ID}`
    ///
    ///```ignore
    /// let response = client.get_message()
    ///    .domain(domain)
    ///    .message_id(message_id)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_message(&self) -> builder::GetMessage;
}

impl ClientMessagesExt for Client {
    fn send_message(&self) -> builder::SendMessage {
        builder::SendMessage::new(self)
    }

    fn get_message(&self) -> builder::GetMessage {
        builder::GetMessage::new(self)
    }
}

pub trait ClientRoutesExt {
    ///Get routes
    ///
    ///Fetches the list of routes. Note that routes are defined globally, per
    /// account, not per domain as most of other API calls.
    ///
    ///Sends a `GET` request to `/routes`
    ///
    ///```ignore
    /// let response = client.get_routes()
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_routes(&self) -> builder::GetRoutes;
    ///Create route
    ///
    ///Creates a new route.
    ///
    ///Sends a `POST` request to `/routes`
    ///
    ///```ignore
    /// let response = client.create_route()
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_route(&self) -> builder::CreateRoute;
    ///Update route
    ///
    ///Updates a given route by ID. All parameters are optional: this API call
    /// only updates the specified fields leaving others unchanged.
    ///
    ///Sends a `PUT` request to `/routes/{route}`
    ///
    ///```ignore
    /// let response = client.update_route()
    ///    .route(route)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_route(&self) -> builder::UpdateRoute;
    ///Get single route
    ///
    ///Returns a single route object based on its ID.
    ///
    ///Sends a `POST` request to `/routes/{route}`
    ///
    ///```ignore
    /// let response = client.get_route()
    ///    .route(route)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn get_route(&self) -> builder::GetRoute;
    ///Delete route
    ///
    ///Deletes a route based on the id.
    ///
    ///Sends a `DELETE` request to `/{domain}/routes/{id}`
    ///
    ///```ignore
    /// let response = client.delete_route()
    ///    .domain(domain)
    ///    .id(id)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_route(&self) -> builder::DeleteRoute;
}

impl ClientRoutesExt for Client {
    fn get_routes(&self) -> builder::GetRoutes {
        builder::GetRoutes::new(self)
    }

    fn create_route(&self) -> builder::CreateRoute {
        builder::CreateRoute::new(self)
    }

    fn update_route(&self) -> builder::UpdateRoute {
        builder::UpdateRoute::new(self)
    }

    fn get_route(&self) -> builder::GetRoute {
        builder::GetRoute::new(self)
    }

    fn delete_route(&self) -> builder::DeleteRoute {
        builder::DeleteRoute::new(self)
    }
}

pub trait ClientStatsExt {
    ///Get domain stats
    ///
    ///Returns total stats for a given domain.
    ///
    ///| Parameter  | Description                                                                                                             |
    ///|------------|-------------------------------------------------------------------------------------------------------------------------|
    ///| event      | The type of the event. For a complete list of all events written to the log see the [Event Types](https://documentation.mailgun.com/en/latest/api-stats.html#event-types) table below. (Required) |
    ///| start      | The starting time. Should be in [RFC 2822#page-14](https://tools.ietf.org/html/rfc2822.html#page-14) or unix epoch format. Default: 7 days from the current time.           |
    ///| end        | The ending date. Should be in [RFC 2822#page-14](https://tools.ietf.org/html/rfc2822.html#page-14) or unix epoch format. Default: current time.                             |
    ///| resolution | Can be either `hour`, `day` or `month`. Default: `day`                                                                          |
    ///| duration   | Period of time with resoluton encoded. See [Duration](https://documentation.mailgun.com/en/latest/api-stats.html#duration) for more info. If provided, overwrites the start date.              |
    ///
    ///Sends a `GET` request to `/{domain}/stats/total`
    ///
    ///Arguments:
    /// - `domain`
    /// - `event`: The type of the event. For a complete list of all events
    ///   written to the log see the Event Types table below. (Required)
    /// - `authorization`
    ///```ignore
    /// let response = client.get_domain_stats()
    ///    .domain(domain)
    ///    .event(event)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_stats(&self) -> builder::GetDomainStats;
    ///Get stat items
    ///
    ///Returns a list of event stats items. Each record represents counts for
    /// one event per one day.
    ///
    /// > **Warning**: This is the legacy API that should not be used.
    ///
    ///Sends a `GET` request to `/{domain}/stats`
    ///
    ///Arguments:
    /// - `domain`
    /// - `limit`: Maximum number of records to return. (100 by default)
    /// - `skip`: Number of records to skip. (0 by default)
    /// - `authorization`
    ///```ignore
    /// let response = client.get_domain_stat_items()
    ///    .domain(domain)
    ///    .limit(limit)
    ///    .skip(skip)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_stat_items(&self) -> builder::GetDomainStatItems;
}

impl ClientStatsExt for Client {
    fn get_domain_stats(&self) -> builder::GetDomainStats {
        builder::GetDomainStats::new(self)
    }

    fn get_domain_stat_items(&self) -> builder::GetDomainStatItems {
        builder::GetDomainStatItems::new(self)
    }
}

pub trait ClientSuppressionsBouncesExt {
    ///Get bounces
    ///
    ///Paginate over a list of bounces for a domain.
    ///
    ///Sends a `GET` request to `/{domain}/bounces`
    ///
    ///```ignore
    /// let response = client.get_bounces()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_bounces(&self) -> builder::GetBounces;
    ///Add single bounce
    ///
    ///Add a bounce record to the bounce list. Updates the existing record if
    /// the address is already there.
    ///
    ///Sends a `POST` request to `/{domain}/bounces`
    ///
    ///```ignore
    /// let response = client.add_bounce()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_bounce(&self) -> builder::AddBounce;
    ///Delete bounce list
    ///
    ///Add a bounce record to the bounce list. Updates the existing record if
    /// the address is already there.
    ///
    ///Sends a `DELETE` request to `/{domain}/bounces`
    ///
    ///```ignore
    /// let response = client.delete_bounce_list()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_bounce_list(&self) -> builder::DeleteBounceList;
    ///Get single bounce
    ///
    ///Fetch a single bounce event by a given email address. Useful to check if
    /// a given email address has bounced before.
    ///
    ///Sends a `GET` request to `/{domain}/bounces/{address}`
    ///
    ///```ignore
    /// let response = client.get_bounce()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_bounce(&self) -> builder::GetBounce;
    ///Delete bounce
    ///
    ///Add a bounce record to the bounce list. Updates the existing record if
    /// the address is already there.
    ///
    ///Sends a `DELETE` request to `/{domain}/bounces/{address}`
    ///
    ///```ignore
    /// let response = client.delete_bounce()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_bounce(&self) -> builder::DeleteBounce;
    ///Add multiple bounces
    ///
    ///Add a bounce record to the bounce list. Updates the existing record if
    /// the address is already there.
    ///
    ///Sends a `POST` request to `/v3/{domain}/bounces`
    ///
    ///```ignore
    /// let response = client.add_bounces()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_bounces(&self) -> builder::AddBounces;
}

impl ClientSuppressionsBouncesExt for Client {
    fn get_bounces(&self) -> builder::GetBounces {
        builder::GetBounces::new(self)
    }

    fn add_bounce(&self) -> builder::AddBounce {
        builder::AddBounce::new(self)
    }

    fn delete_bounce_list(&self) -> builder::DeleteBounceList {
        builder::DeleteBounceList::new(self)
    }

    fn get_bounce(&self) -> builder::GetBounce {
        builder::GetBounce::new(self)
    }

    fn delete_bounce(&self) -> builder::DeleteBounce {
        builder::DeleteBounce::new(self)
    }

    fn add_bounces(&self) -> builder::AddBounces {
        builder::AddBounces::new(self)
    }
}

pub trait ClientSuppressionsComplaintsExt {
    ///Get all complaints
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `GET` request to `/{domain}/complaints`
    ///
    ///```ignore
    /// let response = client.get_complaints()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_complaints(&self) -> builder::GetComplaints;
    ///Add multiple complaints
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `POST` request to `/{domain}/complaints`
    ///
    ///```ignore
    /// let response = client.add_complaints()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_complaints(&self) -> builder::AddComplaints;
    ///Get single complaint
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `GET` request to `/{domain}/complaints/{address}`
    ///
    ///```ignore
    /// let response = client.get_complaint()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .send()
    ///    .await;
    /// ```
    fn get_complaint(&self) -> builder::GetComplaint;
    ///Delete single complaint
    ///
    ///Remove a given spam complaint.
    ///
    ///Sends a `DELETE` request to `/{domain}/complaints/{address}`
    ///
    ///```ignore
    /// let response = client.delete_complaint()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_complaint(&self) -> builder::DeleteComplaint;
    ///Add single complaint
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `POST` request to `/v3/{domain}/complaints`
    ///
    ///```ignore
    /// let response = client.add_complaint()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_complaint(&self) -> builder::AddComplaint;
}

impl ClientSuppressionsComplaintsExt for Client {
    fn get_complaints(&self) -> builder::GetComplaints {
        builder::GetComplaints::new(self)
    }

    fn add_complaints(&self) -> builder::AddComplaints {
        builder::AddComplaints::new(self)
    }

    fn get_complaint(&self) -> builder::GetComplaint {
        builder::GetComplaint::new(self)
    }

    fn delete_complaint(&self) -> builder::DeleteComplaint {
        builder::DeleteComplaint::new(self)
    }

    fn add_complaint(&self) -> builder::AddComplaint {
        builder::AddComplaint::new(self)
    }
}

pub trait ClientSuppressionsUnsubscribesExt {
    ///Get domain unsubscribes
    ///
    ///Paginate over a list of unsubscribes for a domain.
    ///
    ///Sends a `GET` request to `/{domain}/unsubscribes`
    ///
    ///```ignore
    /// let response = client.get_domain_unsubscribes()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_unsubscribes(&self) -> builder::GetDomainUnsubscribes;
    ///Add multiple unsubscribes
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `POST` request to `/{domain}/unsubscribes`
    ///
    ///```ignore
    /// let response = client.add_domain_unsubscribes()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .content_type(content_type)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn add_domain_unsubscribes(&self) -> builder::AddDomainUnsubscribes;
    ///Get single unsubscribe
    ///
    ///Paginate over a list of unsubscribes for a domain.
    ///
    ///Sends a `GET` request to `/{domain}/unsubscribes/{address}`
    ///
    ///```ignore
    /// let response = client.get_domain_unsubscribe()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_unsubscribe(&self) -> builder::GetDomainUnsubscribe;
    ///Delete single unsubscribe
    ///
    ///Add multiple unsubscribe records to the unsubscribe list in a single API
    /// call.
    ///
    ///Sends a `DELETE` request to `/{domain}/unsubscribes/{address}`
    ///
    ///```ignore
    /// let response = client.delete_domain_unsubscribe()
    ///    .domain(domain)
    ///    .address(address)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_domain_unsubscribe(&self) -> builder::DeleteDomainUnsubscribe;
}

impl ClientSuppressionsUnsubscribesExt for Client {
    fn get_domain_unsubscribes(&self) -> builder::GetDomainUnsubscribes {
        builder::GetDomainUnsubscribes::new(self)
    }

    fn add_domain_unsubscribes(&self) -> builder::AddDomainUnsubscribes {
        builder::AddDomainUnsubscribes::new(self)
    }

    fn get_domain_unsubscribe(&self) -> builder::GetDomainUnsubscribe {
        builder::GetDomainUnsubscribe::new(self)
    }

    fn delete_domain_unsubscribe(&self) -> builder::DeleteDomainUnsubscribe {
        builder::DeleteDomainUnsubscribe::new(self)
    }
}

pub trait ClientTagsExt {
    ///Get domain tags
    ///
    ///Returns a list of tags for a domain. Provides pagination urls if the
    /// result set is too long to be returned in a single response.
    ///
    ///Sends a `GET` request to `/{domain}/tags`
    ///
    ///```ignore
    /// let response = client.get_domain_tags()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_tags(&self) -> builder::GetDomainTags;
    ///Get single tag
    ///
    ///Returns a given tag.
    ///
    ///Sends a `GET` request to `/{domain}/tags/{tag}`
    ///
    ///```ignore
    /// let response = client.get_domain_tag()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_tag(&self) -> builder::GetDomainTag;
    ///Update tag
    ///
    ///Updates a given tag with the information provided.
    ///
    ///Sends a `PUT` request to `/{domain}/tags/{tag}`
    ///
    ///```ignore
    /// let response = client.update_domain_tag()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_domain_tag(&self) -> builder::UpdateDomainTag;
    ///Delete tag
    ///
    ///Deletes the tag. Note: The statistics for the tag are not destroyed.
    ///
    ///Sends a `DELETE` request to `/{domain}/tags/{tag}`
    ///
    ///```ignore
    /// let response = client.delete_domain_tag()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_domain_tag(&self) -> builder::DeleteDomainTag;
    ///Get domain providers
    ///
    ///Returns a list of email providers for a given domain for different event
    /// types.
    ///
    ///Sends a `GET` request to
    /// `/{domain}/tags/{tag}/stats/aggregates/providers`
    ///
    ///```ignore
    /// let response = client.get_domain_providers()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_providers(&self) -> builder::GetDomainProviders;
    ///Get tag stats
    ///
    ///Returns statistics for a given tag.
    ///
    ///Sends a `GET` request to `/{domain}/tags/{tag}/stats`
    ///
    ///Arguments:
    /// - `domain`
    /// - `tag`
    /// - `event`: The type of the event. For a complete list of all events
    ///   written to the log see the Event Types table below. (Required)
    /// - `authorization`
    ///```ignore
    /// let response = client.get_tag_stats()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .event(event)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_tag_stats(&self) -> builder::GetTagStats;
    ///Get domain countries
    ///
    ///Returns a list of countries of origin for a given domain for different
    /// event types.
    ///
    ///Sends a `GET` request to
    /// `/{domain}/tags/{tag}/stats/aggregates/countries`
    ///
    ///```ignore
    /// let response = client.get_domain_countries()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_countries(&self) -> builder::GetDomainCountries;
    ///Get domain devices
    ///
    ///Returns a list of devices for a given domain that have triggered event
    /// types.
    ///
    ///Sends a `GET` request to `/{domain}/tags/{tag}/stats/aggregates/devices`
    ///
    ///```ignore
    /// let response = client.get_domain_devices()
    ///    .domain(domain)
    ///    .tag(tag)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_domain_devices(&self) -> builder::GetDomainDevices;
}

impl ClientTagsExt for Client {
    fn get_domain_tags(&self) -> builder::GetDomainTags {
        builder::GetDomainTags::new(self)
    }

    fn get_domain_tag(&self) -> builder::GetDomainTag {
        builder::GetDomainTag::new(self)
    }

    fn update_domain_tag(&self) -> builder::UpdateDomainTag {
        builder::UpdateDomainTag::new(self)
    }

    fn delete_domain_tag(&self) -> builder::DeleteDomainTag {
        builder::DeleteDomainTag::new(self)
    }

    fn get_domain_providers(&self) -> builder::GetDomainProviders {
        builder::GetDomainProviders::new(self)
    }

    fn get_tag_stats(&self) -> builder::GetTagStats {
        builder::GetTagStats::new(self)
    }

    fn get_domain_countries(&self) -> builder::GetDomainCountries {
        builder::GetDomainCountries::new(self)
    }

    fn get_domain_devices(&self) -> builder::GetDomainDevices {
        builder::GetDomainDevices::new(self)
    }
}

pub trait ClientWebhooksExt {
    ///Get webhooks
    ///
    ///Returns a list of webhooks set for the specified domain.
    ///
    ///Sends a `GET` request to `/domains/{domain}/webhooks`
    ///
    ///```ignore
    /// let response = client.get_webhooks()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_webhooks(&self) -> builder::GetWebhooks;
    ///Create webhook
    ///
    ///Creates a new webhook. Note When adding a Clicked or Opened webhook,
    /// ensure that you also have tracking enabled.
    ///
    ///Sends a `POST` request to `/domains/{domain}/webhooks`
    ///
    ///```ignore
    /// let response = client.create_webhook()
    ///    .domain(domain)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn create_webhook(&self) -> builder::CreateWebhook;
    ///Get webhook details
    ///
    ///Returns details about a the webhook specified in the URL.
    ///
    ///Sends a `GET` request to `/domains/{domain}/webhooks/{webhookname}`
    ///
    ///```ignore
    /// let response = client.get_webhook_details()
    ///    .domain(domain)
    ///    .webhookname(webhookname)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn get_webhook_details(&self) -> builder::GetWebhookDetails;
    ///Update webhook
    ///
    ///Updates an existing webhook.
    ///
    ///Sends a `PUT` request to `/domains/{domain}/webhooks/{webhookname}`
    ///
    ///```ignore
    /// let response = client.update_webhook()
    ///    .domain(domain)
    ///    .webhookname(webhookname)
    ///    .authorization(authorization)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn update_webhook(&self) -> builder::UpdateWebhook;
    ///Delete webhook
    ///
    ///Sends a `DELETE` request to `/domains/{domain}/webhooks/{webhookname}`
    ///
    ///```ignore
    /// let response = client.delete_webhook()
    ///    .domain(domain)
    ///    .webhookname(webhookname)
    ///    .authorization(authorization)
    ///    .send()
    ///    .await;
    /// ```
    fn delete_webhook(&self) -> builder::DeleteWebhook;
}

impl ClientWebhooksExt for Client {
    fn get_webhooks(&self) -> builder::GetWebhooks {
        builder::GetWebhooks::new(self)
    }

    fn create_webhook(&self) -> builder::CreateWebhook {
        builder::CreateWebhook::new(self)
    }

    fn get_webhook_details(&self) -> builder::GetWebhookDetails {
        builder::GetWebhookDetails::new(self)
    }

    fn update_webhook(&self) -> builder::UpdateWebhook {
        builder::UpdateWebhook::new(self)
    }

    fn delete_webhook(&self) -> builder::DeleteWebhook {
        builder::DeleteWebhook::new(self)
    }
}

pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    ///Builder for [`ClientMessagesExt::send_message`]
    ///
    ///[`ClientMessagesExt::send_message`]: super::ClientMessagesExt::send_message
    #[derive(Debug, Clone)]
    pub struct SendMessage<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
        body: Result<types::builder::SendMessageBody, String>,
    }

    impl<'a> SendMessage<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
                body: Ok(types::builder::SendMessageBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::SendMessageBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `SendMessageBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::SendMessageBody) -> types::builder::SendMessageBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{domain}/messages`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::SendMessageBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/messages",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMessagesExt::get_message`]
    ///
    ///[`ClientMessagesExt::get_message`]: super::ClientMessagesExt::get_message
    #[derive(Debug, Clone)]
    pub struct GetMessage<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        message_id: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetMessage<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                message_id: Err("message_id was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn message_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.message_id = value
                .try_into()
                .map_err(|_| "conversion to `String` for message_id failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/messages/{message_ID}`
        pub async fn send(self) -> Result<ResponseValue<types::StoredMessage>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                message_id,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let message_id = message_id.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/messages/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&message_id.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::get_domains`]
    ///
    ///[`ClientDomainsExt::get_domains`]: super::ClientDomainsExt::get_domains
    #[derive(Debug, Clone)]
    pub struct GetDomains<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomains<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains`
        pub async fn send(self) -> Result<ResponseValue<types::GetDomainsResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/domains", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::create_domain`]
    ///
    ///[`ClientDomainsExt::create_domain`]: super::ClientDomainsExt::create_domain
    #[derive(Debug, Clone)]
    pub struct CreateDomain<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::CreateDomainBody, String>,
    }

    impl<'a> CreateDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
                body: Ok(types::builder::CreateDomainBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateDomainBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `CreateDomainBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateDomainBody,
            ) -> types::builder::CreateDomainBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/domains`
        pub async fn send(self) -> Result<ResponseValue<types::DomainResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateDomainBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/domains", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::get_domain`]
    ///
    ///[`ClientDomainsExt::get_domain`]: super::ClientDomainsExt::get_domain
    #[derive(Debug, Clone)]
    pub struct GetDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}`
        pub async fn send(self) -> Result<ResponseValue<types::DomainResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::delete_domain`]
    ///
    ///[`ClientDomainsExt::delete_domain`]: super::ClientDomainsExt::delete_domain
    #[derive(Debug, Clone)]
    pub struct DeleteDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/domains/{domain}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::verify_domain`]
    ///
    ///[`ClientDomainsExt::verify_domain`]: super::ClientDomainsExt::verify_domain
    #[derive(Debug, Clone)]
    pub struct VerifyDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<serde_json::Map<String, serde_json::Value>, String>,
    }

    impl<'a> VerifyDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
        {
            self . body = value . try_into () . map_err (| _ | "conversion to `serde_json :: Map < String , serde_json :: Value >` for body failed" . to_string ()) ;
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/verify`
        pub async fn send(self) -> Result<ResponseValue<types::DomainResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/verify",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::get_credentials`]
    ///
    ///[`ClientDomainsExt::get_credentials`]: super::ClientDomainsExt::get_credentials
    #[derive(Debug, Clone)]
    pub struct GetCredentials<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        limit: Result<Option<i64>, String>,
        skip: Result<Option<i64>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetCredentials<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                limit: Ok(None),
                skip: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for limit failed".to_string());
            self
        }

        pub fn skip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.skip = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for skip failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}/credentials`
        pub async fn send(self) -> Result<ResponseValue<types::GetCredentialsResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                limit,
                skip,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let skip = skip.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/credentials",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut __progenitor_query = Vec::with_capacity(2usize);
            if let Some(v) = &limit {
                __progenitor_query.push(("limit", v.to_string()));
            }
            if let Some(v) = &skip {
                __progenitor_query.push(("skip", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::create_smtp_credentials`]
    ///
    ///[`ClientDomainsExt::create_smtp_credentials`]: super::ClientDomainsExt::create_smtp_credentials
    #[derive(Debug, Clone)]
    pub struct CreateSmtpCredentials<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::CreateSmtpCredentialsBody, String>,
    }

    impl<'a> CreateSmtpCredentials<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::CreateSmtpCredentialsBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateSmtpCredentialsBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `CreateSmtpCredentialsBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateSmtpCredentialsBody,
            ) -> types::builder::CreateSmtpCredentialsBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/domains/{domain}/credentials`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateSmtpCredentialsBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/credentials",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_smtp_credentials`]
    ///
    ///[`ClientDomainsExt::update_smtp_credentials`]: super::ClientDomainsExt::update_smtp_credentials
    #[derive(Debug, Clone)]
    pub struct UpdateSmtpCredentials<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        login: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateSmtpCredentialsBody, String>,
    }

    impl<'a> UpdateSmtpCredentials<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                login: Err("login was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateSmtpCredentialsBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn login<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.login = value
                .try_into()
                .map_err(|_| "conversion to `String` for login failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateSmtpCredentialsBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `UpdateSmtpCredentialsBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateSmtpCredentialsBody,
            ) -> types::builder::UpdateSmtpCredentialsBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/credentials/{login}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                login,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let login = login.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateSmtpCredentialsBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/credentials/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&login.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::delete_smtp_credentials`]
    ///
    ///[`ClientDomainsExt::delete_smtp_credentials`]: super::ClientDomainsExt::delete_smtp_credentials
    #[derive(Debug, Clone)]
    pub struct DeleteSmtpCredentials<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        login: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteSmtpCredentials<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                login: Err("login was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn login<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.login = value
                .try_into()
                .map_err(|_| "conversion to `String` for login failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/domains/{domain}/credentials/{login}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DeleteCredentialsResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                login,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let login = login.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/credentials/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&login.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::get_domain_connection`]
    ///
    ///[`ClientDomainsExt::get_domain_connection`]: super::ClientDomainsExt::get_domain_connection
    #[derive(Debug, Clone)]
    pub struct GetDomainConnection<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainConnection<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}/connection`
        pub async fn send(self) -> Result<ResponseValue<types::GetConnectionResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/connection",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_domain_connection`]
    ///
    ///[`ClientDomainsExt::update_domain_connection`]: super::ClientDomainsExt::update_domain_connection
    #[derive(Debug, Clone)]
    pub struct UpdateDomainConnection<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDomainConnectionBody, String>,
    }

    impl<'a> UpdateDomainConnection<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDomainConnectionBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDomainConnectionBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `UpdateDomainConnectionBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDomainConnectionBody,
            ) -> types::builder::UpdateDomainConnectionBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/connection`
        pub async fn send(self) -> Result<ResponseValue<types::ConnectionUpdate>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateDomainConnectionBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/connection",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::get_domain_tracking`]
    ///
    ///[`ClientDomainsExt::get_domain_tracking`]: super::ClientDomainsExt::get_domain_tracking
    #[derive(Debug, Clone)]
    pub struct GetDomainTracking<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainTracking<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}/tracking`
        pub async fn send(self) -> Result<ResponseValue<types::GetTrackingResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/tracking",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_domain_tracking`]
    ///
    ///[`ClientDomainsExt::update_domain_tracking`]: super::ClientDomainsExt::update_domain_tracking
    #[derive(Debug, Clone)]
    pub struct UpdateDomainTracking<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDomainTrackingBody, String>,
    }

    impl<'a> UpdateDomainTracking<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDomainTrackingBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDomainTrackingBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `UpdateDomainTrackingBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDomainTrackingBody,
            ) -> types::builder::UpdateDomainTrackingBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/tracking/open`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateDomainTrackingBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/tracking/open",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_domain_click_tracking`]
    ///
    ///[`ClientDomainsExt::update_domain_click_tracking`]: super::ClientDomainsExt::update_domain_click_tracking
    #[derive(Debug, Clone)]
    pub struct UpdateDomainClickTracking<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDomainClickTrackingBody, String>,
    }

    impl<'a> UpdateDomainClickTracking<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDomainClickTrackingBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDomainClickTrackingBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `UpdateDomainClickTrackingBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDomainClickTrackingBody,
            ) -> types::builder::UpdateDomainClickTrackingBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/tracking/click`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateDomainClickTrackingBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/tracking/click",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_domain_unsubscribe_tracking`]
    ///
    ///[`ClientDomainsExt::update_domain_unsubscribe_tracking`]: super::ClientDomainsExt::update_domain_unsubscribe_tracking
    #[derive(Debug, Clone)]
    pub struct UpdateDomainUnsubscribeTracking<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDomainUnsubscribeTrackingBody, String>,
    }

    impl<'a> UpdateDomainUnsubscribeTracking<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDomainUnsubscribeTrackingBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDomainUnsubscribeTrackingBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `UpdateDomainUnsubscribeTrackingBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDomainUnsubscribeTrackingBody,
            ) -> types::builder::UpdateDomainUnsubscribeTrackingBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/tracking/unsubscribe`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(
                    std::convert::TryInto::<types::UpdateDomainUnsubscribeTrackingBody>::try_into,
                )
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/tracking/unsubscribe",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientDomainsExt::update_dkim_authority`]
    ///
    ///[`ClientDomainsExt::update_dkim_authority`]: super::ClientDomainsExt::update_dkim_authority
    #[derive(Debug, Clone)]
    pub struct UpdateDkimAuthority<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDkimAuthorityBody, String>,
    }

    impl<'a> UpdateDkimAuthority<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDkimAuthorityBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDkimAuthorityBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateDkimAuthorityBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDkimAuthorityBody,
            ) -> types::builder::UpdateDkimAuthorityBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/dkim_authority`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateDkimAuthorityBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/dkim_authority",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientIPsExt::get_ips`]
    ///
    ///[`ClientIPsExt::get_ips`]: super::ClientIPsExt::get_ips
    #[derive(Debug, Clone)]
    pub struct GetIps<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetIps<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ips`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/ips", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientIPsExt::get_ip`]
    ///
    ///[`ClientIPsExt::get_ip`]: super::ClientIPsExt::get_ip
    #[derive(Debug, Clone)]
    pub struct GetIp<'a> {
        __progenitor_client: &'a super::Client,
        ip: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetIp<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                ip: Err("ip was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn ip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.ip = value
                .try_into()
                .map_err(|_| "conversion to `String` for ip failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/ips/{ip}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                ip,
                authorization,
            } = self;
            let ip = ip.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/ips/{}",
                __progenitor_client.baseurl,
                encode_path(&ip.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientIPsExt::get_ip_for_domain`]
    ///
    ///[`ClientIPsExt::get_ip_for_domain`]: super::ClientIPsExt::get_ip_for_domain
    #[derive(Debug, Clone)]
    pub struct GetIpForDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetIpForDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v3/domains/{domain}/ips`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/v3/domains/{}/ips",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientIPsExt::assign_ip_to_domain`]
    ///
    ///[`ClientIPsExt::assign_ip_to_domain`]: super::ClientIPsExt::assign_ip_to_domain
    #[derive(Debug, Clone)]
    pub struct AssignIpToDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::AssignIpToDomainBody, String>,
    }

    impl<'a> AssignIpToDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::AssignIpToDomainBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AssignIpToDomainBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AssignIpToDomainBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AssignIpToDomainBody,
            ) -> types::builder::AssignIpToDomainBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/domains/{domain}/ips`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AssignIpToDomainBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/ips",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientIPsExt::unassign_ip_from_domain`]
    ///
    ///[`ClientIPsExt::unassign_ip_from_domain`]: super::ClientIPsExt::unassign_ip_from_domain
    #[derive(Debug, Clone)]
    pub struct UnassignIpFromDomain<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        ip: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> UnassignIpFromDomain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                ip: Err("ip was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn ip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.ip = value
                .try_into()
                .map_err(|_| "conversion to `String` for ip failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/domains/{domain}/ips/{ip}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                ip,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let ip = ip.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/ips/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&ip.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientEventsExt::get_events`]
    ///
    ///[`ClientEventsExt::get_events`]: super::ClientEventsExt::get_events
    #[derive(Debug, Clone)]
    pub struct GetEvents<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetEvents<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/v3/{domain}/events`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/v3/{}/events",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientStatsExt::get_domain_stats`]
    ///
    ///[`ClientStatsExt::get_domain_stats`]: super::ClientStatsExt::get_domain_stats
    #[derive(Debug, Clone)]
    pub struct GetDomainStats<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        event: Result<Option<String>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainStats<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                event: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn event<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.event = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for event failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/stats/total`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                event,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let event = event.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/stats/total",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut __progenitor_query = Vec::with_capacity(1usize);
            if let Some(v) = &event {
                __progenitor_query.push(("event", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientStatsExt::get_domain_stat_items`]
    ///
    ///[`ClientStatsExt::get_domain_stat_items`]: super::ClientStatsExt::get_domain_stat_items
    #[derive(Debug, Clone)]
    pub struct GetDomainStatItems<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        limit: Result<Option<i64>, String>,
        skip: Result<Option<i64>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainStatItems<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                limit: Ok(None),
                skip: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for limit failed".to_string());
            self
        }

        pub fn skip<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.skip = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for skip failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/stats`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                limit,
                skip,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let skip = skip.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/stats",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut __progenitor_query = Vec::with_capacity(2usize);
            if let Some(v) = &limit {
                __progenitor_query.push(("limit", v.to_string()));
            }
            if let Some(v) = &skip {
                __progenitor_query.push(("skip", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_domain_tags`]
    ///
    ///[`ClientTagsExt::get_domain_tags`]: super::ClientTagsExt::get_domain_tags
    #[derive(Debug, Clone)]
    pub struct GetDomainTags<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainTags<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/tags`
        pub async fn send(self) -> Result<ResponseValue<types::GetDomainTags>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_domain_tag`]
    ///
    ///[`ClientTagsExt::get_domain_tag`]: super::ClientTagsExt::get_domain_tag
    #[derive(Debug, Clone)]
    pub struct GetDomainTag<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainTag<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/tags/{tag}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::update_domain_tag`]
    ///
    ///[`ClientTagsExt::update_domain_tag`]: super::ClientTagsExt::update_domain_tag
    #[derive(Debug, Clone)]
    pub struct UpdateDomainTag<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateDomainTagBody, String>,
    }

    impl<'a> UpdateDomainTag<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateDomainTagBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateDomainTagBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateDomainTagBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateDomainTagBody,
            ) -> types::builder::UpdateDomainTagBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/{domain}/tags/{tag}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateDomainTagBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::delete_domain_tag`]
    ///
    ///[`ClientTagsExt::delete_domain_tag`]: super::ClientTagsExt::delete_domain_tag
    #[derive(Debug, Clone)]
    pub struct DeleteDomainTag<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteDomainTag<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/tags/{tag}`
        pub async fn send(self) -> Result<ResponseValue<types::DeleteTag>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_domain_providers`]
    ///
    ///[`ClientTagsExt::get_domain_providers`]: super::ClientTagsExt::get_domain_providers
    #[derive(Debug, Clone)]
    pub struct GetDomainProviders<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainProviders<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/{domain}/tags/{tag}/stats/aggregates/providers`
        pub async fn send(self) -> Result<ResponseValue<types::GetProvidersEvents>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}/stats/aggregates/providers",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_tag_stats`]
    ///
    ///[`ClientTagsExt::get_tag_stats`]: super::ClientTagsExt::get_tag_stats
    #[derive(Debug, Clone)]
    pub struct GetTagStats<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        event: Result<Option<String>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetTagStats<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                event: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn event<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.event = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for event failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/tags/{tag}/stats`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                event,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let event = event.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}/stats",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut __progenitor_query = Vec::with_capacity(1usize);
            if let Some(v) = &event {
                __progenitor_query.push(("event", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_domain_countries`]
    ///
    ///[`ClientTagsExt::get_domain_countries`]: super::ClientTagsExt::get_domain_countries
    #[derive(Debug, Clone)]
    pub struct GetDomainCountries<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainCountries<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/{domain}/tags/{tag}/stats/aggregates/countries`
        pub async fn send(self) -> Result<ResponseValue<types::ProviderCountryEvents>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}/stats/aggregates/countries",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientTagsExt::get_domain_devices`]
    ///
    ///[`ClientTagsExt::get_domain_devices`]: super::ClientTagsExt::get_domain_devices
    #[derive(Debug, Clone)]
    pub struct GetDomainDevices<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        tag: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainDevices<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                tag: Err("tag was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.tag = value
                .try_into()
                .map_err(|_| "conversion to `String` for tag failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/{domain}/tags/{tag}/stats/aggregates/devices`
        pub async fn send(self) -> Result<ResponseValue<types::ProviderDeviceEvents>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                tag,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let tag = tag.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/tags/{}/stats/aggregates/devices",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&tag.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::get_bounces`]
    ///
    ///[`ClientSuppressionsBouncesExt::get_bounces`]: super::ClientSuppressionsBouncesExt::get_bounces
    #[derive(Debug, Clone)]
    pub struct GetBounces<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetBounces<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/bounces`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/bounces",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::add_bounce`]
    ///
    ///[`ClientSuppressionsBouncesExt::add_bounce`]: super::ClientSuppressionsBouncesExt::add_bounce
    #[derive(Debug, Clone)]
    pub struct AddBounce<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::AddBounceBody, String>,
    }

    impl<'a> AddBounce<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::AddBounceBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddBounceBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AddBounceBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AddBounceBody) -> types::builder::AddBounceBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/{domain}/bounces`
        pub async fn send(self) -> Result<ResponseValue<types::AddBounceResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddBounceBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/bounces",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::delete_bounce_list`]
    ///
    ///[`ClientSuppressionsBouncesExt::delete_bounce_list`]: super::ClientSuppressionsBouncesExt::delete_bounce_list
    #[derive(Debug, Clone)]
    pub struct DeleteBounceList<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
    }

    impl<'a> DeleteBounceList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/bounces`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/bounces",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::get_bounce`]
    ///
    ///[`ClientSuppressionsBouncesExt::get_bounce`]: super::ClientSuppressionsBouncesExt::get_bounce
    #[derive(Debug, Clone)]
    pub struct GetBounce<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetBounce<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/bounces/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/bounces/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::delete_bounce`]
    ///
    ///[`ClientSuppressionsBouncesExt::delete_bounce`]: super::ClientSuppressionsBouncesExt::delete_bounce
    #[derive(Debug, Clone)]
    pub struct DeleteBounce<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
    }

    impl<'a> DeleteBounce<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/bounces/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
                content_type,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/bounces/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsBouncesExt::add_bounces`]
    ///
    ///[`ClientSuppressionsBouncesExt::add_bounces`]: super::ClientSuppressionsBouncesExt::add_bounces
    #[derive(Debug, Clone)]
    pub struct AddBounces<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
        body: Result<types::AddBouncesRequest, String>,
    }

    impl<'a> AddBounces<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddBouncesRequest>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `AddBouncesRequest` for body failed".to_string());
            self
        }

        ///Sends a `POST` request to `/v3/{domain}/bounces`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/v3/{}/bounces",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsComplaintsExt::get_complaints`]
    ///
    ///[`ClientSuppressionsComplaintsExt::get_complaints`]: super::ClientSuppressionsComplaintsExt::get_complaints
    #[derive(Debug, Clone)]
    pub struct GetComplaints<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetComplaints<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/complaints`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/complaints",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsComplaintsExt::add_complaints`]
    ///
    ///[`ClientSuppressionsComplaintsExt::add_complaints`]: super::ClientSuppressionsComplaintsExt::add_complaints
    #[derive(Debug, Clone)]
    pub struct AddComplaints<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
        body: Result<types::ComplaintRequest, String>,
    }

    impl<'a> AddComplaints<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ComplaintRequest>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `ComplaintRequest` for body failed".to_string());
            self
        }

        ///Sends a `POST` request to `/{domain}/complaints`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/complaints",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsComplaintsExt::get_complaint`]
    ///
    ///[`ClientSuppressionsComplaintsExt::get_complaint`]: super::ClientSuppressionsComplaintsExt::get_complaint
    #[derive(Debug, Clone)]
    pub struct GetComplaint<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
    }

    impl<'a> GetComplaint<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/complaints/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
                content_type,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/complaints/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsComplaintsExt::delete_complaint`]
    ///
    ///[`ClientSuppressionsComplaintsExt::delete_complaint`]: super::ClientSuppressionsComplaintsExt::delete_complaint
    #[derive(Debug, Clone)]
    pub struct DeleteComplaint<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
    }

    impl<'a> DeleteComplaint<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/complaints/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
                content_type,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/complaints/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientSuppressionsComplaintsExt::add_complaint`]
    ///
    ///[`ClientSuppressionsComplaintsExt::add_complaint`]: super::ClientSuppressionsComplaintsExt::add_complaint
    #[derive(Debug, Clone)]
    pub struct AddComplaint<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
        body: Result<types::builder::AddComplaintBody, String>,
    }

    impl<'a> AddComplaint<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
                body: Ok(types::builder::AddComplaintBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddComplaintBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AddComplaintBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AddComplaintBody,
            ) -> types::builder::AddComplaintBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/v3/{domain}/complaints`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddComplaintBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/v3/{}/complaints",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for
    /// [`ClientSuppressionsUnsubscribesExt::get_domain_unsubscribes`]
    ///
    ///[`ClientSuppressionsUnsubscribesExt::get_domain_unsubscribes`]: super::ClientSuppressionsUnsubscribesExt::get_domain_unsubscribes
    #[derive(Debug, Clone)]
    pub struct GetDomainUnsubscribes<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainUnsubscribes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/unsubscribes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/unsubscribes",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for
    /// [`ClientSuppressionsUnsubscribesExt::add_domain_unsubscribes`]
    ///
    ///[`ClientSuppressionsUnsubscribesExt::add_domain_unsubscribes`]: super::ClientSuppressionsUnsubscribesExt::add_domain_unsubscribes
    #[derive(Debug, Clone)]
    pub struct AddDomainUnsubscribes<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        content_type: Result<Option<String>, String>,
        body: Result<types::UnsubscribeRequest, String>,
    }

    impl<'a> AddDomainUnsubscribes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                content_type: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn content_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.content_type = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for content_type failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UnsubscribeRequest>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `UnsubscribeRequest` for body failed".to_string());
            self
        }

        ///Sends a `POST` request to `/{domain}/unsubscribes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                content_type,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let content_type = content_type.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/unsubscribes",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(2usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            if let Some(v) = content_type {
                header_map.append("Content-Type", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for
    /// [`ClientSuppressionsUnsubscribesExt::get_domain_unsubscribe`]
    ///
    ///[`ClientSuppressionsUnsubscribesExt::get_domain_unsubscribe`]: super::ClientSuppressionsUnsubscribesExt::get_domain_unsubscribe
    #[derive(Debug, Clone)]
    pub struct GetDomainUnsubscribe<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetDomainUnsubscribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/{domain}/unsubscribes/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/unsubscribes/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for
    /// [`ClientSuppressionsUnsubscribesExt::delete_domain_unsubscribe`]
    ///
    ///[`ClientSuppressionsUnsubscribesExt::delete_domain_unsubscribe`]: super::ClientSuppressionsUnsubscribesExt::delete_domain_unsubscribe
    #[derive(Debug, Clone)]
    pub struct DeleteDomainUnsubscribe<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteDomainUnsubscribe<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/unsubscribes/{address}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                address,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/unsubscribes/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientRoutesExt::get_routes`]
    ///
    ///[`ClientRoutesExt::get_routes`]: super::ClientRoutesExt::get_routes
    #[derive(Debug, Clone)]
    pub struct GetRoutes<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetRoutes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/routes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/routes", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientRoutesExt::create_route`]
    ///
    ///[`ClientRoutesExt::create_route`]: super::ClientRoutesExt::create_route
    #[derive(Debug, Clone)]
    pub struct CreateRoute<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::CreateRouteBody, String>,
    }

    impl<'a> CreateRoute<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
                body: Ok(types::builder::CreateRouteBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateRouteBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `CreateRouteBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::CreateRouteBody) -> types::builder::CreateRouteBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/routes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<ByteStream>, Error<serde_json::Map<String, serde_json::Value>>>
        {
            let Self {
                __progenitor_client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateRouteBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/routes", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(__progenitor_response)),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(__progenitor_response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientRoutesExt::update_route`]
    ///
    ///[`ClientRoutesExt::update_route`]: super::ClientRoutesExt::update_route
    #[derive(Debug, Clone)]
    pub struct UpdateRoute<'a> {
        __progenitor_client: &'a super::Client,
        route: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateRouteBody, String>,
    }

    impl<'a> UpdateRoute<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                route: Err("route was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateRouteBody::default()),
            }
        }

        pub fn route<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.route = value
                .try_into()
                .map_err(|_| "conversion to `String` for route failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateRouteBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateRouteBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UpdateRouteBody) -> types::builder::UpdateRouteBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/routes/{route}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                route,
                authorization,
                body,
            } = self;
            let route = route.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateRouteBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/routes/{}",
                __progenitor_client.baseurl,
                encode_path(&route.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientRoutesExt::get_route`]
    ///
    ///[`ClientRoutesExt::get_route`]: super::ClientRoutesExt::get_route
    #[derive(Debug, Clone)]
    pub struct GetRoute<'a> {
        __progenitor_client: &'a super::Client,
        route: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<serde_json::Map<String, serde_json::Value>, String>,
    }

    impl<'a> GetRoute<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                route: Err("route was not initialized".to_string()),
                authorization: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn route<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.route = value
                .try_into()
                .map_err(|_| "conversion to `String` for route failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<serde_json::Map<String, serde_json::Value>>,
        {
            self . body = value . try_into () . map_err (| _ | "conversion to `serde_json :: Map < String , serde_json :: Value >` for body failed" . to_string ()) ;
            self
        }

        ///Sends a `POST` request to `/routes/{route}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                route,
                authorization,
                body,
            } = self;
            let route = route.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/routes/{}",
                __progenitor_client.baseurl,
                encode_path(&route.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientRoutesExt::delete_route`]
    ///
    ///[`ClientRoutesExt::delete_route`]: super::ClientRoutesExt::delete_route
    #[derive(Debug, Clone)]
    pub struct DeleteRoute<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        id: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteRoute<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                id: Err("id was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.id = value
                .try_into()
                .map_err(|_| "conversion to `String` for id failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/{domain}/routes/{id}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                id,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let id = id.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/{}/routes/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&id.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientWebhooksExt::get_webhooks`]
    ///
    ///[`ClientWebhooksExt::get_webhooks`]: super::ClientWebhooksExt::get_webhooks
    #[derive(Debug, Clone)]
    pub struct GetWebhooks<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetWebhooks<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}/webhooks`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/webhooks",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientWebhooksExt::create_webhook`]
    ///
    ///[`ClientWebhooksExt::create_webhook`]: super::ClientWebhooksExt::create_webhook
    #[derive(Debug, Clone)]
    pub struct CreateWebhook<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::CreateWebhookBody, String>,
    }

    impl<'a> CreateWebhook<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::CreateWebhookBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateWebhookBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `CreateWebhookBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::CreateWebhookBody,
            ) -> types::builder::CreateWebhookBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/domains/{domain}/webhooks`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateWebhookBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/webhooks",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientWebhooksExt::get_webhook_details`]
    ///
    ///[`ClientWebhooksExt::get_webhook_details`]: super::ClientWebhooksExt::get_webhook_details
    #[derive(Debug, Clone)]
    pub struct GetWebhookDetails<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        webhookname: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetWebhookDetails<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                webhookname: Err("webhookname was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn webhookname<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.webhookname = value
                .try_into()
                .map_err(|_| "conversion to `String` for webhookname failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/domains/{domain}/webhooks/{webhookname}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                webhookname,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let webhookname = webhookname.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/webhooks/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&webhookname.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientWebhooksExt::update_webhook`]
    ///
    ///[`ClientWebhooksExt::update_webhook`]: super::ClientWebhooksExt::update_webhook
    #[derive(Debug, Clone)]
    pub struct UpdateWebhook<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        webhookname: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateWebhookBody, String>,
    }

    impl<'a> UpdateWebhook<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                webhookname: Err("webhookname was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateWebhookBody::default()),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn webhookname<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.webhookname = value
                .try_into()
                .map_err(|_| "conversion to `String` for webhookname failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateWebhookBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateWebhookBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateWebhookBody,
            ) -> types::builder::UpdateWebhookBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/domains/{domain}/webhooks/{webhookname}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                webhookname,
                authorization,
                body,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let webhookname = webhookname.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateWebhookBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/webhooks/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&webhookname.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientWebhooksExt::delete_webhook`]
    ///
    ///[`ClientWebhooksExt::delete_webhook`]: super::ClientWebhooksExt::delete_webhook
    #[derive(Debug, Clone)]
    pub struct DeleteWebhook<'a> {
        __progenitor_client: &'a super::Client,
        domain: Result<String, String>,
        webhookname: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteWebhook<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                domain: Err("domain was not initialized".to_string()),
                webhookname: Err("webhookname was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn domain<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.domain = value
                .try_into()
                .map_err(|_| "conversion to `String` for domain failed".to_string());
            self
        }

        pub fn webhookname<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.webhookname = value
                .try_into()
                .map_err(|_| "conversion to `String` for webhookname failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to
        /// `/domains/{domain}/webhooks/{webhookname}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                domain,
                webhookname,
                authorization,
            } = self;
            let domain = domain.map_err(Error::InvalidRequest)?;
            let webhookname = webhookname.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/domains/{}/webhooks/{}",
                __progenitor_client.baseurl,
                encode_path(&domain.to_string()),
                encode_path(&webhookname.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::get_lists`]
    ///
    ///[`ClientMailingListsExt::get_lists`]: super::ClientMailingListsExt::get_lists
    #[derive(Debug, Clone)]
    pub struct GetLists<'a> {
        __progenitor_client: &'a super::Client,
        limit: Result<Option<i64>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetLists<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                limit: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for limit failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/lists/pages`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                limit,
                authorization,
            } = self;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/lists/pages", __progenitor_client.baseurl,);
            let mut __progenitor_query = Vec::with_capacity(1usize);
            if let Some(v) = &limit {
                __progenitor_query.push(("limit", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::get_list`]
    ///
    ///[`ClientMailingListsExt::get_list`]: super::ClientMailingListsExt::get_list
    #[derive(Debug, Clone)]
    pub struct GetList<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/lists/{address}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                authorization,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::update_list`]
    ///
    ///[`ClientMailingListsExt::update_list`]: super::ClientMailingListsExt::update_list
    #[derive(Debug, Clone)]
    pub struct UpdateList<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateListBody, String>,
    }

    impl<'a> UpdateList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateListBody::default()),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateListBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateListBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::UpdateListBody) -> types::builder::UpdateListBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/lists/{address}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                authorization,
                body,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateListBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::delete_list`]
    ///
    ///[`ClientMailingListsExt::delete_list`]: super::ClientMailingListsExt::delete_list
    #[derive(Debug, Clone)]
    pub struct DeleteList<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to `/lists/{address}`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                authorization,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::create_list`]
    ///
    ///[`ClientMailingListsExt::create_list`]: super::ClientMailingListsExt::create_list
    #[derive(Debug, Clone)]
    pub struct CreateList<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::CreateListBody, String>,
    }

    impl<'a> CreateList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
                body: Ok(types::builder::CreateListBody::default()),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateListBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `CreateListBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::CreateListBody) -> types::builder::CreateListBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/lists`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
                body,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateListBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/lists", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::get_list_members`]
    ///
    ///[`ClientMailingListsExt::get_list_members`]: super::ClientMailingListsExt::get_list_members
    #[derive(Debug, Clone)]
    pub struct GetListMembers<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        limit: Result<Option<i64>, String>,
        subscribed: Result<Option<String>, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetListMembers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                limit: Ok(None),
                subscribed: Ok(None),
                authorization: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for limit failed".to_string());
            self
        }

        pub fn subscribed<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.subscribed = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for subscribed failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/lists/{address}/members/pages`
        pub async fn send(self) -> Result<ResponseValue<types::ListMembersResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                limit,
                subscribed,
                authorization,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let subscribed = subscribed.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members/pages",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut __progenitor_query = Vec::with_capacity(2usize);
            if let Some(v) = &limit {
                __progenitor_query.push(("limit", v.to_string()));
            }
            if let Some(v) = &subscribed {
                __progenitor_query.push(("subscribed", v.to_string()));
            }
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::get_list_member`]
    ///
    ///[`ClientMailingListsExt::get_list_member`]: super::ClientMailingListsExt::get_list_member
    #[derive(Debug, Clone)]
    pub struct GetListMember<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        memberaddress: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> GetListMember<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                memberaddress: Err("memberaddress was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn memberaddress<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.memberaddress = value
                .try_into()
                .map_err(|_| "conversion to `String` for memberaddress failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/lists/{address}/members/{memberaddress}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ListMemberAddressResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                memberaddress,
                authorization,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let memberaddress = memberaddress.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
                encode_path(&memberaddress.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::update_list_member`]
    ///
    ///[`ClientMailingListsExt::update_list_member`]: super::ClientMailingListsExt::update_list_member
    #[derive(Debug, Clone)]
    pub struct UpdateListMember<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        memberaddress: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::UpdateListMemberBody, String>,
    }

    impl<'a> UpdateListMember<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                memberaddress: Err("memberaddress was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::UpdateListMemberBody::default()),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn memberaddress<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.memberaddress = value
                .try_into()
                .map_err(|_| "conversion to `String` for memberaddress failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::UpdateListMemberBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `UpdateListMemberBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::UpdateListMemberBody,
            ) -> types::builder::UpdateListMemberBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to `/lists/{address}/members/{memberaddress}`
        pub async fn send(self) -> Result<ResponseValue<types::UpdateMailingList>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                memberaddress,
                authorization,
                body,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let memberaddress = memberaddress.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::UpdateListMemberBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
                encode_path(&memberaddress.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .put(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::delete_list_member`]
    ///
    ///[`ClientMailingListsExt::delete_list_member`]: super::ClientMailingListsExt::delete_list_member
    #[derive(Debug, Clone)]
    pub struct DeleteListMember<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        memberaddress: Result<String, String>,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> DeleteListMember<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                memberaddress: Err("memberaddress was not initialized".to_string()),
                authorization: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn memberaddress<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.memberaddress = value
                .try_into()
                .map_err(|_| "conversion to `String` for memberaddress failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `DELETE` request to
        /// `/lists/{address}/members/{memberaddress}`
        pub async fn send(self) -> Result<ResponseValue<types::DeleteListMember>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                memberaddress,
                authorization,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let memberaddress = memberaddress.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members/{}",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
                encode_path(&memberaddress.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .delete(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::add_list_member`]
    ///
    ///[`ClientMailingListsExt::add_list_member`]: super::ClientMailingListsExt::add_list_member
    #[derive(Debug, Clone)]
    pub struct AddListMember<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::AddListMemberBody, String>,
    }

    impl<'a> AddListMember<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::AddListMemberBody::default()),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddListMemberBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AddListMemberBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AddListMemberBody,
            ) -> types::builder::AddListMemberBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/lists/{address}/members`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<ByteStream>, Error<serde_json::Map<String, serde_json::Value>>>
        {
            let Self {
                __progenitor_client,
                address,
                authorization,
                body,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddListMemberBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(__progenitor_response)),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(__progenitor_response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientMailingListsExt::add_list_members`]
    ///
    ///[`ClientMailingListsExt::add_list_members`]: super::ClientMailingListsExt::add_list_members
    #[derive(Debug, Clone)]
    pub struct AddListMembers<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<String, String>,
        authorization: Result<Option<String>, String>,
        body: Result<types::builder::AddListMembersBody, String>,
    }

    impl<'a> AddListMembers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Err("address was not initialized".to_string()),
                authorization: Ok(None),
                body: Ok(types::builder::AddListMembersBody::default()),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddListMembersBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AddListMembersBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AddListMembersBody,
            ) -> types::builder::AddListMembersBody,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/lists/{address}/members.json`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                authorization,
                body,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddListMembersBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!(
                "{}/lists/{}/members.json",
                __progenitor_client.baseurl,
                encode_path(&address.to_string()),
            );
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .post(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientEmailValidationExt::validate_address`]
    ///
    ///[`ClientEmailValidationExt::validate_address`]: super::ClientEmailValidationExt::validate_address
    #[derive(Debug, Clone)]
    pub struct ValidateAddress<'a> {
        __progenitor_client: &'a super::Client,
        address: Result<Option<String>, String>,
        api_key: Result<Option<String>, String>,
        mailbox_verification: Result<Option<bool>, String>,
    }

    impl<'a> ValidateAddress<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                address: Ok(None),
                api_key: Ok(None),
                mailbox_verification: Ok(None),
            }
        }

        pub fn address<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.address = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for address failed".to_string());
            self
        }

        pub fn api_key<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.api_key = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for api_key failed".to_string());
            self
        }

        pub fn mailbox_verification<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.mailbox_verification = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for mailbox_verification failed".to_string());
            self
        }

        ///Sends a `GET` request to `/address/validate`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
                address,
                api_key,
                mailbox_verification,
            } = self;
            let address = address.map_err(Error::InvalidRequest)?;
            let api_key = api_key.map_err(Error::InvalidRequest)?;
            let mailbox_verification = mailbox_verification.map_err(Error::InvalidRequest)?;
            let __progenitor_url = format!("{}/address/validate", __progenitor_client.baseurl,);
            let mut __progenitor_query = Vec::with_capacity(3usize);
            if let Some(v) = &address {
                __progenitor_query.push(("address", v.to_string()));
            }
            if let Some(v) = &api_key {
                __progenitor_query.push(("api_key", v.to_string()));
            }
            if let Some(v) = &mailbox_verification {
                __progenitor_query.push(("mailbox_verification", v.to_string()));
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&__progenitor_query)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientEmailValidationExt::parse_list`]
    ///
    ///[`ClientEmailValidationExt::parse_list`]: super::ClientEmailValidationExt::parse_list
    #[derive(Debug, Clone)]
    pub struct ParseList<'a> {
        __progenitor_client: &'a super::Client,
    }

    impl<'a> ParseList<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
            }
        }

        ///Sends a `GET` request to `/address/parse`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
            let Self {
                __progenitor_client,
            } = self;
            let __progenitor_url = format!("{}/address/parse", __progenitor_client.baseurl,);
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientEmailValidationExt::validate_address_private`]
    ///
    ///[`ClientEmailValidationExt::validate_address_private`]: super::ClientEmailValidationExt::validate_address_private
    #[derive(Debug, Clone)]
    pub struct ValidateAddressPrivate<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> ValidateAddressPrivate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/address/private/validate`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url =
                format!("{}/address/private/validate", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }

    ///Builder for [`ClientEmailValidationExt::parse_list_private`]
    ///
    ///[`ClientEmailValidationExt::parse_list_private`]: super::ClientEmailValidationExt::parse_list_private
    #[derive(Debug, Clone)]
    pub struct ParseListPrivate<'a> {
        __progenitor_client: &'a super::Client,
        authorization: Result<Option<String>, String>,
    }

    impl<'a> ParseListPrivate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                __progenitor_client: client,
                authorization: Ok(None),
            }
        }

        pub fn authorization<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.authorization = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for authorization failed".to_string());
            self
        }

        ///Sends a `GET` request to `/address/private/varse`
        pub async fn send(self) -> Result<ResponseValue<types::SuccessResponse>, Error<()>> {
            let Self {
                __progenitor_client,
                authorization,
            } = self;
            let authorization = authorization.map_err(Error::InvalidRequest)?;
            let __progenitor_url =
                format!("{}/address/private/varse", __progenitor_client.baseurl,);
            let mut header_map = HeaderMap::with_capacity(1usize);
            if let Some(v) = authorization {
                header_map.append("Authorization", HeaderValue::try_from(v)?);
            }
            let __progenitor_request = __progenitor_client
                .client
                .get(__progenitor_url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let __progenitor_result = __progenitor_client
                .client
                .execute(__progenitor_request)
                .await;
            let __progenitor_response = __progenitor_result?;
            match __progenitor_response.status().as_u16() {
                200u16 => ResponseValue::from_response(__progenitor_response).await,
                _ => Err(Error::UnexpectedResponse(__progenitor_response)),
            }
        }
    }
}

pub mod prelude {
    pub use super::Client;
    pub use super::ClientDomainsExt;
    pub use super::ClientEmailValidationExt;
    pub use super::ClientEventsExt;
    pub use super::ClientIPsExt;
    pub use super::ClientMailingListsExt;
    pub use super::ClientMessagesExt;
    pub use super::ClientRoutesExt;
    pub use super::ClientStatsExt;
    pub use super::ClientSuppressionsBouncesExt;
    pub use super::ClientSuppressionsComplaintsExt;
    pub use super::ClientSuppressionsUnsubscribesExt;
    pub use super::ClientTagsExt;
    pub use super::ClientWebhooksExt;
}
