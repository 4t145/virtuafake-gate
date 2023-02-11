use actix_web::{get, web, HttpResponse};
use bilibili_client::reqwest_client::ReqwestClient as BiliClient;

use crate::{AddData};

#[get("/liver/user-info/{uid}")]
pub async fn user_info(data: web::Data<AddData>, uid: web::Path<(u64,)>) -> HttpResponse {
    let cache = data.user_info_cache.clone();
    let uid = uid.0;
    let client = BiliClient::new(Some(data.cookies.clone()));
    let resp = client.get_room_info_cached(uid, &cache).await;
    
    if let Ok(resp) = resp {
        if let Some(userinfo) = &resp.data {
            return HttpResponse::Ok().json(userinfo);
        } else {
            return HttpResponse::InternalServerError().body("无法获取数据");
        }
    } else {
        return HttpResponse::NotFound().body("网络请求错误");
    }
}