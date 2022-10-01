
use super::*;

/// Each game listing has an ID associated with it.
#[derive(Debug, Component)]
pub struct ID(pub u32);

impl Default for ID {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Deserialize, Debug, Component)]
pub struct GameListing {
    pub metadata: Metadata,
    pub config: Config
}

impl Default for GameListing {
    fn default() -> Self {
        Self { metadata: Default::default(), config: Default::default() }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Metadata {
    pub name: String,
    pub authors: Vec<String>,
    pub creation_date: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub img_path: String,
    pub exec_path: String,
    pub scores_path: Option<String>,
    pub volume: Option<u8>,
}