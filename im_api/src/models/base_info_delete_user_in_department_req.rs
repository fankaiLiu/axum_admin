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
pub struct BaseInfoDeleteUserInDepartmentReq {
    #[serde(rename = "departmentID")]
    pub department_id: String,
    #[serde(rename = "operationID")]
    pub operation_id: String,
    #[serde(rename = "userID")]
    pub user_id: String,
}

impl BaseInfoDeleteUserInDepartmentReq {
    pub fn new(department_id: String, operation_id: String, user_id: String) -> BaseInfoDeleteUserInDepartmentReq {
        BaseInfoDeleteUserInDepartmentReq {
            department_id,
            operation_id,
            user_id,
        }
    }
}


