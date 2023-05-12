use time::Date;

use crate::{manager::Contact, Entitie};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Customer {
    pub id: u64,
    pub name: String,
    pub address: String,
    // pub contacts: Vec<Contact>,
    pub organization_id: u64,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[async_trait::async_trait]
impl Entitie<Customer> for Customer {
    const TABLE_NAME: &'static str = "customers";

    async fn find(pool: &sqlx::MySqlPool) -> anyhow::Result<Vec<Customer>> {
        sqlx::query_as!(Customer, "SELECT * FROM customers")
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))
    }

    async fn find_by_id(pool: &sqlx::MySqlPool, id: i32) -> anyhow::Result<Customer> {
        Ok(
            sqlx::query_as!(Customer, "SELECT * FROM customers WHERE id = ?", id)
                .fetch_one(pool)
                .await?,
        )
    }

    async fn insert(pool: &sqlx::MySqlPool, item: Customer) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO customers (name, address, organization_id) VALUES (?, ?, ?)",
            item.name,
            item.address,
            item.organization_id
        )
        .execute(pool)
        .await
        .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }

    fn update<'life0, 'async_trait>(
        pool: &'life0 sqlx::MySqlPool,
        item: Customer,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = anyhow::Result<()>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn delete<'life0, 'async_trait>(
        pool: &'life0 sqlx::MySqlPool,
        id: i32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = anyhow::Result<()>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn count<'life0, 'async_trait>(
        pool: &'life0 sqlx::MySqlPool,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = anyhow::Result<i64>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }

    fn exists<'life0, 'async_trait>(
        pool: &'life0 sqlx::MySqlPool,
        id: i32,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = anyhow::Result<bool>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
    {
        todo!()
    }
}
