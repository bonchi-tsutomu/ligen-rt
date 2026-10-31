use actix_web::{HttpResponse, web::{Data, Path}};
use crate::{ActixData, services::category::CategoryService};
use anyhow::Result;
use tera::Context;

use crate::services::links::LinksService;
use crate::services::link::LinkService;
use crate::repositories;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathReq {
    links_id: String
}

pub async fn endpoint_(data: Data<ActixData>, path: Path<PathReq>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    ctx.insert("site_description", &data.conf.site.description);
    ctx.insert("site_allowed_request", &data.conf.site.allow_request); // リンク追加リクエスト

    let pool = repositories::load_db(
        &data.conf.database.dbname,
    ).await?;

    let links = LinksService::links_from_id(&pool, &path.links_id).await?;
    let parent_category = CategoryService::category_from_id(&pool, &links.category).await?;

    ctx.insert("links_id", &links.id);
    ctx.insert("links_name", &links.name);
    ctx.insert("links_description", &links.description);
    ctx.insert("links_parent_categoryid", &links.category);
    ctx.insert("links_parent_categoryname", &parent_category.name);

    let links = LinkService::link_from_linksid(&pool, &links.id).await?;
    ctx.insert("link_list", &links); 

    let html = data.tera.render("links.html", &ctx)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn endpoint(data: Data<ActixData>, path: Path<PathReq>) -> HttpResponse {
    match endpoint_(data, path).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

