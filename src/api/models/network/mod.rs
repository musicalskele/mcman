use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub const NETWORK_TOML: &str = "network.toml";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Network {
    pub name: String,
    pub variables: HashMap<String, String>,

    pub servers: HashMap<String, ServerEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ServerEntry {
    pub groups: Vec<String>,
}
