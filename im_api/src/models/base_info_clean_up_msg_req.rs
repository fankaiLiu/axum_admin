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
pub struct BaseInfoCleanUpMsgReq {
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl BaseInfoCleanUpMsgReq {
    pub fn new(operation_id: String, user_id: String) -> BaseInfoCleanUpMsgReq {
        BaseInfoCleanUpMsgReq {
            operation_id,
            user_id,
        }
    }
}


