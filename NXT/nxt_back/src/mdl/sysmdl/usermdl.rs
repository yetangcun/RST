use serde::{Serialize,Deserialize};
use utoipa::{ToSchema, IntoParams};
use utoipa::openapi::{schema::Schema, ObjectBuilder, Object};


#[derive(ToSchema,Deserialize)]
pub struct lginput {
    // #[param(ignore)]
    // #[deprecated = "use account instead of usr"]
    pub usr: String,
    pub pwd: String,
    pub code: String
}