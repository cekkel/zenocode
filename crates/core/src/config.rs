use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
}

