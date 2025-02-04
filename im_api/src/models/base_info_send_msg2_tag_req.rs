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
pub struct BaseInfoSendMsg2TagReq {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "groupList", skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<String>>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "senderPlatformID")]
    pub sender_platform_id: i32,
    #[serde(rename = "tagList", skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(rename = "userList", skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<String>>,
}

impl BaseInfoSendMsg2TagReq {
    pub fn new(content: String, operation_id: String, sender_platform_id: i32) -> BaseInfoSendMsg2TagReq {
        BaseInfoSendMsg2TagReq {
            content,
            group_list: None,
            operation_id,
            sender_platform_id,
            tag_list: None,
            user_list: None,
        }
    }
}


