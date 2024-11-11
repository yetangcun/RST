use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use sqlx::{FromRow};
use utoipa::openapi::{schema::Schema, ObjectBuilder, Object};
use chrono::{NaiveDateTime};

/// 登录入参
#[derive(ToSchema,IntoParams,Deserialize,Serialize)]
pub struct lginput {
    // #[param(ignore)]
    pub usr: String,
    pub pwd: String,
    pub code: String
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