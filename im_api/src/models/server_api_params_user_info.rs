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
pub struct ServerApiParamsUserInfo {
    #[serde(rename = "appMangerLevel", skip_serializing_if = "Option::is_none")]
    pub app_manger_level: Option<i32>,
    #[serde(rename = "birth", skip_serializing_if = "Option::is_none")]
    pub birth: Option<i32>,
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
    #[serde(rename = "faceURL", skip_serializing_if = "Option::is_none")]
    pub face_url: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(rename = "globalRecvMsgOpt", skip_serializing_if = "Option::is_none")]
    pub global_recv_msg_opt: Option<i32>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "phoneNumber", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ServerApiParamsUserInfo {
    pub fn new() -> ServerApiParamsUserInfo {
        ServerApiParamsUserInfo {
            app_manger_level: None,
            birth: None,
            create_time: None,
            email: None,
            ex: None,
            face_url: None,
            gender: None,
            global_recv_msg_opt: None,
            nickname: None,
            phone_number: None,
            user_id: None,
        }
    }
}


