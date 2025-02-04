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
pub struct BaseInfoKickGroupMemberReq {
    #[serde(rename = "groupID")]
    pub group_id: String,
    #[serde(rename = "kickedUserIDList")]
    pub kicked_user_id_list: Vec<String>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl BaseInfoKickGroupMemberReq {
    pub fn new(group_id: String, kicked_user_id_list: Vec<String>, operation_id: String) -> BaseInfoKickGroupMemberReq {
        BaseInfoKickGroupMemberReq {
            group_id,
            kicked_user_id_list,
            operation_id,
            reason: None,
        }
    }
}


