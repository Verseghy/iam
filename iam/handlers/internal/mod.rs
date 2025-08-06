mod v1;

use crate::state::StateTrait;
use axum::Router;

pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new().nest("/v1", v1::routes::<S>())
}
