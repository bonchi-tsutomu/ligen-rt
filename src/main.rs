use actix_web::{HttpServer, App, web::{Data, get, post}};
use anyhow::Result;
use clap::Parser;
use tera::Tera;

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

    tera.add_template_file(themepath.join("base.html"), Some("base.html"))?;
    tera.add_template_file(themepath.join("index.html"), Some("index.html"))?;
    tera.add_template_file(themepath.join("request.html"), Some("request.html"))?;
    tera.add_template_file(themepath.join("request_result.html"), Some("request_result.html"))?;
    tera.add_template_file(themepath.join("search_result.html"), Some("search_result.html"))?;
    tera.add_template_file(themepath.join("category.html"), Some("category.html"))?;
    tera.add_template_file(themepath.join("links.html"), Some("links.html"))?;
    tera.add_template_file(themepath.join("error/request_not_allowed.html"), Some("error/request_not_allowed.html"))?;

    let data = ActixData {
        conf: conf,
        tera: tera
    };

    HttpServer::new(move || App::new()
        .app_data(Data::new(data.clone()))
        .route("/", get().to(views::index::endpoint))
        .route("/request/{category_id}", get().to(views::request::endpoint))
        .route("/request", post().to(controllers::request::endpoint))
        .route("/search", post().to(views::request::endpoint))
        .route("/category/{category_id}", get().to(views::category::endpoint))
        .route("/links/{links_id}", get().to(views::links::endpoint))
        .route("/jump/{link_id}", get().to(views::linkjump::endpoint))
    )
        .bind(addr)?
        .run().await?;

    Ok(())
}