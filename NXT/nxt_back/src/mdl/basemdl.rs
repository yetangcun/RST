use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

// 统一响应结果结构
#[derive(Serialize)]
pub struct resmdl<T> where T: Serialize {
    pub code: String,
    pub msg: String,
    pub is_succ: bool,
    pub data: T
}
impl<T> resmdl<T> where T: Serialize {
    pub fn succ(code: String, msg: String, data: T) -> resmdl<T> {
        resmdl {
            code,
            msg,
            is_succ: true,
            data
        }
    }

    pub fn fail(code: String, msg: String, data: T) -> resmdl<T> {
        resmdl {
            code,
            msg,
            is_succ: false,
            data
        }
    }
}

// 分页查询入参
#[derive(ToSchema, Deserialize)]
pub struct req_pg<T> where T: Serialize  {
    pub page: i32,
    pub size: i32,
    pub params: T
 }
 
 // 分页查询结果
 #[derive(ToSchema, Serialize)]
 pub struct res_pg<T> where T: Serialize  {
     pub is_succ: bool,
     pub msg: String,
     pub code: String,
     pub total: i32,
     pub page: i32,
     pub data: Vec<T>
 }
 impl<T> res_pg<T> where T: Serialize  {
     pub fn succ(total: i32, page: i32, data: Vec<T>) -> res_pg<T> {
         res_pg {
             is_succ: true,
             msg: "".to_string(),
             code: "200".to_string(),
             total,
             page,
             data
         }
     }

     pub fn fail(msg: String, code: String) -> res_pg<T> {
         res_pg {
             is_succ: false,
             msg,
             code: code,
             total: 0,
             page: 0,
             data: vec![]
         }
     }
 }