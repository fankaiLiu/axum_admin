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
pub struct BaseInfoDismissGroupReq {
    #[serde(rename = "groupID")]
    pub group_id: String,
    #[serde(rename = "operationID")]
    pub operation_id: String,
}

impl BaseInfoDismissGroupReq {
    pub fn new(group_id: String, operation_id: String) -> BaseInfoDismissGroupReq {
        BaseInfoDismissGroupReq {
            group_id,
            operation_id,
        }
    }
}


