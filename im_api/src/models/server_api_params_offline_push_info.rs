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
pub struct ServerApiParamsOfflinePushInfo {
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
    #[serde(rename = "iOSBadgeCount", skip_serializing_if = "Option::is_none")]
    pub i_os_badge_count: Option<bool>,
    #[serde(rename = "iOSPushSound", skip_serializing_if = "Option::is_none")]
    pub i_os_push_sound: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ServerApiParamsOfflinePushInfo {
    pub fn new() -> ServerApiParamsOfflinePushInfo {
        ServerApiParamsOfflinePushInfo {
            desc: None,
            ex: None,
            i_os_badge_count: None,
            i_os_push_sound: None,
            title: None,
        }
    }
}


