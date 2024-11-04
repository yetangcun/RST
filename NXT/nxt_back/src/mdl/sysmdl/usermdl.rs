use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use utoipa::openapi::{schema::Schema, ObjectBuilder, Object};


#[derive(ToSchema,IntoParams,Deserialize,Serialize)]
pub struct lginput {
    // #[param(ignore)]
    pub usr: String,
    pub pwd: String,
    pub code: String
}

#[derive(ToSchema,Serialize,Deserialize)]
pub struct usr_page_input {
    pub name: String,
    pub employee_no: String,
    pub org_id: String,
    pub phone: String
}

#[derive(ToSchema,Deserialize,Serialize)]
pub struct usr_permissions {
    pub tk: String
}

#[derive(ToSchema,Deserialize,Serialize)]
pub struct usrs {
    pub uid: i32,
    pub name: String,
    pub employee_no: String,
    pub org_id: String,
    pub phone: String,
    pub email: String,
    pub address: String,
    pub status: i32,
    pub in_time: String, // 创建时间
    pub up_time: String  // 更新时间
}