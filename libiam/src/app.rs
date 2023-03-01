use std::sync::Arc;

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::{unwrap_res, ErrorMessage, Result},
    utils::Either,
    Iam,
};

#[derive(Debug)]
pub struct AppInner {
    token: String,
}

#[derive(Debug, Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

impl App {
    pub async fn login(iam: &Iam, secret: &str) -> Result<Self> {
        #[derive(Debug, Deserialize)]
        struct Response {
            token: String,
        }

        let res = Client::new()
            .post(iam.get_url("/v1/apps/login"))
            .json(&json!({
                "token": secret,
            }))
            .send()
            .await?
            .json::<Either<Response, ErrorMessage>>()
            .await?;

        let res = unwrap_res(res)?;

        Ok(Self {
            inner: Arc::new(AppInner { token: res.token }),
        })
    }

    pub fn token(&self) -> &str {
        &self.inner.token
    }

    pub fn id(&self) -> String {
        let (id, _) = iam_common::app::parse_token(&self.inner.token).unwrap();
        id
    }
}
