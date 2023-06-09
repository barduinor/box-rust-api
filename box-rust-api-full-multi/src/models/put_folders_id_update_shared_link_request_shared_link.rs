/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// PutFoldersIdUpdateSharedLinkRequestSharedLink : The settings for the shared link to update.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PutFoldersIdUpdateSharedLinkRequestSharedLink {
    /// The level of access for the shared link. This can be restricted to anyone with the link (`open`), only people within the company (`company`) and only those who have been invited to the folder (`collaborators`).  If not set, this field defaults to the access level specified by the enterprise admin. To create a shared link with this default setting pass the `shared_link` object with no `access` field, for example `{ \"shared_link\": {} }`.  The `company` access level is only available to paid accounts.
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<Access>,
    /// The password required to access the shared link. Set the password to `null` to remove it.  A password can only be set when `access` is set to `open`.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Defines a custom vanity name to use in the shared link URL, for example `https://app.box.com/v/my-shared-link`.  Custom URLs should not be used when sharing sensitive content as vanity URLs are a lot easier to guess than regular shared links.
    #[serde(rename = "vanity_name", skip_serializing_if = "Option::is_none")]
    pub vanity_name: Option<String>,
    /// The timestamp at which this shared link will expire. This field can only be set by users with paid accounts. The value must be greater than the current date and time.
    #[serde(rename = "unshared_at", skip_serializing_if = "Option::is_none")]
    pub unshared_at: Option<String>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Box<crate::models::PutFoldersIdAddSharedLinkRequestSharedLinkPermissions>>,
}

impl PutFoldersIdUpdateSharedLinkRequestSharedLink {
    /// The settings for the shared link to update.
    pub fn new() -> PutFoldersIdUpdateSharedLinkRequestSharedLink {
        PutFoldersIdUpdateSharedLinkRequestSharedLink {
            access: None,
            password: None,
            vanity_name: None,
            unshared_at: None,
            permissions: None,
        }
    }
}

/// The level of access for the shared link. This can be restricted to anyone with the link (`open`), only people within the company (`company`) and only those who have been invited to the folder (`collaborators`).  If not set, this field defaults to the access level specified by the enterprise admin. To create a shared link with this default setting pass the `shared_link` object with no `access` field, for example `{ \"shared_link\": {} }`.  The `company` access level is only available to paid accounts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Access {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "company")]
    Company,
    #[serde(rename = "collaborators")]
    Collaborators,
}

impl Default for Access {
    fn default() -> Access {
        Self::Open
    }
}

