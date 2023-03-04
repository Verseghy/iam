use std::sync::Arc;

use iam_common::user::UserInfo;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{
    error::{unwrap_res, ErrorMessage, Result},
    utils::{create_client, Either},
    Iam,
};

#[derive(Debug)]
pub struct AppInner {
    secret: String,
    token: String,
    iam: Iam,
    client: Client,
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

        tracing::debug!(secret, "app logging into iam");

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
            inner: Arc::new(AppInner {
                secret: secret.to_owned(),
                client: create_client(&res.token),
                token: res.token,
                iam: iam.clone(),
            }),
        })
    }

    #[inline]
    fn client(&self) -> &Client {
        &self.inner.client
    }

    pub fn token(&self) -> &str {
        &self.inner.token
    }

    pub fn id(&self) -> String {
        let (id, _) = iam_common::app::parse_token(&self.inner.secret).unwrap();
        id
    }

    pub async fn get_user_info(&self, id: &str) -> Result<UserInfo> {
        let res = self
            .client()
            .get(self.inner.iam.get_url(&format!("/v1/users/{}/", id)))
            .send()
            .await?
            .json::<Either<UserInfo, ErrorMessage>>()
            .await?;

        let res = unwrap_res(res)?;

        Ok(res)
    }
}
