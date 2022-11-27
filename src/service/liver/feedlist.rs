use serde::Serialize;
use actix_web::{get, web, HttpResponse};

use crate::{AddData, model::liver::Liver};

#[derive(Debug, Clone, Serialize, Default)]
pub struct StreamingListResponse<'a>(Vec<&'a Liver>);

#[get("/liver/feedlist/liver")]
pub async fn feedlist_liver(data: web::Data<AddData>) -> HttpResponse {
    let list = &data.feedlist.liver;
    return HttpResponse::Ok().json(list)
}