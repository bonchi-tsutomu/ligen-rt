use actix_web::{HttpResponse, web::{Data, Path}};
use crate::ActixData;
use anyhow::Result;
use tera::Context;

use crate::services::links::LinksService;
use crate::repositories;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathReq {
    category_id: String
}


pub async fn endpoint_(data: Data<ActixData>, path: Path<PathReq>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    
    if data.conf.site.allow_request {
        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;

        let links = LinksService::links_get_from_category(&pool, &path.category_id).await?;

        ctx.insert("site_guidelines", &data.conf.site.guidelines);
        ctx.insert("links_list", &links);

        Ok(HttpResponse::Ok().content_type("text/html").body(data.tera.render("request.html", &ctx)?))
    } else {
        Ok(HttpResponse::Forbidden().content_type("text/html").body(data.tera.render("error/request_not_allowed.html", &ctx)?))
    }

}

pub async fn endpoint(data: Data<ActixData>, path: Path<PathReq>) -> HttpResponse {
    match endpoint_(data, path).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

