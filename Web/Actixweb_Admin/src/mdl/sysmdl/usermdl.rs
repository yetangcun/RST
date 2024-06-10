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

pub struct userQueryInput { // 查询入参
    pub name:String,
    pub uid:i32,
    pub orgid:String
}
pub struct userOptInput { // 操作入参
    pub name:String,
    pub employeeNo:String,
    pub roleId:String,
    pub orgId:String,
    pub addr:String,
    pub age:i32,
    pub phone:String,
    pub addUser:String,
    pub addTime:String,
    pub upUser:String,
    pub upTime:String,
    pub state:i32
}

pub struct userQueryOutput { // 查询出参
    pub name:String,
    pub employeeNo:String,
    pub roleName:String,
    pub orgName:String,
    pub addr:String,
    pub age:i32,
    pub phone:String,
    pub addUser:String,
    pub addTime:String,
    pub upUser:String,
    pub upTime:String,
    pub state:i32
}