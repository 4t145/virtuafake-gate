use futures::TryStreamExt;
use serde::{Deserialize};
use actix_web::{post, web, HttpResponse};
use crate::model::event::Event;
use crate::AddData;
use crate::service::{Pagination, Paged};

#[derive(Debug, Clone, Deserialize)]
pub struct LiveDanmakuRequest {
    pagination: Pagination,
    roomid: u64,
    uid: Option<u64>,
    #[serde(default)]
    no_emoticon: bool,
    #[serde(default)]
    no_draw: bool,
    time_from: u64,
    time_to: u64
}

#[post("/liveroom/danmaku")]
pub async fn danmaku(data: web::Data<AddData>, req: web::Json<LiveDanmakuRequest>) -> HttpResponse {
    let collection_name = req.roomid.to_string();
    let collection = data.db.collection::<Event>(&collection_name);
    let mut filter = bson::doc! {
        "tag": "Danmaku",
        "timestamp": {
            "$exists": true,
            "$gte": req.time_from as i64,
            "$lt": req.time_to as i64,
        },
    };
    if let Some(uid) = req.uid {
        filter.insert("data.user.uid", bson::Bson::Int64(uid as i64));
    }
    if req.no_emoticon {
        filter.insert("data.message.tag", "Plain");
    }
    if req.no_draw {
        filter.insert("data.message.junk_flag", bson::doc! {
            "$ne": 1
        });
    }
    let options = req.pagination.clone().as_mongodb_option();
    let mut collector = Vec::new();
    let total =  collection.count_documents(filter.clone(), None).await.unwrap();
    match collection.find(filter, options).await {
        Ok(mut cursor) => {
            while let Ok(Some(record)) = cursor.try_next().await {
                collector.push(record);
            }
            return HttpResponse::Ok().json(Paged {
                data: collector,
                pagination: Pagination {
                    total,
                    size: req.pagination.size,
                    page: req.pagination.page
                }
            });
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}