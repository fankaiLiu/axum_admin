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
pub struct BaseInfoUserTokenReq {
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "platform")]
    pub platform: i32,
    #[serde(rename = "secret")]
    pub secret: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl BaseInfoUserTokenReq {
    pub fn new(operation_id: String, platform: i32, secret: String, user_id: String) -> BaseInfoUserTokenReq {
        BaseInfoUserTokenReq {
            operation_id,
            platform,
            secret,
            user_id,
        }
    }
}


