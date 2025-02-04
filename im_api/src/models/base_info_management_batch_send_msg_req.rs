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
pub struct BaseInfoManagementBatchSendMsgReq {
    #[serde(rename = "businessOperationID", skip_serializing_if = "Option::is_none")]
    pub business_operation_id: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: i32,
    #[serde(rename = "groupID", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "isOnlineOnly", skip_serializing_if = "Option::is_none")]
    pub is_online_only: Option<bool>,
    #[serde(rename = "notOfflinePush", skip_serializing_if = "Option::is_none")]
    pub not_offline_push: Option<bool>,
    #[serde(rename = "offlinePushInfo", skip_serializing_if = "Option::is_none")]
    pub offline_push_info: Option<crate::models::ServerApiParamsOfflinePushInfo>,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "recvIDList", skip_serializing_if = "Option::is_none")]
    pub recv_id_list: Option<Vec<String>>,
    #[serde(rename = "sendID")]
    pub send_id: String,
    #[serde(rename = "senderFaceURL", skip_serializing_if = "Option::is_none")]
    pub sender_face_url: Option<String>,
    #[serde(rename = "senderNickname", skip_serializing_if = "Option::is_none")]
    pub sender_nickname: Option<String>,
    #[serde(rename = "senderPlatformID", skip_serializing_if = "Option::is_none")]
    pub sender_platform_id: Option<i32>,
    #[serde(rename = "sessionType")]
    pub session_type: i32,
}

impl BaseInfoManagementBatchSendMsgReq {
    pub fn new(content_type: i32, operation_id: String, send_id: String, session_type: i32) -> BaseInfoManagementBatchSendMsgReq {
        BaseInfoManagementBatchSendMsgReq {
            business_operation_id: None,
            content_type,
            group_id: None,
            is_online_only: None,
            not_offline_push: None,
            offline_push_info: None,
            operation_id,
            recv_id_list: None,
            send_id,
            sender_face_url: None,
            sender_nickname: None,
            sender_platform_id: None,
            session_type,
        }
    }
}


