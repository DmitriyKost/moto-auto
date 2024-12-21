use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

pub const API_KEY: &str = "apikey";

#[derive(Default, Deserialize, Serialize)]
pub struct ApiKey(pub String);

pub type Cache = Arc<RwLock<HashMap<String, String>>>;
