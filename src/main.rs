extern crate rocket;

use rocket::{serde::json::Json, State};
use rocket::fairing::AdHoc;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

struct Asset {
    id: u64,
    asset_code: String,
    description: String,
    location: String,
    cost: u16,
}