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
pub struct BaseInfoMuteGroupMemberReq {
    #[serde(rename = "groupID")]
    pub group_id: String,
    #[serde(rename = "mutedSeconds")]
    pub muted_seconds: i32,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl BaseInfoMuteGroupMemberReq {
    pub fn new(group_id: String, muted_seconds: i32, operation_id: String, user_id: String) -> BaseInfoMuteGroupMemberReq {
        BaseInfoMuteGroupMemberReq {
            group_id,
            muted_seconds,
            operation_id,
            user_id,
        }
    }
}


