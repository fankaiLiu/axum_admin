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
pub struct BaseInfoImportFriendReq {
    #[serde(rename = "friendUserIDList")]
    pub friend_user_id_list: Vec<String>,
    #[serde(rename = "fromUserID")]
    pub from_user_id: String,
    #[serde(rename = "operationID")]
    pub operation_id: String,
}

impl BaseInfoImportFriendReq {
    pub fn new(friend_user_id_list: Vec<String>, from_user_id: String, operation_id: String) -> BaseInfoImportFriendReq {
        BaseInfoImportFriendReq {
            friend_user_id_list,
            from_user_id,
            operation_id,
        }
    }
}


