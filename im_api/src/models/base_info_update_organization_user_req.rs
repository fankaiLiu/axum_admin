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
pub struct BaseInfoUpdateOrganizationUserReq {
    #[serde(rename = "birth", skip_serializing_if = "Option::is_none")]
    pub birth: Option<i32>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "englishName", skip_serializing_if = "Option::is_none")]
    pub english_name: Option<String>,
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
    #[serde(rename = "faceURL", skip_serializing_if = "Option::is_none")]
    pub face_url: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(rename = "mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl BaseInfoUpdateOrganizationUserReq {
    pub fn new(operation_id: String) -> BaseInfoUpdateOrganizationUserReq {
        BaseInfoUpdateOrganizationUserReq {
            birth: None,
            create_time: None,
            email: None,
            english_name: None,
            ex: None,
            face_url: None,
            gender: None,
            mobile: None,
            nickname: None,
            operation_id,
            telephone: None,
            user_id: None,
        }
    }
}


