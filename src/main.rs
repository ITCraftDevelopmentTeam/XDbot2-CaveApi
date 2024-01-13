mod config;
mod services;

// use log::info;
use env_logger;
use dotenv::dotenv;
use config::{Config, get_config};
use actix_web::{HttpServer, App, web};
use log::info;
use services::data::DataHelper;
use std::io::Result;
use std::path::PathBuf;


#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let conf: Config = get_config();
    info!("Starting server at {}:{}", conf.host, conf.port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(DataHelper {
                base_path: PathBuf::from(&conf.source)
            }))
            .service(services::index)
    })
    .bind((conf.host.clone(), conf.port))?
    .run()
    .await
}
