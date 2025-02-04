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
pub struct OpenImInternalApiManageRevokeElem {
    #[serde(rename = "revokeMsgClientID")]
    pub revoke_msg_client_id: String,
}

impl OpenImInternalApiManageRevokeElem {
    pub fn new(revoke_msg_client_id: String) -> OpenImInternalApiManageRevokeElem {
        OpenImInternalApiManageRevokeElem {
            revoke_msg_client_id,
        }
    }
}


