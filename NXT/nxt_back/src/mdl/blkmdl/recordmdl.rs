use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use clickhouse::Row;
// use time::{Date, Month, OffsetDateTime, Time};
use chrono::{DateTime, TimeZone, Local, Utc };
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