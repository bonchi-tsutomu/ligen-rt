use actix_web::{HttpServer, App, web::{Data, get, post}, cookie::Key};
use actix_session::{
    Session,
    SessionMiddleware,
    storage::CookieSessionStore,
};
use anyhow::Result;
use clap::Parser;
use tera::Tera;

mod admin;

use std::path::{Path, PathBuf};

mod utils;
mod views;
mod controllers;
mod repositories;
mod services;

#[derive(Clone)]
struct ActixData {
    conf: utils::config::MasterConfig,
    tera: Tera
} 

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = utils::args::Args::parse();
    let conf = utils::config::read_config(&Path::new(&args.config))?;
    let mut tera = Tera::new();

    let addr = format!("{}:{}", &conf.server.bind, conf.server.port);

    let themepath = PathBuf::new().join(&conf.server.theme);

    let secret_key = Key::from(&conf.site.secret_key.clone().into_bytes());

    tera.add_template_file(themepath.join("base.html"), Some("base.html"))?;
    tera.add_template_file(themepath.join("index.html"), Some("index.html"))?;
    tera.add_template_file(themepath.join("request.html"), Some("request.html"))?;
    tera.add_template_file(themepath.join("request_result.html"), Some("request_result.html"))?;
    tera.add_template_file(themepath.join("search_result.html"), Some("search_result.html"))?;
    tera.add_template_file(themepath.join("category.html"), Some("category.html"))?;
    tera.add_template_file(themepath.join("links.html"), Some("links.html"))?;
    tera.add_template_file(themepath.join("error/request_not_allowed.html"), Some("error/request_not_allowed.html"))?;
    tera.add_template_file(themepath.join("admin/index.html"), Some("admin/index.html"))?;
    tera.add_template_file(themepath.join("admin/login.html"), Some("admin/login.html"))?;

    let data = ActixData {
        conf: conf,
        tera: tera
    };

    HttpServer::new(move || App::new()
        .wrap(
            SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            )
        )
        .app_data(Data::new(data.clone()))
        .route("/", get().to(views::index::endpoint))
        .route("/request/{category_id}", get().to(views::request::endpoint))
        .route("/request", post().to(controllers::request::endpoint))
        .route("/search", post().to(views::request::endpoint))
        .route("/category/{category_id}", get().to(views::category::endpoint))
        .route("/links/{links_id}", get().to(views::links::endpoint))
        .route("/jump/{link_id}", get().to(views::linkjump::endpoint))

        .route("/admin/", get().to(admin::views::index::endpoint))
        .route("/admin/login", post().to(admin::controllers::login::endpoint))

        .route("/admin/add/category", post().to(admin::controllers::category::endpoint_create))
        .route("/admin/mod/category", post().to(admin::controllers::category::endpoint_modify))
        .route("/admin/del/category", post().to(admin::controllers::category::endpoint_delete))

        .route("/admin/add/links", post().to(admin::controllers::links::endpoint_create))
        .route("/admin/mod/links", post().to(admin::controllers::links::endpoint_modify))
        .route("/admin/del/links", post().to(admin::controllers::links::endpoint_delete))

        .route("/admin/add/link", post().to(admin::controllers::link::endpoint_create))
        .route("/admin/mod/link", post().to(admin::controllers::link::endpoint_modify))
        .route("/admin/del/link", post().to(admin::controllers::link::endpoint_delete))

        .route("/admin/add/attribute", post().to(admin::controllers::attribute::endpoint_create))
        .route("/admin/del/attribute", post().to(admin::controllers::attribute::endpoint_delete))
    )
        .bind(addr)?
        .run().await?;

    Ok(())
}