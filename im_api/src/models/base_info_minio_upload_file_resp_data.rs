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
pub struct BaseInfoMinioUploadFileRespData {
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub URL: Option<String>,
    #[serde(rename = "newName", skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "snapshotName", skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "snapshotURL", skip_serializing_if = "Option::is_none")]
    pub snapshot_url: Option<String>,
}

impl BaseInfoMinioUploadFileRespData {
    pub fn new() -> BaseInfoMinioUploadFileRespData {
        BaseInfoMinioUploadFileRespData {
            URL: None,
            new_name: None,
            snapshot_name: None,
            snapshot_url: None,
        }
    }
}


