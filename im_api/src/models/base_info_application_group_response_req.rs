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
pub struct BaseInfoApplicationGroupResponseReq {
    /// application from FromUserID
    #[serde(rename = "fromUserID")]
    pub from_user_id: String,
    #[serde(rename = "groupID")]
    pub group_id: String,
    #[serde(rename = "handleResult")]
    pub handle_result: HandleResult,
    #[serde(rename = "handledMsg", skip_serializing_if = "Option::is_none")]
    pub handled_msg: Option<String>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
}

impl BaseInfoApplicationGroupResponseReq {
    pub fn new(from_user_id: String, group_id: String, handle_result: HandleResult, operation_id: String) -> BaseInfoApplicationGroupResponseReq {
        BaseInfoApplicationGroupResponseReq {
            from_user_id,
            group_id,
            handle_result,
            handled_msg: None,
            operation_id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HandleResult {
    #[serde(rename = "-1")]
    _1,
    // #[serde(rename = "1")]
    // _1,
}

