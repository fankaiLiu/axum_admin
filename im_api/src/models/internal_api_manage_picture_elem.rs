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
pub struct InternalApiManagePictureElem {
    #[serde(rename = "bigPicture", skip_serializing_if = "Option::is_none")]
    pub big_picture: Option<crate::models::InternalApiManagePictureBaseInfo>,
    #[serde(rename = "snapshotPicture", skip_serializing_if = "Option::is_none")]
    pub snapshot_picture: Option<crate::models::InternalApiManagePictureBaseInfo>,
    #[serde(rename = "sourcePath", skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(rename = "sourcePicture", skip_serializing_if = "Option::is_none")]
    pub source_picture: Option<crate::models::InternalApiManagePictureBaseInfo>,
}

impl InternalApiManagePictureElem {
    pub fn new() -> InternalApiManagePictureElem {
        InternalApiManagePictureElem {
            big_picture: None,
            snapshot_picture: None,
            source_path: None,
            source_picture: None,
        }
    }
}


