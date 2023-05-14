use futures::stream::BoxStream;

use crate::{Entitie, ToSql};

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Status {
    pub _id: String,
    pub slug: String,
    pub triggers: Vec<Trigger>,
    pub next: Vec<String>, // Next avaliable status | You can't change from closed to new
    pub theme: Option<String>,
    pub updated_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Trigger {}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Order {
    pub id: u64,
    pub manager_id: u64,           // id of Order
    pub organization_id: u64,      // id of Organization
    pub warehouse_id: Option<u64>, // id of Warehouse
    pub status: String,
    pub products_id: u64, // ids of products
    // pub activity: Vec<String>,
    // pub notes: Vec<Note>,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Note {
    pub manager: String,
    pub body: String,
    pub done: Option<bool>,
    pub assign: Option<String>, // Manager id
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Activity {
    pub manager: String,
    pub action: String,
    pub created_at: i64,
}

pub enum OrderFilter {
    OrganizationId(u64),
}

impl ToSql for Vec<OrderFilter> {
    fn to_sql(&self) -> String {
        let mut result = String::new();
        for (idx, filter) in self.iter().enumerate() {
            match filter {
                OrderFilter::OrganizationId(id) => {
                    result.push_str(&format!("organization_id = {}", id))
                } // Self::Name(name) => format!("name = {}", name),
            }
            if idx != self.len() {
                result.push(',')
            }
        }
        result
    }
}

#[async_trait::async_trait]
impl Entitie<Order> for Order {
    const TABLE_NAME: &'static str = "orders";
    type Filter = Vec<OrderFilter>;

    async fn find(
        pool: &sqlx::MySqlPool,
        filter: Self::Filter,
    ) -> BoxStream<Result<Order, sqlx::Error>> {
        sqlx::query_as!(Order, "SELECT * FROM orders where ?", filter.to_sql()).fetch(pool)
    }

    async fn find_by_id(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<Order> {
        Ok(
            sqlx::query_as!(Order, "SELECT * FROM orders WHERE id = ?", id)
                .fetch_one(pool)
                .await?,
        )
    }

    async fn insert(
        pool: &sqlx::MySqlPool,
        meili: &meilisearch_sdk::Client,
        mut item: Order,
    ) -> anyhow::Result<u64> {
        let mut tx = pool.begin().await?;
        sqlx::query!(
            "INSERT INTO orders (organization_id, warehouse_id, manager_id, products_id, status) VALUES (?, ?, ?, ?, ?);",
            item.organization_id,
            item.warehouse_id,
            item.manager_id,
            item.products_id,
            item.status,
        )
        .execute(&mut tx)
        .await?;
        item.id = sqlx::query!("SELECT LAST_INSERT_ID() as id;")
            .fetch_one(&mut tx)
            .await?
            .id
            .unwrap_or_default();
        Order::index(meili, &item).await?;
        tx.commit().await?;
        Ok(item.id)
    }

    async fn update(pool: &sqlx::MySqlPool, item: Order) -> anyhow::Result<()> {
        todo!()
    }

    async fn delete(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<()> {
        todo!()
    }

    async fn index(meili: &meilisearch_sdk::Client, item: &Order) -> anyhow::Result<()> {
        let index = meili.index("orders");
        let key = item.id.to_string();

        index.add_documents(&[item], Some(&key)).await?;
        Ok(())
    }
}
