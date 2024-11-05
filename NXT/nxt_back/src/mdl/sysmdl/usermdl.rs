use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use sqlx::{FromRow};
use utoipa::openapi::{schema::Schema, ObjectBuilder, Object};
use chrono::{NaiveDateTime};


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

#[derive(
    Deserialize,
    Serialize,
    FromRow
  )
]
pub struct usrs {
    pub Id: String,
    pub Account: String,
    pub Status: i32,
    pub CreateTime: NaiveDateTime
}