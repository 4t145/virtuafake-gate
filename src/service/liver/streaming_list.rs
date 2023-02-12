use std::{time::Duration, sync::Arc};

use actix_web::{get, web, HttpResponse};
use bilibili_client::{reqwest_client::{ReqwestClient as BiliClient}, api::{user::info::UserInfoRequest, CommonResp}};
use expire::MaybeExpired;
use http_api_util::cache::ApiCache;

use futures::future::join_all;
use serde::Serialize;

use crate::{model::liver::Liver, AddData};

#[derive(Debug, Clone, Serialize, Default)]
pub struct StreamingListResponse<'a>(Vec<&'a Liver>);

#[get("/liver/streaming-list")]
pub async fn streaming_list(data: web::Data<AddData>) -> HttpResponse {
    // let cache = data.user_info_cache.clone();
    // let feedlist = &data.feedlist;
    // let mut response_list = Vec::new();
    // let mut request_list = Vec::new();
    // {
    //     let cache = cache.read().unwrap();
    //     for liver in &feedlist.liver {
    //         if let Some(resp) = cache.get(&UserInfoRequest { mid: liver.uid }) {
    //             if let Some(resp) = resp.get().map(|x|x.as_ref().clone()) {
    //                 if let Some(resp_data) = resp.data {
    //                     response_list.push(resp_data)
    //                 }
    //             }
    //         } else {
    //             request_list.push(liver.uid);
    //         };
    //     }
    // }
    // let client = BiliClient::new(Some(data.cookies.clone()));
    // for chunk in request_list.chunks(50) {
    //     let resp = client.get_user_info_list(chunk.to_vec()).await;
    //     if let Ok(resp) = resp {
    //         if let Some(resp_data) = resp.data {
    //             let mut cache = cache.write().unwrap();
    //             for resp_data_item in resp_data {
    //                 let mid = resp_data_item.mid as u64;
    //                 if resp_data_item.mid != 0 {
    //                     let mut cache_data_item = MaybeExpired::new();
    //                     response_list.push(resp_data_item.clone());
    //                     cache_data_item.set(Arc::new(CommonResp {
    //                         code: 0,
    //                         data: Some(resp_data_item),
    //                         message: None
    //                     }), Duration::from_secs(60));
    //                     cache.put(UserInfoRequest { mid }, cache_data_item);
    //                 }
    //             }
    //         }
    //     }
    // }
    return HttpResponse::Ok().json(Vec::<i32>::new());
}
