pub mod api;
mod app;
pub mod jwt;
pub mod testing;
mod user;

use api::Api;
use std::sync::Arc;

pub use app::App;
pub use user::User;

#[derive(Debug)]
pub struct IamInner {
    api: Api,
}

#[derive(Debug, Clone)]
pub struct Iam {
    inner: Arc<IamInner>,
}

impl Iam {
    pub fn new(base_url: &str) -> Self {
        Self {
            inner: Arc::new(IamInner {
                api: Api::new(base_url, None).unwrap(),
            }),
        }
    }

    pub fn api(&self) -> &Api {
        &self.inner.api
    }
}
