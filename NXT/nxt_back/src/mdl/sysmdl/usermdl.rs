use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use sqlx::{FromRow};
use utoipa::openapi::{schema::Schema, ObjectBuilder, Object};
use chrono::{NaiveDateTime};

/// 登录入参
#[derive(ToSchema,IntoParams,Deserialize,Serialize)]
pub struct lginput {
    /// 用户名
    // #[param(ignore)]
    pub usr: String,

    /// 密码
    pub pwd: String,

    /// 验证码
    pub code: String
}

#[derive(ToSchema,Serialize,FromRow)]
pub struct lg_res {
    /// 用户名
    pub Id: String,

    /// 密码
    pub Passwd: String,
}

#[derive(ToSchema,Serialize)]
pub struct lgoutput {
    /// 用户名
    pub tken: String,

    /// 密码
    pub uid: String,
}

/// 用户分页查询入参
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
pub struct permissions_input {
    pub id: String
}

#[derive(
    Deserialize,
    Serialize,
    FromRow
  )
]
pub struct usrs {
    /// 用户Id
    pub Id: String,
    /// 账号 唯一
    pub Account: String,
    /// 状态
    pub Status: i32,
    /// 创建时间
    pub CreateTime: NaiveDateTime
}