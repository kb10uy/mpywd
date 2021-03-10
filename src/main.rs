pub mod data;
pub mod endpoint;
pub mod schema;

use async_std::{fs::read_to_string, path::Path, sync::Arc,};

use anyhow::Result;
use log::info;
use rand_distr::WeightedAliasIndex;

#[async_std::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let config = load_configuration("config.toml").await?;

    let weights = config.lootbox.rarity_weights().collect();
    let state = data::State {
        lootbox: config.lootbox,
        distribution: WeightedAliasIndex::new(weights)?,
    };

    let mut app = tide::with_state(Arc::new(state));
    app.at("/").get(endpoint::index);
    app.at("/api").get(endpoint::api);
    app.at("/fancy").get(endpoint::fancy);

    info!("Mpywd starting, listening at {}", config.listen_at);
    app.listen(config.listen_at).await?;
    Ok(())
}

async fn load_configuration(path: impl AsRef<Path>) -> Result<data::Configuration> {
    info!(
        "Loading configuration from {}",
        path.as_ref().to_str().unwrap_or_default()
    );
    let toml_string = read_to_string(path).await?;
    let toml = toml::from_str(&toml_string)?;

    Ok(toml)
}
