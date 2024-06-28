use serde::{Serialize,Deserialize};
use DataExtensionLib::mysqlLib::{FrmRow};
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

#[derive(Serialize, Deserialize)]
pub struct userOptSimplInput { // 操作入参
    pub id:i32,
    pub account:String,
    pub passwd:String,
    pub status:i32,
    pub isdeleted:i32,
    pub createtime:String,
    pub createuserid:i32
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

#[derive(Serialize)]
pub struct userQuerySimple {
    pub id:i32,
    pub account:String,
    pub passwd:String
}

impl FrmRow for userQuerySimple {
    fn from_row(row: mysql::Row) -> Result<Self, mysql::Error> {
        Ok(userQuerySimple {
            id: row.get(0).unwrap(),
            account: row.get(1).unwrap(),
            passwd: row.get(2).unwrap(),
        })
    }
}

