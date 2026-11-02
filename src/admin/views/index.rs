use actix_web::{HttpResponse, web::Data};
use crate::ActixData;
use anyhow::Result;
use tera::Context;

use crate::services::link::LinkService;
use crate::services::links::LinksService;
use crate::services::category::CategoryService;

use crate::repositories::link::LinkRepository;
use crate::repositories::links::LinksRepository;
use crate::repositories::category::CategoryRepository;
use crate::repositories::link_attribute::LinkAttributeRepository;

use crate::repositories;

use actix_session::Session;




pub async fn endpoint_(data: Data<ActixData>, session: Session) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    ctx.insert("site_description", &data.conf.site.description);
    ctx.insert("site_allowed_request", &data.conf.site.allow_request); // リンク追加リクエスト

    let html = if session.get("password")? == Some(data.conf.site.admin_password.clone()) {

        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;

        ctx.insert("link_list", &LinkRepository::read_all(&pool).await?);
        ctx.insert("links_list", &LinksRepository::read_all(&pool).await?);
        ctx.insert("category_list", &CategoryRepository::read_all(&pool).await?);
        ctx.insert("attribute_list", &LinkAttributeRepository::read_all(&pool).await?);

        data.tera.render("admin/index.html", &ctx)?

    } else {
        data.tera.render("admin/login.html", &ctx)?
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub async fn endpoint(data: Data<ActixData>, session: Session) -> HttpResponse {
    match endpoint_(data, session).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

