use actix_web::{HttpResponse, web::{Data, Form}};
use serde::Deserialize;
use crate::{ActixData, utils};
use anyhow::Result;
use tera::Context;

use crate::services;
use crate::repositories;

#[derive(Deserialize)]
pub struct FormReq {
    pub linksid: String,
    pub url: String,
    pub description: String,
    pub handlename: String,
} 

pub async fn endpoint_(data: Data<ActixData>, req: Form<FormReq>) -> Result<HttpResponse> {
    let mut ctx = Context::new();

    ctx.insert("site_title", &data.conf.site.title);
    
    if data.conf.site.allow_request {

        let pool = repositories::load_db(
            &data.conf.database.dbname,
        ).await?;
        
        let linkid = utils::gen_id::generate_id(8);

        services::link::LinkService::create(
            &pool,
            &linkid,
            &req.url,
            &req.description,
            &req.handlename,
        ).await?;

        let linksid_list: Vec<String> = req.linksid.split("\n").map(|id| id.to_string()).collect();

        for linksid in linksid_list {
            let lar = repositories::link_attribute::LinkAttributeRepository::new(&linkid, &linksid);
            lar.create(&pool).await?;
        }

        Ok(HttpResponse::Ok().content_type("text/html").body(data.tera.render("request_result.html", &ctx)?))
    } else {
        Ok(HttpResponse::Forbidden().content_type("text/html").body(data.tera.render("error/request_not_allowed.html", &ctx)?))
    }

}

pub async fn endpoint(data: Data<ActixData>, req: Form<FormReq>) -> HttpResponse {
    match endpoint_(data, req).await {
        Ok(response) => response,
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

