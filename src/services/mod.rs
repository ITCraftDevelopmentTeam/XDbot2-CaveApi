mod responses;
pub mod data;
mod query;

use data::DataHelper;
use responses::{
    IndexResponse,
    CaveCount,
    Error,
    CaveItem
};
use actix_web::{HttpResponse, get, Responder, web};



#[get("/")]
pub async fn index(data_helper: web::Data<DataHelper>) -> impl Responder {
    let cave_count: data::CaveCount = match data_helper.get_cave_count() {
        Ok(count) => count,
        Err(err) => {
            return HttpResponse::InternalServerError().json(Error {
                code: 500,
                message: err
            });
        }
    };
    HttpResponse::Ok().json(IndexResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        count: CaveCount {
            total: cave_count.total,
            valid: cave_count.valid
        },
        code: 200
    })
}


#[get("/random")]
pub async fn random(
    data_helper: web::Data<DataHelper>,
    query_params: web::Query<query::Random>
) -> impl Responder {
    let cave = match data_helper.random_cave(
        query_params.max_length.unwrap_or(usize::MAX),
        query_params.no_image.unwrap_or(false)
    ) {
        Ok(data) => data,
        Err(err) => return HttpResponse::InternalServerError().json(Error {
            code: 500,
            message: err
        })
    };
    HttpResponse::Ok().json(CaveItem {
        code: 200,
        id: cave.id,
        content: cave.content.clone(),
        sender: cave.sender.clone(),
        time: cave.time
    })
}



