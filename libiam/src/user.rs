use crate::{
    error::{unwrap_res, ErrorMessage, Result},
    utils::Either,
    Iam,
};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserInner {
    token: String,
    _iam: Iam,
}

#[derive(Debug, Clone)]
pub struct User {
    inner: Arc<UserInner>,
}

impl User {
    pub async fn register(iam: &Iam, name: &str, email: &str, password: &str) -> Result<Self> {
        #[derive(Debug, Deserialize)]
        #[allow(unused)]
        struct Response {
            id: String,
        }

        let res = Client::new()
            .post(iam.get_url("/v1/users/register"))
            .json(&json!({
                "name": name,
                "email": email,
                "password": password,
            }))
            .send()
            .await?
            .json::<Either<Response, ErrorMessage>>()
            .await?;

        let _ = unwrap_res(res)?;

        Self::login(iam, email, password).await
    }

    pub async fn login(iam: &Iam, email: &str, password: &str) -> Result<Self> {
        let client = Client::new();

        #[derive(Debug, Deserialize)]
        struct Response {
            token: String,
        }

        let res = client
            .post(iam.get_url("/v1/users/login"))
            .json(&json!({
                "email": email,
                "password": password,
            }))
            .send()
            .await?
            .json::<Either<Response, ErrorMessage>>()
            .await?;

        let res = unwrap_res(res)?;

        Ok(Self {
            inner: Arc::new(UserInner {
                token: res.token,
                _iam: iam.clone(),
            }),
        })
    }

    pub fn token(&self) -> &str {
        &self.inner.token
    }
}
