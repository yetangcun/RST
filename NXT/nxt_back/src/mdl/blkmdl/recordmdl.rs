use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use clickhouse::Row;
use chrono::{NaiveDateTime,Local,DateTime,Utc, TimeZone};

#[derive(Serialize, Deserialize, Row, PartialEq, Debug)]
pub struct dial_record {
    pub id: u64,
    pub name: String,
    pub age: i32,
    pub intime: DateTime<Utc>   // DateTime<Local> DateTime<Utc>
    // pub intime: String
    // pub intime: NaiveDateTime
}

// 为dial_record实现Default特征
// impl Default for dial_record {
//     fn default() -> Self {
//         dial_record {
//             id: 0,
//             name: String::new(),
//             age: 0,
//             intime: Local::now().naive_local(), // 默认值为None
//         }
//     }
// }

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