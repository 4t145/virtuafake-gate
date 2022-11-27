use serde::{Deserialize};
use actix_web::{post, web, HttpResponse};
use futures::TryStreamExt;
use crate::model::event::Event;
use crate::AddData;
use crate::model::stat::Heat;

#[derive(Debug, Clone, Deserialize)]
pub struct LiveDanmakuHeatRequest {
    roomid: u64,
    time_from: u64,
    time_to: u64
}

#[post("/liveroom/danmaku-heat")]
pub async fn danmaku_heat(data: web::Data<AddData>, req: web::Json<LiveDanmakuHeatRequest>) -> HttpResponse {
    let now_min = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()/60;
    const MINUTE: u64 = 60_000;
    let collection_name = req.roomid.to_string();
    let collection = data.db.collection::<Event>(&collection_name);
    let heat_collection_name = format!("{collection_name}_stat_heat");
    let danmaku_heat_collection = data.db.collection::<Heat>(&heat_collection_name);
    let minute_start = req.time_from/MINUTE;
    let minute_end = (req.time_to/MINUTE).min(now_min-1);
    let minute_count = (minute_end - minute_start) as usize;
    if minute_count > 360 {
        return HttpResponse::Forbidden().body("分钟数超越最大限制360min")
    }
    let mut collector = vec![None;minute_count];
    
    let filter = bson::doc! {
        "minute": {
            "$gte": minute_start as i64,
            "$lt": minute_end as i64
        },
    };

    let mut cursor = danmaku_heat_collection.find(filter, None).await.unwrap();

    while let Ok(Some(record)) = cursor.try_next().await {
        let index = (record.minute - minute_start as u32) as usize;
        collector[index] = Some(record.danmaku_count)
    }

    let mut filter = bson::doc! {
        "tag": "Danmaku",
    };

    let mut result = vec![0;minute_count];
    let mut update = Vec::<Heat>::new();

    for index in 0..collector.len() {
        let record = collector[index];
        let danmaku_count = match record {
            Some(danmaku_conut) => {
                danmaku_conut
            }
            None => {
                let minute = index as i64 + minute_start as i64;
                let ts_start = minute * (MINUTE as i64);
                filter.insert("timestamp", bson::doc! {
                    "$exists": true,
                    "$gte": ts_start,
                    "$lt": (ts_start + MINUTE as i64),
                });
                let danmaku_count = collection.count_documents(filter.clone(), None).await.unwrap();
                update.push(Heat {
                    minute: minute as u32,
                    danmaku_count: danmaku_count as u32
                });
                danmaku_count as u32
            }
        };
        result[index] = danmaku_count;
    }

    actix_web::rt::spawn(async move {
        if !update.is_empty() {
            danmaku_heat_collection.insert_many(update, None).await.unwrap();
        }
    });
    
    let resp_body = bincode::serialize(&result).unwrap();
    HttpResponse::Ok().body(resp_body)
}