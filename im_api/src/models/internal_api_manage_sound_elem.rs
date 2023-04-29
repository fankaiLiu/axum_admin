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
pub struct InternalApiManageSoundElem {
    #[serde(rename = "dataSize", skip_serializing_if = "Option::is_none")]
    pub data_size: Option<i32>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "soundPath", skip_serializing_if = "Option::is_none")]
    pub sound_path: Option<String>,
    #[serde(rename = "sourceURL", skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl InternalApiManageSoundElem {
    pub fn new() -> InternalApiManageSoundElem {
        InternalApiManageSoundElem {
            data_size: None,
            duration: None,
            sound_path: None,
            source_url: None,
            uuid: None,
        }
    }
}


