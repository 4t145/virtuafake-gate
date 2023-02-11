use futures::TryStreamExt;
use serde::{Deserialize};
use actix_web::{post, web, HttpResponse};
use crate::model::event::Event;
use crate::AddData;

#[derive(Debug, Clone, Deserialize)]
pub struct LiveWatchedRequest {
    roomid: u64,
    time_from: u64,
    time_to: u64
}

#[post("/liveroom/watched")]
pub async fn watched(data: web::Data<AddData>, req: web::Json<LiveWatchedRequest>) -> HttpResponse {
    let collection_name = req.roomid.to_string();
    let collection = data.db.collection::<Event>(&collection_name);
    let filter = bson::doc! {
        "tag": "WatchedUpdate",
        "timestamp": {
            "$exists": true,
            "$gte": req.time_from as i64,
            "$lt": req.time_to as i64,
        },
    };
    let mut collector = Vec::new();
    match collection.find(filter, None).await {
        Ok(mut cursor) => {
            while let Ok(Some(record)) = cursor.try_next().await {
                match record.event.data {
                    bilive_danmaku::event::EventData::WatchedUpdate { num } => {
                        collector.push(num as u32)
                    }
                    _ => {}
                }
            }
            let body = bincode::serialize(&collector).unwrap();
            return HttpResponse::Ok().body(body);
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}