use axum::extract::State;
use proto::prisma::*;

use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};

use super::AppState;

#[derive(serde::Deserialize)]
pub struct FindProductsQuery {}

pub async fn find_products(
    State(state): State<AppState>,
    auth: AuthData,
    // query: Query<FindProductsQuery>,
    Bincode(products): Bincode<FindProductsQuery>,
) -> Result<Bincode<Vec<product::Data>>> {
    let products = state
        .prisma
        .product()
        .find_many(vec![product::WhereParam::OrganizationId(
            read_filters::IntFilter::Equals(auth.organization),
        )])
        .take(50)
        .exec()
        .await?;

    Ok(extractors::bincode::Bincode(products))
}

pub async fn insert_products(
    auth: AuthData,
    State(state): State<AppState>,
    Bincode(products): Bincode<Vec<product::Data>>,
) -> Result<()> {
    let prisma = state.prisma;
    prisma
        ._transaction()
        .run(|client| async move {
            for product in products {
                client
                    .product()
                    .create(
                        product.name,
                        product.description,
                        product.price,
                        organization::UniqueWhereParam::IdEquals(auth.organization),
                        vec![],
                    )
                    .exec()
                    .await?;
            }
            Ok::<(), anyhow::Error>(())
        })
        .await?;

    // tokio::spawn(async move {
    //     let index = state.meili.index("products");
    //     index.add_documents(&products, Some("_id")).await.unwrap();
    // });

    Ok(())
}
