use actix_web::{HttpResponse, web::{Data, Path}};
use crate::ActixData;
use anyhow::Result;
use tera::Context;

use crate::services::links::LinksService;
use crate::services::category::CategoryService;
use crate::repositories;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathReq {
    category_id: String
}

pub async fn endpoint_(data: Data<ActixData>, path: Path<PathReq>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    ctx.insert("site_description", &data.conf.site.description);
    ctx.insert("site_allowed_request", &data.conf.site.allow_request); // リンク追加リクエスト

    let pool = repositories::load_db(
        &data.conf.database.dbname,
    ).await?;

    let category = CategoryService::category_from_id(&pool, &path.category_id).await?;

    ctx.insert("category_id", &category.id);
    ctx.insert("category_name", &category.name);
    ctx.insert("category_description", &category.description);

    let links = LinksService::links_get_from_category(&pool, &path.category_id).await?;
    ctx.insert("links_list", &links); 

    let html = data.tera.render("category.html", &ctx)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn endpoint(data: Data<ActixData>, path: Path<PathReq>) -> HttpResponse {
    match endpoint_(data, path).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

