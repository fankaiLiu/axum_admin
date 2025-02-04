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
pub struct BaseInfoCancelMuteGroupResp {
    #[serde(rename = "errCode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errMsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
}

impl BaseInfoCancelMuteGroupResp {
    pub fn new() -> BaseInfoCancelMuteGroupResp {
        BaseInfoCancelMuteGroupResp {
            err_code: None,
            err_msg: None,
        }
    }
}


