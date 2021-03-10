//! Contains endpoint handlers.

use crate::{
    State,
    schema::{
        ErrorResponse, FancyApiQuery, FancyApiResponse, FancyApiResponseFrame, SimpleApiQuery,
        SimpleApiResponse,
    },
};

use async_std::sync::Arc;

use rand::prelude::*;
use tide::{http::StatusCode, Request, Response, Result as TideResult};

/// Handles `GET /`.
pub async fn index(request: Request<Arc<State>>) -> TideResult {
    let state = request.state();

    let mut rng = thread_rng();
    let lootbox = &state.lootbox;
    let dist = &state.distribution;

    let rarity = &lootbox.rarities[dist.sample(&mut rng)];
    let item = rarity
        .items
        .choose(&mut rng)
        .expect("A rarity set must have at least one item");

    let response = Response::builder(StatusCode::Found)
        .header("X-Lootbox-Title", &lootbox.title)
        .header("X-Lootbox-Item-Rarity", &rarity.label)
        .header("X-Lootbox-Item-Title", &item.title)
        .header("Location", &item.url)
        .build();

    Ok(response)
}

/// Handles `GET /api`.
pub async fn api(request: Request<Arc<State>>) -> TideResult {
    let query: SimpleApiQuery = request.query()?;
    let state = request.state();

    let mut rng = thread_rng();
    let lootbox = &state.lootbox;
    let dist = &state.distribution;

    let count = query.count.unwrap_or(1);
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        let rarity = &lootbox.rarities[dist.sample(&mut rng)];
        let item = rarity
            .items
            .choose(&mut rng)
            .expect("A rarity set must have at least one item");
        result.push(item.url.clone());
    }

    let response = Response::builder(StatusCode::Ok)
        .header("X-Lootbox-Title", &lootbox.title)
        .body(serde_json::to_value(SimpleApiResponse {
            title: lootbox.title.clone(),
            result,
        })?)
        .build();

    Ok(response)
}

/// Handles `GET /fancy`.
pub async fn fancy(request: Request<Arc<State>>) -> TideResult {
    let query: FancyApiQuery = request.query()?;
    let state = request.state();

    let mut rng = thread_rng();
    let lootbox = &state.lootbox;
    let dist = &state.distribution;

    let count = query.count.unwrap_or(1);
    let reserved_count = match query.reserved_count {
        Some(i) if i <= count => i,
        None => 0,
        _ => {
            let response = Response::builder(StatusCode::BadRequest)
                .body(serde_json::to_value(ErrorResponse {
                    code: 100,
                    reason: "Too many reserved frames".into(),
                })?)
                .build();
            return Ok(response);
        }
    };
    let reserved_index = match query.reserved_rarity {
        Some(i) if i < lootbox.rarities.len() => i,
        None => 0,
        _ => {
            let response = Response::builder(StatusCode::BadRequest)
                .body(serde_json::to_value(ErrorResponse {
                    code: 100,
                    reason: "Out of rarity range".into(),
                })?)
                .build();
            return Ok(response);
        }
    };

    let mut result = Vec::with_capacity(count);
    for i in 0..count {
        let reserved = count - i <= reserved_count;
        let index = if reserved {
            dist.sample(&mut rng).max(reserved_index)
        } else {
            dist.sample(&mut rng)
        };

        let rarity = &lootbox.rarities[index];
        let item = rarity
            .items
            .choose(&mut rng)
            .expect("A rarity set must have at least one item");
        result.push(FancyApiResponseFrame {
            rarity: rarity.label.clone(),
            title: item.title.clone(),
            url: item.url.clone(),
            reserved,
        });
    }

    let response = Response::builder(StatusCode::Ok)
        .header("X-Lootbox-Title", &lootbox.title)
        .body(serde_json::to_value(FancyApiResponse {
            title: lootbox.title.clone(),
            result,
        })?)
        .build();

    Ok(response)
}
