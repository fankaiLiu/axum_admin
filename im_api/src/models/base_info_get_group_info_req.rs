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
pub struct BaseInfoGetGroupInfoReq {
    #[serde(rename = "groupIDList")]
    pub group_id_list: Vec<String>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
}

impl BaseInfoGetGroupInfoReq {
    pub fn new(group_id_list: Vec<String>, operation_id: String) -> BaseInfoGetGroupInfoReq {
        BaseInfoGetGroupInfoReq {
            group_id_list,
            operation_id,
        }
    }
}


