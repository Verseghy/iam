mod app;
mod error;
mod user;
mod utils;

use std::sync::Arc;

pub use app::App;
pub use user::User;

#[derive(Debug)]
pub struct IamInner {
    base_url: String,
}

#[derive(Debug, Clone)]
pub struct Iam {
    inner: Arc<IamInner>,
}

impl Iam {
    pub fn new(base_url: &str) -> Self {
        Self {
            inner: Arc::new(IamInner {
                base_url: base_url.to_owned(),
            }),
        }
    }

    pub(crate) fn get_url(&self, path: &str) -> String {
        format!("{}{}", self.inner.base_url, path)
    }
}
