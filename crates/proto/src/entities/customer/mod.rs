use futures::stream::BoxStream;
use time::Date;

use crate::manager::Contact;

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

// pub enum CustomerFilter {
//     OrganizationId(u64),
// }

// impl ToSql for Vec<CustomerFilter> {
//     fn to_sql(&self) -> String {
//         let mut result = String::new();
//         for (idx, filter) in self.iter().enumerate() {
//             match filter {
//                 CustomerFilter::OrganizationId(id) => {
//                     result.push_str(&format!("organization_id = {}", id))
//                 } // Self::Name(name) => format!("name = {}", name),
//             }
//             if idx != self.len() {
//                 result.push(',')
//             }
//         }
//         result
//     }
// }

// #[async_trait::async_trait]
// impl Entitie<Customer> for Customer {
//     const TABLE_NAME: &'static str = "customers";
//     type Filter = Vec<CustomerFilter>;

//     async fn find(
//         pool: &sqlx::MySqlPool,
//         filter: Self::Filter,
//     ) -> BoxStream<Result<Customer, sqlx::Error>> {
//         sqlx::query_as!(Customer, "SELECT * FROM customers where ?", filter.to_sql()).fetch(pool)
//     }

//     async fn find_by_id(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<Customer> {
//         Ok(
//             sqlx::query_as!(Customer, "SELECT * FROM customers WHERE id = ?", id)
//                 .fetch_one(pool)
//                 .await?,
//         )
//     }

//     async fn insert(
//         pool: &sqlx::MySqlPool,
//         meili: &meilisearch_sdk::Client,
//         mut item: Customer,
//     ) -> anyhow::Result<u64> {
//         let mut tx = pool.begin().await?;
//         sqlx::query!(
//             "INSERT INTO customers (name, address, organization_id) VALUES (?, ?, ?);",
//             item.name,
//             item.address,
//             item.organization_id
//         )
//         .execute(&mut tx)
//         .await?;
//         item.id = sqlx::query!("SELECT LAST_INSERT_ID() as id;")
//             .fetch_one(&mut tx)
//             .await?
//             .id
//             .unwrap_or_default();
//         Customer::index(meili, &item).await?;
//         tx.commit().await?;
//         Ok(item.id)
//     }

//     async fn update(pool: &sqlx::MySqlPool, item: Customer) -> anyhow::Result<()> {
//         todo!()
//     }

//     async fn delete(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<()> {
//         todo!()
//     }

//     async fn index(meili: &meilisearch_sdk::Client, item: &Customer) -> anyhow::Result<()> {
//         let index = meili.index("customers");
//         let key = item.id.to_string();

//         index.add_documents(&[item], Some(&key)).await?;
//         Ok(())
//     }
// }
