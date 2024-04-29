use axum::{routing::get, Router};
use controller::get_root_controller;

use crate::controller;

pub fn create_router() -> Router {
    Router::new().route("/", get(get_root_controller))
}
