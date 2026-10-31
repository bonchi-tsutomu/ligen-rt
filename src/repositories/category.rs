use sqlx::{FromRow, Pool, Sqlite, query, query_as};
use anyhow::Result;
use serde::Serialize;

#[derive(FromRow, Serialize)]
pub struct CategoryRepository {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl CategoryRepository {
    pub fn new(
        id: &str,
        name: &str,
        description: &str
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string()
        }
    }

    pub async fn create(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;
        
        query("INSERT INTO category (id, name, description) VALUES ($1, $2, $3)")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.description)
            .execute(db).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn read(db: &Pool<Sqlite>, id: &str) -> Result<Self> {
        let link = query_as::<Sqlite, Self>("SELECT id, name, description FROM category WHERE id = $1")
            .bind(id)
            .fetch_one(db).await?;

        Ok(link)
    }

    pub async fn read_all(db: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let link = query_as::<Sqlite, Self>("SELECT id, name, description FROM category")
            .fetch_all(db).await?;

        Ok(link)
    }

    pub async fn update(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("UPDATE category SET name = $1, description = $2 WHERE id = $3")
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.id)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("DELETE FROM category WHERE id = $1")
            .bind(&self.id)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }

}