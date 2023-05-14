use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};
use axum::{body::StreamBody, extract::State, response::IntoResponse};

use futures::StreamExt;

use proto::prisma::*;

use super::AppState;

pub async fn insert_order(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(order): Bincode<order::Data>,
) -> Result<Bincode<i32>> {
    // let id = Order::insert(&state.pool, &state.meili, order).await?;
    let id = state
        .prisma
        .order()
        .create(
            organization::UniqueWhereParam::IdEquals(order.organization_id),
            warehouse::UniqueWhereParam::IdEquals(order.warehouse_id),
            customer::UniqueWhereParam::IdEquals(order.customer_id),
            "Created".into(),
            vec![],
        )
        .exec()
        .await?
        .id;

    Ok(extractors::bincode::Bincode(id))
}

// pub async fn find_orders(
//     State(state): State<AppState>,
//     auth: AuthData,
//     Bincode(order): Bincode<Order>,
// ) -> impl IntoResponse {
//     let res = StreamBody::new(
//         Order::find(
//             &state.pool,
//             vec![OrderFilter::OrganizationId(auth.organization)],
//         )
//         .await
//         .map(|Ok(o)| async { Bincode(o) }),
//     );

//     Ok(res)
// }

pub async fn update_order(
    State(state): State<AppState>,
    auth: AuthData,
    Bincode(order): Bincode<order::Data>,
) -> Result<Bincode<u64>> {
    // let col = state
    //     .mongo
    //     .database(&auth.organization)
    //     .collection::<Order>("orders");

    // let res = col
    //     .replace_one(doc! {"_id": &order._id}, &order, None)
    //     .await?;

    // tokio::spawn(async move {
    //     let index = state.meili.index("orders");
    //     index.add_or_update(&[order], Some("_id")).await.unwrap();
    // });

    Ok(extractors::bincode::Bincode(0))
}
