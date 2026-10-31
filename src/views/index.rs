use actix_web::{HttpResponse, web::Data};
use crate::ActixData;
use anyhow::Result;
use tera::Context;

use crate::services::link::LinkService;
use crate::services::category::CategoryService;

use crate::repositories;

pub async fn endpoint_(data: Data<ActixData>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    ctx.insert("site_description", &data.conf.site.description);
    ctx.insert("site_allowed_request", &data.conf.site.allow_request); // リンク追加リクエスト

    let pool = repositories::load_db(
        &data.conf.database.dbname,
    ).await?;

    let links = LinkService::link_from_limit(&pool, 10).await?;
    let categories = CategoryService::categories_all(&pool).await?;

    ctx.insert("links_headline", &links);
    ctx.insert("categories", &categories);


    let html = data.tera.render("index.html", &ctx)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn endpoint(data: Data<ActixData>) -> HttpResponse {
    match endpoint_(data).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

