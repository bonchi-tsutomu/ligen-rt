use crate::{
    repositories::{link::LinkRepository, link_attribute::LinkAttributeRepository},
    utils::config::SiteConfig
};

use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct LinkService;

impl LinkService {
    pub async fn create(
        db: &Pool<Sqlite>,
        id: &str,
        url: &str,
        description: &str,
        contributor: &str
    ) -> Result<LinkRepository> {
        let repos = LinkRepository::new(id, url, contributor, description);
        repos.create(db).await?;
        Ok(repos)
    }

    pub async fn increment(repos: &mut LinkRepository, pool: &Pool<Sqlite>) -> Result<()> {
        repos.counter += 1;
        repos.update(pool).await?;
        Ok(())
    }
    
    pub async fn read_from_id(pool: &Pool<Sqlite>, id: &str) -> Result<LinkRepository> {
        LinkRepository::read(pool, id).await
    }
    
    pub async fn link_from_limit(pool: &Pool<Sqlite>, limit: usize) -> Result<Vec<LinkRepository>> {
        LinkRepository::read_limit(pool, limit).await
    }

    // N+1問題は将来的に解決する
    pub async fn link_from_linksid(pool: &Pool<Sqlite>, linksid: &str) -> Result<Vec<LinkRepository>> {
        let attributes = LinkAttributeRepository::read_from_ids(pool, linksid).await?;
        let mut links = Vec::new();
        for attribute in attributes {
            log::info!("GetAttribute {} {}", attribute.linkid, attribute.linksid);
            links.push(LinkService::read_from_id(pool, &attribute.linkid).await?);
        }

        Ok(links)
    }
}