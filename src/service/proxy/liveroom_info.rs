use serde::{Deserialize, Serialize};
use actix_web::{get, web, Result as AwResult, HttpResponse};
use bilibili_client::reqwest_client::ReqwestClient as BiliClient;

use crate::AddData;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct LiveRoomRequest {
    uid: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Default)]
pub struct LiveRoomResponse {
    title: String,
    watched: u32,
    cover: String,
    is_streaming: bool
}

#[get("/proxy/liveroom/info")]
pub async fn liveroom_info(data: web::Data<AddData>, req: web::Json<LiveRoomRequest>) -> HttpResponse {
    let cache = &data.user_info_cache;
    let client = BiliClient::new();
    let resp = client.get_room_info_cached(req.uid, cache).await;
    if let Ok(resp) = resp {
        if let Some(userinfo) = resp.data {
            let resp = LiveRoomResponse {
                title: userinfo.live_room.title,
                watched: userinfo.live_room.watched_show.num as u32,
                cover: userinfo.live_room.cover,
                is_streaming: userinfo.live_room.live_status == 1
            };
            return HttpResponse::Ok().json(resp) // <- send response
        }
        return HttpResponse::NotFound().body(resp.message.unwrap_or_default())
    } else {
        let err = resp.err().unwrap();
        HttpResponse::NotFound().body(format!("{err:?}"))
    }
}