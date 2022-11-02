use serde::Serialize;
use actix_web::{get, web, HttpResponse};
use bilibili_client::reqwest_client::ReqwestClient as BiliClient;

use crate::{AddData, model::liver::Liver};

#[derive(Debug, Clone, Serialize, Default)]
pub struct StreamingListResponse<'a>(Vec<&'a Liver>);

#[get("/liver/streaming-list")]
pub async fn streaming_list(data: web::Data<AddData>) -> HttpResponse {
    let time_in = std::time::Instant::now();
    let cache = &data.user_info_cache;
    let feedlist = &data.feedlist;
    let client = BiliClient::new();
    let mut response_list = Vec::new();
    for liver in &feedlist.liver {
        let resp = client.get_room_info_cached(liver.uid, cache).await;
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
            response_list.push(liver)
        }
    }
    let time_out = std::time::Instant::now();
    println!("用时{}s", (time_out-time_in).as_micros());
    return HttpResponse::Ok().json(response_list)
}