use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};
use axum::extract::{Path, State};

use proto::prisma::*;

use super::AppState;

pub async fn insert_cell(
    State(state): State<AppState>,
    auth: AuthData,
    Path((warehouse_id, cell)): Path<(i32, String)>,
) -> Result<Bincode<i32>> {
    let warehouse = state
        .prisma
        .warehouse()
        .find_first(vec![
            warehouse::WhereParam::OrganizationId(read_filters::IntFilter::Equals(
                auth.organization,
            )),
            warehouse::WhereParam::Id(read_filters::IntFilter::Equals(warehouse_id)),
        ])
        .exec()
        .await?;

    if let Some(warehouse) = warehouse {
        let cell = state
            .prisma
            .storage_cell()
            .create(warehouse::UniqueWhereParam::IdEquals(warehouse.id), vec![])
            .exec()
            .await?
            .id;
        return Ok(extractors::bincode::Bincode(cell));
    }

    Err(anyhow::anyhow!("Warehouse not found").into())
}
