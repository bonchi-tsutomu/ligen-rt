use anyhow::Result;

pub mod link;
pub mod links;
pub mod link_attribute;
pub mod category;

use sqlx::{Sqlite, Pool};

pub async fn load_db(
    name: &str
) -> Result<Pool<Sqlite>> {
    Ok(Pool::connect(name).await?)
}