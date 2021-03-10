//! Contains HTTP requests and responses structs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ErrorResponse {
    pub reason: String,
    pub code: usize,
}

/// Represents the query parameter of `GET /api`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SimpleApiQuery {
    pub count: Option<usize>,
}

/// Represents the response object of `GET /api`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SimpleApiResponse {
    pub title: String,
    pub result: Vec<String>,
}

/// Represents the query parameter of `GET /fancy`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FancyApiQuery {
    pub count: Option<usize>,
    pub reserved_count: Option<usize>,
    pub reserved_rarity: Option<usize>,
}

/// Represents the response object of `GET /api`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FancyApiResponse {
    pub title: String,
    pub result: Vec<FancyApiResponseFrame>,
}

/// Represents a frame of lootbox in `GET /fancy`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FancyApiResponseFrame {
    /// The rarity of this frame.
    pub rarity: String,

    /// Whether this frame is reserved. (確定枠)
    pub reserved: bool,

    /// Title
    pub title: String,

    /// URL
    pub url: String,
}
