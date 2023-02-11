use futures::TryStreamExt;
use serde::{Deserialize};
use actix_web::{post, web, HttpResponse};
use crate::model::event::Event;
use crate::AddData;
use crate::service::{Pagination, Paged};

#[derive(Debug, Clone, Deserialize)]
pub struct LiveEnterroomRequest {
    pagination: Pagination,
    roomid: u64,
    uid: Option<u64>,
    time_from: u64,
    time_to: u64
}

#[post("/liveroom/enterroom")]
pub async fn enterroom(data: web::Data<AddData>, mut req: web::Json<LiveEnterroomRequest>) -> HttpResponse {
    let collection_name = req.roomid.to_string();
    let collection = data.db.collection::<Event>(&collection_name);
    let mut filter = bson::doc! {
        "tag": "EnterRoom",
        "timestamp": {
            "$exists": true,
            "$gte": req.time_from as i64,
            "$lt": req.time_to as i64,
        },
        
    };
    if let Some(uid) = req.uid {
        filter.insert("data.user.uid", bson::Bson::Int64(uid as i64));
    }
    let total =  collection.count_documents(filter.clone(), None).await.unwrap();
    req.pagination.total = total;
    let pagination = &req.pagination;
    let options = pagination.clone().as_mongodb_option(true);
    let mut collector = Vec::new();
    match collection.find(filter, options).await {
        Ok(mut cursor) => {
            while let Ok(Some(record)) = cursor.try_next().await {
                collector.push(record);
            }
            return HttpResponse::Ok().json(Paged {
                data: collector,
                pagination: pagination.clone(),
            });
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}