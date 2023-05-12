use crate::base::extractors::{self, auth::AuthData, bincode::Bincode, Result};
use axum::extract::{Path, State};

use mongodb::bson::{self, doc, oid::ObjectId};
use proto::{storage::Cell, warehouse::Warehouse};

use super::AppState;

pub async fn insert_cell(
    State(state): State<AppState>,
    auth: AuthData,
    Path((warehouse_id, cell)): Path<(String, String)>,
) -> Result<Bincode<u64>> {
    let col = state
        .mongo
        .database(&auth.organization)
        .collection::<Warehouse>("warehouses");

    let cell = bson::to_bson(&Cell {
        _id: ObjectId::default().to_hex(),
        name: cell,
    })?;

    let res = col
        .update_one(
            doc! {"_id": warehouse_id},
            doc! {"$addToSet": {
                "cells": cell
            }},
            None,
        )
        .await?;

    Ok(extractors::bincode::Bincode(res.modified_count))
}
