mod responses;
pub mod data;

use data::DataHelper;
use responses::{
    IndexResponse,
    CaveCount
};
use actix_web::{HttpResponse, get, Responder, web};

#[get("/")]
pub async fn index(data_helper: web::Data<DataHelper>) -> impl Responder {
    let cave_count = match data_helper.get_cave_count() {
        Ok(count) => count,
        Err(err) => {
            return HttpResponse::InternalServerError().body(err);
        }
    };
    HttpResponse::Ok().json(IndexResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        count: CaveCount {
            total: cave_count.total,
            valid: cave_count.valid as u64
        }
    })
}




