use actix_session::Session;
use actix_web::{HttpResponse, http::header, web::{Data, Form}};
use serde::Deserialize;
use crate::ActixData;
use anyhow::Result;
use tera::Context;

use crate::repositories;

use repositories::category::CategoryRepository;

#[derive(Deserialize)]
pub struct CreateFormReq {
    pub id: String,
    pub name: String,
    pub description: String
} 

pub async fn endpoint_create_(data: Data<ActixData>, req: Form<CreateFormReq>, session: Session) -> Result<HttpResponse> {
    if session.get("password")? == Some(data.conf.site.admin_password.clone()) {
        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;

        let repos = CategoryRepository::new(&req.id, &req.name, &req.description);

        repos.create(&pool).await?;

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

pub async fn endpoint_create(data: Data<ActixData>, req: Form<CreateFormReq>, session: Session) -> HttpResponse {
    match endpoint_create_(data, req, session).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}




#[derive(Deserialize)]
pub struct ModifyFormReq {
    pub id: String,
    pub name: String,
    pub description: String
} 

pub async fn endpoint_modify_(data: Data<ActixData>, req: Form<ModifyFormReq>, session: Session) -> Result<HttpResponse> {
    if session.get("password")? == Some(data.conf.site.admin_password.clone()) {
        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;

        let mut repos = CategoryRepository::read(&pool, &req.id).await?;
        
        repos.name = req.name.clone();
        repos.description = req.description.clone();

        repos.update(&pool).await?;

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

pub async fn endpoint_modify(data: Data<ActixData>, req: Form<ModifyFormReq>, session: Session) -> HttpResponse {
    match endpoint_modify_(data, req, session).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[derive(Deserialize)]
pub struct DeleteFormReq {
    pub id: String,
} 

pub async fn endpoint_delete_(data: Data<ActixData>, req: Form<DeleteFormReq>, session: Session) -> Result<HttpResponse> {
    if session.get("password")? == Some(data.conf.site.admin_password.clone()) {
        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;

        let repos = CategoryRepository::read(&pool, &req.id).await?;
        
        repos.delete(&pool).await?;

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

pub async fn endpoint_delete(data: Data<ActixData>, req: Form<DeleteFormReq>, session: Session) -> HttpResponse {
    match endpoint_delete_(data, req, session).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

