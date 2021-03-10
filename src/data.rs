//! Contains configuration structs.

use rand_distr::WeightedAliasIndex;
use serde::Deserialize;

#[derive(Debug)]
pub struct State {
    pub lootbox: Lootbox,
    pub distribution: WeightedAliasIndex<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Configuration {
    pub listen_at: String,
    pub lootbox: Lootbox,
}

/// Represents a lootbox.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Lootbox {
    pub title: String,
    pub rarities: Vec<Rarity>,
}

/// Represents one of rarity group.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Rarity {
    pub label: String,
    pub probability: usize,
    pub items: Vec<Item>,
}

/// Represents an URL item.
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Item {
    pub title: String,
    pub url: String,
}

impl Lootbox {
    pub fn rarity_weights(&self) -> impl Iterator<Item = usize> + '_ {
        self.rarities.iter().map(|r| r.probability)
    }
}
