use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use clickhouse::Row;
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, Row, PartialEq, Debug)]
pub struct dial_record {
    pub id: u64,
    pub name: String,
    pub age: i32,
    pub intime: NaiveDateTime
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct dial_page_input {
    pub id: u64,
    pub name: String,
    pub age: i32,
    pub intime: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct dial_rcd_input {
    pub id: u64,
    pub name: String,
    pub age: i32
}