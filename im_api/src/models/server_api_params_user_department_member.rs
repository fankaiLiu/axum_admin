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
pub struct ServerApiParamsUserDepartmentMember {
    #[serde(rename = "departmentMember", skip_serializing_if = "Option::is_none")]
    pub department_member: Option<crate::models::ServerApiParamsDepartmentMember>,
    #[serde(rename = "organizationUser", skip_serializing_if = "Option::is_none")]
    pub organization_user: Option<crate::models::ServerApiParamsOrganizationUser>,
}

impl ServerApiParamsUserDepartmentMember {
    pub fn new() -> ServerApiParamsUserDepartmentMember {
        ServerApiParamsUserDepartmentMember {
            department_member: None,
            organization_user: None,
        }
    }
}


