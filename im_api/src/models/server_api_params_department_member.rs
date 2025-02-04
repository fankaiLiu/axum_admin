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
pub struct ServerApiParamsDepartmentMember {
    #[serde(rename = "departmentID", skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(rename = "ex", skip_serializing_if = "Option::is_none")]
    pub ex: Option<String>,
    #[serde(rename = "leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<i32>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ServerApiParamsDepartmentMember {
    pub fn new() -> ServerApiParamsDepartmentMember {
        ServerApiParamsDepartmentMember {
            department_id: None,
            ex: None,
            leader: None,
            order: None,
            position: None,
            status: None,
            user_id: None,
        }
    }
}


