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
pub struct OfficeComment {
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentID", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    #[serde(rename = "faceURL", skip_serializing_if = "Option::is_none")]
    pub face_url: Option<String>,
    #[serde(rename = "replyUserID", skip_serializing_if = "Option::is_none")]
    pub reply_user_id: Option<String>,
    #[serde(rename = "replyUserName", skip_serializing_if = "Option::is_none")]
    pub reply_user_name: Option<String>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl OfficeComment {
    pub fn new() -> OfficeComment {
        OfficeComment {
            content: None,
            content_id: None,
            create_time: None,
            face_url: None,
            reply_user_id: None,
            reply_user_name: None,
            user_id: None,
            user_name: None,
        }
    }
}


