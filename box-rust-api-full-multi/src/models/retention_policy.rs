/*
 * Box Platform API
 *
 * [Box Platform](https://box.dev) provides functionality to provide access to content stored within [Box](https://box.com). It provides endpoints for basic manipulation of files and folders, management of users within an enterprise, as well as more complex topics such as legal holds and retention policies.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: devrel@box.com
 * Generated by: https://openapi-generator.tech
 */

/// RetentionPolicy : A retention policy blocks permanent deletion of content for a specified amount of time. Admins can create retention policies and then later assign them to specific folders, metadata templates, or their entire enterprise.  To use this feature, you must have the manage retention policies scope enabled for your API key via your application management console.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// The unique identifier that represents a retention policy.
    #[serde(rename = "id")]
    pub id: String,
    /// `retention_policy`
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// The name given to the retention policy.
    #[serde(rename = "policy_name", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    /// The length of the retention policy. This value specifies the duration in days that the retention policy will be active for after being assigned to content.  If the policy has a `policy_type` of `indefinite`, the `retention_length` will also be `indefinite`.
    #[serde(rename = "retention_length", skip_serializing_if = "Option::is_none")]
    pub retention_length: Option<String>,
    /// The disposition action of the retention policy. This action can be `permanently_delete`, which will cause the content retained by the policy to be permanently deleted, or `remove_retention`, which will lift the retention policy from the content, allowing it to be deleted by users, once the retention policy has expired.
    #[serde(rename = "disposition_action", skip_serializing_if = "Option::is_none")]
    pub disposition_action: Option<DispositionAction>,
    /// The additional text description of the retention policy.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The type of the retention policy. A retention policy type can either be `finite`, where a specific amount of time to retain the content is known upfront, or `indefinite`, where the amount of time to retain the content is still unknown.
    #[serde(rename = "policy_type", skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<PolicyType>,
    /// Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example,  you can add or remove folders, shorten or lengthen  the policy duration, or delete the assignment.  Use this type if your retention policy  is not related to any regulatory purposes.  * `non-modifiable`: You can modify the retention policy  only in a limited way: add a folder, lengthen the duration,  retire the policy, change the disposition action  or notification settings. You cannot perform other actions,  such as deleting the assignment or shortening the  policy duration. Use this type to ensure  compliance with regulatory retention policies.
    #[serde(rename = "retention_type", skip_serializing_if = "Option::is_none")]
    pub retention_type: Option<RetentionType>,
    /// The status of the retention policy. The status of a policy will be `active`, unless explicitly retired by an administrator, in which case the status will be `retired`. Once a policy has been retired, it cannot become active again.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<Box<crate::models::RetentionPolicyAllOfCreatedBy>>,
    /// When the retention policy object was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// When the retention policy object was last modified.
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    /// Determines if the owner of items under the policy can extend the retention when the original retention duration is about to end.
    #[serde(rename = "can_owner_extend_retention", skip_serializing_if = "Option::is_none")]
    pub can_owner_extend_retention: Option<bool>,
    /// Determines if owners and co-owners of items under the policy are notified when the retention duration is about to end.
    #[serde(rename = "are_owners_notified", skip_serializing_if = "Option::is_none")]
    pub are_owners_notified: Option<bool>,
    /// A list of users notified when the retention policy duration is about to end.
    #[serde(rename = "custom_notification_recipients", skip_serializing_if = "Option::is_none")]
    pub custom_notification_recipients: Option<Vec<crate::models::UserMini>>,
    #[serde(rename = "assignment_counts", skip_serializing_if = "Option::is_none")]
    pub assignment_counts: Option<Box<crate::models::RetentionPolicyAllOfAssignmentCounts>>,
}

impl RetentionPolicy {
    /// A retention policy blocks permanent deletion of content for a specified amount of time. Admins can create retention policies and then later assign them to specific folders, metadata templates, or their entire enterprise.  To use this feature, you must have the manage retention policies scope enabled for your API key via your application management console.
    pub fn new(id: String, r#type: RHashType) -> RetentionPolicy {
        RetentionPolicy {
            id,
            r#type,
            policy_name: None,
            retention_length: None,
            disposition_action: None,
            description: None,
            policy_type: None,
            retention_type: None,
            status: None,
            created_by: None,
            created_at: None,
            modified_at: None,
            can_owner_extend_retention: None,
            are_owners_notified: None,
            custom_notification_recipients: None,
            assignment_counts: None,
        }
    }
}

/// `retention_policy`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "retention_policy")]
    RetentionPolicy,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::RetentionPolicy
    }
}
/// The disposition action of the retention policy. This action can be `permanently_delete`, which will cause the content retained by the policy to be permanently deleted, or `remove_retention`, which will lift the retention policy from the content, allowing it to be deleted by users, once the retention policy has expired.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DispositionAction {
    #[serde(rename = "permanently_delete")]
    PermanentlyDelete,
    #[serde(rename = "remove_retention")]
    RemoveRetention,
}

impl Default for DispositionAction {
    fn default() -> DispositionAction {
        Self::PermanentlyDelete
    }
}
/// The type of the retention policy. A retention policy type can either be `finite`, where a specific amount of time to retain the content is known upfront, or `indefinite`, where the amount of time to retain the content is still unknown.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyType {
    #[serde(rename = "finite")]
    Finite,
    #[serde(rename = "indefinite")]
    Indefinite,
}

impl Default for PolicyType {
    fn default() -> PolicyType {
        Self::Finite
    }
}
/// Specifies the retention type:  * `modifiable`: You can modify the retention policy. For example,  you can add or remove folders, shorten or lengthen  the policy duration, or delete the assignment.  Use this type if your retention policy  is not related to any regulatory purposes.  * `non-modifiable`: You can modify the retention policy  only in a limited way: add a folder, lengthen the duration,  retire the policy, change the disposition action  or notification settings. You cannot perform other actions,  such as deleting the assignment or shortening the  policy duration. Use this type to ensure  compliance with regulatory retention policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RetentionType {
    #[serde(rename = "modifiable")]
    Modifiable,
    #[serde(rename = "non-modifiable")]
    NonModifiable,
}

impl Default for RetentionType {
    fn default() -> RetentionType {
        Self::Modifiable
    }
}
/// The status of the retention policy. The status of a policy will be `active`, unless explicitly retired by an administrator, in which case the status will be `retired`. Once a policy has been retired, it cannot become active again.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "retired")]
    Retired,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

