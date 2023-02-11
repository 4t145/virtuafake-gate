use std::sync::{RwLock, Arc};
use config::EnvConfig;
use http_api_util::{cache::*};
use actix_web::{web, App, HttpServer};
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
    feedlist: config::feedlist::Feedlist,
    db: mongodb::Database
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // loading mongodb
    let db = {
        use mongodb::options::{ClientOptions, ServerAddress};
        let dbconfig = config::mongodb::Config::load();
        let host = dbconfig.host;
        let port = Some(dbconfig.port);
        let db = dbconfig.db;
        let options = ClientOptions::builder().hosts(vec![ServerAddress::Tcp{host, port}]).build();
        let db = mongodb::Client::with_options(options).map(|client| {
            client.database(db.as_str())
        }).unwrap();
        db
    };

    let app_data = web::Data::new(AddData {
        user_info_cache: Arc::new(RwLock::new(FifoCache::new(128))),
        feedlist: config::feedlist::Feedlist::load_from_env(),
        db
    });

    HttpServer::new(move || {
        let cors = Cors::permissive()
        .allowed_origin("http://localhost:5173")
        .allowed_origin("http://127.0.0.1")
        .allowed_origin("http://vrp.4t145.com")
        .allowed_origin("https://vrp.4t145.com")
        // .expose_headers(["access-control-allow-origin", "access-control-allow-credentials"])
        .allow_any_method()
        .allow_any_origin()
        .supports_credentials()
        .max_age(3500);

        App::new().app_data(app_data.clone())
        // .wrap(
        //     middleware::DefaultHeaders::new()
        //     .add(("Content-Type", "text/html; charset=utf-8"))
        // )
        .wrap(cors)
        .service(service::index_page)
        .service(service::liveroom::info)
        .service(service::liveroom::danmaku)
        .service(service::liveroom::danmaku_heat)
        .service(service::liveroom::watched)
        .service(service::liveroom::superchat)
        .service(service::liveroom::gift)
        .service(service::liveroom::enterroom)
        .service(service::liver::feedlist_liver)
        .service(service::liver::streaming_list)
        .service(service::liver::feedlist_liver)
        .service(service::liver::user_info)
    })
    .bind(("0.0.0.0", env!("PORT").parse::<u16>().unwrap_or(80)))?
    .run()
    .await
}