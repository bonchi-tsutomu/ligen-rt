use crate::repositories::category::CategoryRepository;

use sqlx::{FromRow, Pool, Sqlite, query, query_as};
use anyhow::Result;

pub struct CategoryService;

impl CategoryService {
    pub async fn categories_all(pool: &Pool<Sqlite>) -> Result<Vec<CategoryRepository>> {
        CategoryRepository::read_all(pool).await
    } 

    pub async fn category_from_id(pool: &Pool<Sqlite>, id: &str) -> Result<CategoryRepository> {
        CategoryRepository::read(pool, id).await
    }
}