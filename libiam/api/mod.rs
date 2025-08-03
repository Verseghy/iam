pub mod app;
pub mod user;
pub mod well_known;

use reqwest::{header::AUTHORIZATION, Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct Api {
    client: Client,
    base: Url,
    handle: Handle,
    token: Option<String>,
}

impl Api {
    pub fn new(base: &str, token: Option<String>) -> anyhow::Result<Self> {
        Ok(Self {
            client: Client::new(),
            base: Url::parse(base)?,
            handle: Handle::current(),
            token,
        })
    }

    pub fn with_token(&self, token: String) -> Self {
        Self {
            client: self.client.clone(),
            base: self.base.clone(),
            handle: self.handle.clone(),
            token: Some(token),
        }
    }

    #[inline]
    async fn request<Req, Res>(
        &self,
        method: Method,
        path: &str,
        req: Option<&Req>,
    ) -> anyhow::Result<Res>
    where
        Req: Serialize,
        Res: DeserializeOwned + Send + 'static,
    {
        let mut r = self.client.request(method, self.base.join(path).unwrap());

        if let Some(token) = &self.token {
            r = r.header(AUTHORIZATION, &format!("Bearer {token}"));
        }

        if let Some(req) = req {
            r = r.json(req)
        }

        self.handle
            .spawn(async move {
                let res: ErrorOr<Res> = r.send().await?.json().await?;

                match res {
                    ErrorOr::Error(error) => {
                        tracing::error!("iam error: {error:?}");
                        Err(error.into())
                    }
                    ErrorOr::Data(res) => anyhow::Ok(res),
                }
            })
            .await
            .unwrap()
    }
}

#[derive(Debug, Deserialize, Error)]
#[error("{error}")]
pub struct Error {
    pub code: String,
    pub error: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ErrorOr<T> {
    Error(Error),
    Data(T),
}
