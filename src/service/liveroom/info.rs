use serde::{Deserialize, Serialize};
use actix_web::{get, web, HttpResponse};
use bilibili_client::reqwest_client::ReqwestClient as BiliClient;

use crate::AddData;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct LiveRoomRequest {
    uid: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Default)]
pub struct LiveRoomResponse<'a> {
    title: &'a str,
    watched: u32,
    cover: &'a str,
    is_streaming: bool
}


#[get("/liveroom/info")]
pub async fn info(data: web::Data<AddData>, req: web::Query<LiveRoomRequest>) -> HttpResponse {
    let cache = &data.user_info_cache;
    let client = BiliClient::new();
    let resp = client.get_room_info_cached(req.uid, cache).await;
    if let Ok(resp) = resp {
        if let Some(userinfo) = &resp.data {
            let resp = LiveRoomResponse {
                title: &userinfo.live_room.title,
                watched: userinfo.live_room.watched_show.num as u32,
                cover: &userinfo.live_room.cover,
                is_streaming: userinfo.live_room.live_status == 1
            };
            return HttpResponse::Ok().json(resp) // <- send response
        }
        let msg = resp.message.clone().unwrap_or_default();
        return HttpResponse::NotFound().body(msg)
    } else {
        let err = resp.err().unwrap();
        HttpResponse::NotFound().body(format!("{err:?}"))
    }
}