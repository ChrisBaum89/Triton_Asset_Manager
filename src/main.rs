extern crate rocket;

use rocket::{serde::json::Json, State};
use rocket::fairing::AdHoc;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
