use actix_session::Session;
use actix_web::{HttpResponse, http::header, web::{Data, Form}};
use serde::Deserialize;
use crate::{ActixData, utils};
use anyhow::Result;
use tera::Context;

#[derive(Deserialize)]
pub struct FormReq {
    pub password: String
} 

pub async fn endpoint_(data: Data<ActixData>, req: Form<FormReq>, session: Session) -> Result<HttpResponse> {
    if req.password == data.conf.site.admin_password {
        session.insert("password", &req.password)?;
        Ok(HttpResponse::build(actix_web::http::StatusCode::FOUND)
            .insert_header((header::LOCATION, "/admin/"))
            .finish())

    } else {
        let mut ctx = Context::new();

        ctx.insert("site_title", &data.conf.site.title);
        ctx.insert("site_description", &data.conf.site.description);
        ctx.insert("site_allowed_request", &data.conf.site.allow_request); // リンク追加リクエスト

        let html = data.tera.render("admin/login.html", &ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    }
}

pub async fn endpoint(data: Data<ActixData>, req: Form<FormReq>, session: Session) -> HttpResponse {
    match endpoint_(data, req, session).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

