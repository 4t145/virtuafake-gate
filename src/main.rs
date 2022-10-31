use std::sync::RwLock;
use http_api_util::{cache::*, Api};
use actix_web::{web, App, middleware, HttpServer};
use expire::MaybeExpired;
use bilibili_client::api::user::info::UserInfo;
mod service;
mod api;

type LockedCache<A> = RwLock<FifoCache<<A as Api>::Request, MaybeExpired<<A as Api>::Response>>> ;
pub struct AddData {
    user_info_cache: LockedCache<UserInfo>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AddData {
        user_info_cache: RwLock::new(FifoCache::new(128))
    });
    HttpServer::new(move || {
        App::new().app_data(app_data.clone())
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "text/html; charset=utf-8")))
            .service(service::index_page)
            .service(service::proxy::liveroom_info)
    })
    .bind(("0.0.0.0", env!("PORT").parse::<u16>().unwrap_or(80)))?
    .run()
    .await
}