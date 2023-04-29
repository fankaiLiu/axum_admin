/*
 * 采购系统
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseInfoWorkMoment {
    #[serde(rename = "atUsers", skip_serializing_if = "Option::is_none")]
    pub at_users: Option<Vec<crate::models::BaseInfoWorkMomentUser>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::BaseInfoComment>>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    #[serde(rename = "faceURL", skip_serializing_if = "Option::is_none")]
    pub face_url: Option<String>,
    #[serde(rename = "likeUsers", skip_serializing_if = "Option::is_none")]
    pub like_users: Option<Vec<crate::models::BaseInfoWorkMomentUser>>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    #[serde(rename = "permissionUsers", skip_serializing_if = "Option::is_none")]
    pub permission_users: Option<Vec<crate::models::BaseInfoWorkMomentUser>>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "workMomentID", skip_serializing_if = "Option::is_none")]
    pub work_moment_id: Option<String>,
}

impl BaseInfoWorkMoment {
    pub fn new() -> BaseInfoWorkMoment {
        BaseInfoWorkMoment {
            at_users: None,
            comments: None,
            content: None,
            create_time: None,
            face_url: None,
            like_users: None,
            permission: None,
            permission_users: None,
            user_id: None,
            user_name: None,
            work_moment_id: None,
        }
    }
}


