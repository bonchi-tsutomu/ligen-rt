use anyhow::Result;
use sqlx::{Pool, Sqlite};
use crate::repositories::links::LinksRepository;

pub struct LinksService;


impl LinksService {
    pub async fn create(
        &self,
        db: &Pool<Sqlite>,
        id: &str,
        name: &str,
        description: &str,
        category: &str
    ) -> Result<LinksRepository> {
        let repos = LinksRepository::new(id, name, description, category);
        repos.create(db).await?;
        Ok(repos)
    }

    pub async fn links_get_from_category(
        db: &Pool<Sqlite>,
        category: &str,
    ) -> Result<Vec<LinksRepository>> {
        LinksRepository::read_from_category(db, category).await
    }

    pub async fn links_from_id(
        db: &Pool<Sqlite>,
        id: &str,
    ) -> Result<LinksRepository> {
        LinksRepository::read(db, id).await
    }

}