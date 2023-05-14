use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};
use axum::extract::State;

use proto::prisma::*;

use super::AppState;

pub struct CreateOrganizationRequest {
    organization_name: String,
    manager: manager::Data,
    warehouse: warehouse::Data,
}

pub async fn create_organization(
    State(state): State<AppState>,
    Bincode(payload): Bincode<CreateOrganizationRequest>,
) -> Result<Bincode<i32>> {
    let prisma = state.prisma;

    let org_id = prisma
        .organization()
        .create(payload.organization_name, vec![])
        .exec()
        .await?
        .id;

    let warehouse_id = prisma
        .warehouse()
        .create(
            payload.warehouse.name,
            payload.warehouse.address,
            organization::UniqueWhereParam::IdEquals(org_id),
            vec![],
        )
        .exec()
        .await?
        .id;

    let manager_id = prisma
        .manager()
        .create(
            payload.manager.username,
            payload.manager.email,
            payload.manager.password,
            warehouse::UniqueWhereParam::IdEquals(warehouse_id),
            vec![],
        )
        .exec()
        .await?
        .id;

    Ok(extractors::bincode::Bincode(manager_id))
}

pub async fn insert_manager_to_organization(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(m): Bincode<manager::Data>,
) -> Result<Bincode<i32>> {
    let warehouse_id = m.warehouse().unwrap().id;
    let warehouse_candidate = state
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
    if let Some(warehouse) = warehouse_candidate {
        let manager_id = state
            .prisma
            .manager()
            .create(
                m.username,
                m.email,
                m.password,
                warehouse::UniqueWhereParam::IdEquals(warehouse_id),
                vec![],
            )
            .exec()
            .await?
            .id;
        return Ok(extractors::bincode::Bincode(manager_id));
    }
    Err(anyhow::anyhow!("Not have permissions").into())
}

// pub async fn update_organization_config(
//     State(state): State<AppState>,
//     auth: AuthData,
//     Bincode(conf): Bincode<proto::organization::Configuration>,
// ) -> Result<Bincode<String>> {
//     let col = state
//         .mongo
//         .database(&auth.organization)
//         .collection::<Manager>("managers");
//     let manager = col
//         .find_one(doc! {"_id": auth.manager}, None)
//         .await?
//         .unwrap();

//     if manager.acl.organization {
//         let col = state
//             .mongo
//             .database(&auth.organization)
//             .collection::<Manager>("organizations");
//         col.update_one(
//             doc! {"_id": auth.organization},
//             doc! {"$set": {"config": to_bson(&conf).unwrap()}},
//             None,
//         )
//         .await?;
//     } else {
//         return Err(anyhow::anyhow!("Not have permissions").into());
//     }

//     Ok(extractors::bincode::Bincode("".into()))
// }
