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
pub struct ServerApiParamsGroupMemberFullInfo {
    #[serde(rename = "appMangerLevel", skip_serializing_if = "Option::is_none")]
    pub app_manger_level: Option<i32>,
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
    #[serde(rename = "faceURL", skip_serializing_if = "Option::is_none")]
    pub face_url: Option<String>,
    #[serde(rename = "groupID", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "inviterUserID", skip_serializing_if = "Option::is_none")]
    pub inviter_user_id: Option<String>,
    #[serde(rename = "joinSource", skip_serializing_if = "Option::is_none")]
    pub join_source: Option<i32>,
    #[serde(rename = "joinTime", skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i32>,
    #[serde(rename = "muteEndTime", skip_serializing_if = "Option::is_none")]
    pub mute_end_time: Option<i32>,
    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "operatorUserID", skip_serializing_if = "Option::is_none")]
    pub operator_user_id: Option<String>,
    #[serde(rename = "roleLevel", skip_serializing_if = "Option::is_none")]
    pub role_level: Option<i32>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ServerApiParamsGroupMemberFullInfo {
    pub fn new() -> ServerApiParamsGroupMemberFullInfo {
        ServerApiParamsGroupMemberFullInfo {
            app_manger_level: None,
            ex: None,
            face_url: None,
            group_id: None,
            inviter_user_id: None,
            join_source: None,
            join_time: None,
            mute_end_time: None,
            nickname: None,
            operator_user_id: None,
            role_level: None,
            user_id: None,
        }
    }
}


