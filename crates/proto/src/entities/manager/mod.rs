use futures::stream::BoxStream;

pub mod header {
    pub const MANAGER: &str = "X-Manager-Id";
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct Manager {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub organization_id: u64,
    pub warehouse_id: u64, // Warehouse
    // pub contacts: Vec<Contact>,
    // pub acl: JsonValue,
    pub created_at: Option<time::OffsetDateTime>,
    pub updated_at: Option<time::OffsetDateTime>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub enum Contact {
    Phone(String),
    #[default]
    None,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
pub struct ACL {
    pub orders: bool,
    pub statistic: bool,
    pub storage: bool,
    pub rights: bool, // Can edit ACL for others
    pub calls: bool,
    pub organization: bool, // Edit any info in organization
}

// pub enum ManagerFilter {
//     OrganizationId(u64),
// }

// impl ToSql for Vec<ManagerFilter> {
//     fn to_sql(&self) -> String {
//         let mut result = String::new();
//         for (idx, filter) in self.iter().enumerate() {
//             match filter {
//                 ManagerFilter::OrganizationId(id) => {
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
// impl Entitie<Manager> for Manager {
//     const TABLE_NAME: &'static str = "managers";
//     type Filter = Vec<ManagerFilter>;

//     async fn find(
//         pool: &sqlx::MySqlPool,
//         filter: Self::Filter,
//     ) -> BoxStream<Result<Manager, sqlx::Error>> {
//         sqlx::query_as!(Manager, "SELECT * FROM managers where ?", filter.to_sql()).fetch(pool)
//     }

//     async fn find_by_id(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<Manager> {
//         Ok(
//             sqlx::query_as!(Manager, "SELECT * FROM managers WHERE id = ?", id)
//                 .fetch_one(pool)
//                 .await?,
//         )
//     }

//     async fn insert(
//         pool: &sqlx::MySqlPool,
//         meili: &meilisearch_sdk::Client,
//         mut item: Manager,
//     ) -> anyhow::Result<u64> {
//         let mut tx = pool.begin().await?;
//         sqlx::query!(
//             "INSERT INTO managers (username, email, password, organization_id, warehouse_id, acl) VALUES (?, ?, ?, ?, ?, ?);",
//             item.username,
//             item.email,
//             item.password,
//             item.organization_id,
//             item.warehouse_id,
//             item.acl
//         )
//         .execute(&mut tx)
//         .await?;
//         item.id = sqlx::query!("SELECT LAST_INSERT_ID() as id;")
//             .fetch_one(&mut tx)
//             .await?
//             .id
//             .unwrap_or_default();
//         Manager::index(meili, &item).await?;
//         tx.commit().await?;
//         Ok(item.id)
//     }

//     async fn update(pool: &sqlx::MySqlPool, item: Manager) -> anyhow::Result<()> {
//         todo!()
//     }

//     async fn delete(pool: &sqlx::MySqlPool, id: u64) -> anyhow::Result<()> {
//         todo!()
//     }

//     async fn index(meili: &meilisearch_sdk::Client, item: &Manager) -> anyhow::Result<()> {
//         let index = meili.index("managers");
//         let key = item.id.to_string();

//         index.add_documents(&[item], Some(&key)).await?;
//         Ok(())
//     }
// }
