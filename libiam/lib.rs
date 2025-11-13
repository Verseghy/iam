pub mod api;
mod app;
pub mod jwt;
pub mod testing;
mod user;

use crate::jwt::Jwt;
use api::Api;
use std::sync::Arc;

pub use app::App;
pub use user::User;

#[derive(Debug)]
pub struct IamInner {
    api: Api,
    jwt: Jwt,
}

#[derive(Debug, Clone)]
pub struct Iam {
    inner: Arc<IamInner>,
}

impl Iam {
    pub async fn new(base_url: &str) -> anyhow::Result<Self> {
        let api = Api::new(base_url, None)?;

        Ok(Self {
            inner: Arc::new(IamInner {
                jwt: Jwt::new(&api).await?,
                api,
            }),
        })
    }

    pub fn api(&self) -> &Api {
        &self.inner.api
    }
}
