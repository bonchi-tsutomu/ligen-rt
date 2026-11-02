use serde::Serialize;
use sqlx::{FromRow, Pool, Sqlite, query, query_as};
use anyhow::Result;
use crate::utils;

#[derive(FromRow, Serialize)]
pub struct LinkAttributeRepository {
    pub linkid: String,
    pub linksid: String,
}


impl LinkAttributeRepository {
    pub fn new(
        linkid: &str,
        linksid: &str
    ) -> Self {
        Self {
            linkid: linkid.to_string(),
            linksid: linksid.to_string()
        }
    }

    pub async fn create(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;
        
        query("INSERT INTO link_attribute (linkid, linksid, id) VALUES ($1, $2, $3)")
            .bind(&self.linkid)
            .bind(&self.linksid)
            .bind(&utils::gen_id::generate_id(8))
            .execute(db).await?;

        transaction.commit().await?;

        Ok(())
    }
    
    pub async fn read_from_linkid_and_linksid(db: &Pool<Sqlite>, linkid: &str, linksid: &str) -> Result<Vec<Self>> {
        let link = query_as::<Sqlite, Self>("SELECT linkid, linksid FROM link_attribute WHERE linkid = $1 AND linksid = $2")
            .bind(linkid)
            .bind(linksid)
            .fetch_all(db).await?;

        Ok(link)
    }

    pub async fn read_from_ids(db: &Pool<Sqlite>, linksid: &str) -> Result<Vec<Self>> {
        let link = query_as::<Sqlite, Self>("SELECT linkid, linksid FROM link_attribute WHERE linksid = $1")
            .bind(linksid)
            .fetch_all(db).await?;

        Ok(link)
    }

    pub async fn read_all(db: &Pool<Sqlite>) -> Result<Vec<Self>> {
        let link = query_as::<Sqlite, Self>("SELECT linkid, linksid FROM link_attribute")
            .fetch_all(db).await?;

        Ok(link)
    }

    pub async fn update(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("UPDATE link_attribute SET linkid = $1, linksid = $2 WHERE linkid = $3 AND linksid = 4")
            .bind(&self.linkid)
            .bind(&self.linksid)
            .bind(&self.linkid)
            .bind(&self.linksid)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, db: &Pool<Sqlite>) -> Result<()> {
        let transaction = db.begin().await?;

        query("DELETE FROM link_attribute WHERE linkid = $1 AND linksid = $2")
            .bind(&self.linkid)
            .bind(&self.linksid)
            .execute(db).await?;
        
        transaction.commit().await?;

        Ok(())
    }
    
}