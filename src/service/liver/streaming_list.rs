use futures::future::join_all;
use serde::Serialize;
use actix_web::{get, web, HttpResponse};
use bilibili_client::reqwest_client::ReqwestClient as BiliClient;

use crate::{AddData, model::liver::Liver};

#[derive(Debug, Clone, Serialize, Default)]
pub struct StreamingListResponse<'a>(Vec<&'a Liver>);

#[get("/liver/streaming-list")]
pub async fn streaming_list(data: web::Data<AddData>) -> HttpResponse {
    let cache = data.user_info_cache.clone();
    let feedlist = &data.feedlist;
    let mut response_list = Vec::new();
    let mut h_set = Vec::new();
    for idx in 0..feedlist.liver.len() {
        use actix_web::rt::*;
        let uid = feedlist.liver[idx].uid;
        let cache = cache.clone();
        let h = spawn(async move {
            let client = BiliClient::new();
            let resp = client.get_room_info_cached(uid, &cache).await;
            let online = {
                if let Ok(resp) = resp {
                    if let Some(userinfo) = &resp.data {
                        userinfo.live_room.live_status == 1
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if online {
                Some(idx)
            } else {
                None
            }
        });
        h_set.push(h);
    }
    for idx in join_all(h_set).await {
        if let Ok(Some(idx)) = idx {
            response_list.push(&feedlist.liver[idx])
        }
    }
    return HttpResponse::Ok().json(response_list)
}