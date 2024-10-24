pub mod rsapi;
pub mod ext;
pub mod bll;
pub mod mdl;

use utoipa_swagger_ui::{SwaggerUi, Url};
use utoipa::{OpenApi, openapi::OpenApiBuilder};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use ext::{swag_ui::*, auth_ext::*};


#[actix_web::main]
async fn main() {
    println!("Hello, world!");
}
