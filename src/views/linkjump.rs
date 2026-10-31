use actix_web::{HttpResponse, http::header, web::{Data, Path}};
use crate::{ActixData, services::category::CategoryService};
use anyhow::Result;

use crate::services::link::LinkService;
use crate::repositories;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathReq {
    link_id: String
}


pub async fn endpoint_(data: Data<ActixData>, path: Path<PathReq>) -> Result<HttpResponse> {

    let pool = repositories::load_db(
        &data.conf.database.dbname,
    ).await?;

    let mut repos = LinkService::read_from_id(&pool, &path.link_id).await?;

    LinkService::increment(&mut repos, &pool).await?;

    Ok(HttpResponse::build(actix_web::http::StatusCode::FOUND)
        .insert_header((header::LOCATION, repos.url))
        .finish())
}

pub async fn endpoint(data: Data<ActixData>, path: Path<PathReq>) -> HttpResponse {
    match endpoint_(data, path).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

