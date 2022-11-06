use std::sync::{RwLock, Arc};
use http_api_util::{cache::*};
use actix_web::{web, App, middleware, HttpServer};
use bilibili_client::api::user::info::UserInfo;
use bilibili_client::reqwest_client::FifoRwlCache;
use actix_cors::Cors;

mod service;
mod api;
mod model;
mod config;
// type LockedCache<A> = RwLock<FifoCache<<A as Api>::Request, MaybeExpired<<A as Api>::Response>>> ;
pub struct AddData {
    user_info_cache: Arc<FifoRwlCache<UserInfo>>,
    feedlist: config::feedlist::Feedlist
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let app_data = web::Data::new(AddData {
        user_info_cache: Arc::new(RwLock::new(FifoCache::new(128))),
        feedlist: config::feedlist::Feedlist::load_from_env()
    });

    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("http://localhost")
              .allowed_origin("http://127.0.0.1")
              .allowed_origin("http://vrp.4t145.com")
              .allowed_origin("https://vrp.4t145.com")
              .allow_any_method()
              .max_age(3500);

        App::new().app_data(app_data.clone())
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "text/html; charset=utf-8")))
        .wrap(cors)
            .service(service::index_page)
            .service(service::liveroom::info)
            .service(service::liver::streaming_list)
    })
    .bind(("0.0.0.0", env!("PORT").parse::<u16>().unwrap_or(80)))?
    .run()
    .await
}