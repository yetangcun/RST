pub mod usermdl;
pub mod rolemdl;
pub mod menumdl;
pub mod orgmdl;
pub mod syscfgmdl;
pub mod permissionmdl;
use serde::{Serialize,Deserialize};


#[derive(Serialize)]
pub struct rsmdl {
    pub msg:String,
    pub succ:bool
}