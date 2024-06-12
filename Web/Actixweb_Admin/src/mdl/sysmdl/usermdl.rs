use serde::{Serialize,Deserialize};
// use utoipa::ToSchema;
// use utoipa::ToSchema;

// #[derive(Schema)]
// #[derive(Serialize, Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct lginput { // 登录入参
    pub account:String,
    pub passwd:String
}

#[derive(Serialize, Deserialize)]
pub struct userQueryInput { // 查询入参
    pub name:String,
    pub uid:i32,
    pub orgid:String
}

#[derive(Serialize, Deserialize)]
pub struct userOptInput { // 操作入参
    pub name:String,
    pub employee_no:String,
    pub role_id:String,
    pub org_id:String,
    pub addr:String,
    pub age:i32,
    pub phone:String,
    pub add_user:String,
    pub add_time:String,
    pub up_user:String,
    pub up_time:String,
    pub state:i32
}

#[derive(Serialize)]
pub struct userQueryOutput { // 查询出参
    pub name:String,
    pub employee_no:String,
    pub role_name:String,
    pub org_name:String,
    pub addr:String,
    pub age:i32,
    pub phone:String,
    pub add_user:String,
    pub add_time:String,
    pub up_user:String,
    pub up_time:String,
    pub state:i32
}