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
pub struct BaseInfoDeleteTagReq {
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "tagID")]
    pub tag_id: String,
}

impl BaseInfoDeleteTagReq {
    pub fn new(operation_id: String, tag_id: String) -> BaseInfoDeleteTagReq {
        BaseInfoDeleteTagReq {
            operation_id,
            tag_id,
        }
    }
}


