
use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite, query, query_as};
use anyhow::Result;

#[derive(FromRow, Serialize)]
pub struct LinkRepository {
    pub id: String,
    pub url: String,
    pub contributor: String,
    pub description: String,
    pub counter: i32
}

impl LinkRepository {
    pub fn new(
        id: &str,
        url: &str,
        contributor: &str,
        description: &str
    ) -> Self {
        Self {
            id: id.to_string(),
            url: url.to_string(),
            contributor: contributor.to_string(),
            description: description.to_string(),
            counter: 0
        }
    }

    pub async fn create(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;
        
        query("INSERT INTO link (id, url, contributor, description, counter) VALUES ($1, $2, $3, $4, $5)")
            .bind(&self.id)
            .bind(&self.url)
            .bind(&self.contributor)
            .bind(&self.description)
            .bind(&self.counter)
            .execute(db).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn read(db: &Pool<Sqlite>, id: &str) -> Result<Self> {
        let link = query_as::<Sqlite, Self>("SELECT id, url, contributor, description, counter FROM link WHERE id = $1")
            .bind(id)
            .fetch_one(db).await?;

        Ok(link)
    }

    pub async fn read_limit(db: &Pool<Sqlite>, limit: usize) -> Result<Vec<Self>> {
        let links = query_as::<Sqlite, Self>("SELECT id, url, contributor, description, counter FROM link LIMIT $1")
            .bind(limit as i32)
            .fetch_all(db).await?;

        Ok(links)
    }

    pub async fn update(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("UPDATE link SET url = $1, description = $2, counter = $3 WHERE id = $4")
            .bind(&self.url)
            .bind(&self.description)
            .bind(&self.counter)
            .bind(&self.id)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("DELETE FROM link WHERE id = $1")
            .bind(&self.id)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }
    
}