use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use chrono::{DateTime, TimeZone, Local, Utc, NaiveDateTime };
// use time::OffsetDateTime;
use clickhouse::{Row};

// use std::any::Any;
// use chrono::{NaiveDateTime,Local,DateTime,Utc, TimeZone, OffsetDateTime};

#[derive(Serialize, Deserialize, Row, PartialEq, Debug)]
pub struct dial_record {
    pub id: u64,
    pub name: String,
    pub age: i32,
    // pub intime: DateTime<Local>,    // DateTime<Local> DateTime<Utc> NaiveDateTime DateTime64(9)
    // pub intime: String
    // pub intime: NaiveDateTime
}

#[derive(Serialize, Deserialize, Row, Debug)]
pub struct dial_record_out {
    pub id: u64,
    pub name: String,
    pub age: i32,
    pub intime: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct dial_page_input {
    #[schema(default = 0, example = 0)]  // 默认值和示例值 需要添加ToSchema
    pub id: u64,
    pub name: String,
    #[schema(default = 0, example = 0)]
    pub age: i32,
    pub intime: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct dial_rcd_input {
    #[schema(default = 0, example = 0)]
    pub id: u64,
    pub name: String,
    #[schema(default = 0, example = 0)]
    pub age: i32
}