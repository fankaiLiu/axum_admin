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
pub struct BaseInfoGetUsersOnlineStatusReq {
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "userIDList")]
    pub user_id_list: Vec<String>,
}

impl BaseInfoGetUsersOnlineStatusReq {
    pub fn new(operation_id: String, user_id_list: Vec<String>) -> BaseInfoGetUsersOnlineStatusReq {
        BaseInfoGetUsersOnlineStatusReq {
            operation_id,
            user_id_list,
        }
    }
}


