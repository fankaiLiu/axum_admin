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
pub struct BaseInfoSetFriendRemarkReq {
    #[serde(rename = "fromUserID")]
    pub from_user_id: String,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    #[serde(rename = "toUserID")]
    pub to_user_id: String,
}

impl BaseInfoSetFriendRemarkReq {
    pub fn new(from_user_id: String, operation_id: String, to_user_id: String) -> BaseInfoSetFriendRemarkReq {
        BaseInfoSetFriendRemarkReq {
            from_user_id,
            operation_id,
            remark: None,
            to_user_id,
        }
    }
}


