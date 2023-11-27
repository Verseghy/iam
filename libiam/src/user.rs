use crate::{
    error::{unwrap_res, ErrorMessage, Result},
    utils::Either,
    Iam,
};
use iam_common::token::Claims;
use iam_common::Id;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Debug)]
pub struct UserInner {
    token: String,
    id: Id,
    _iam: Iam,
}

#[derive(Debug, Clone)]
pub struct User {
    inner: Arc<UserInner>,
}

impl User {
    pub async fn register(iam: &Iam, name: &str, email: &str, password: &str) -> Result<Self> {
        Client::new()
            .post(iam.get_url("/v1/users/register"))
            .json(&json!({
                "name": name,
                "email": email,
                "password": password,
            }))
            .send()
            .await?;

        User::login(iam, email, password).await
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
                token: res.token.clone(),
                id: serde_json::from_str::<Id>(
                    format!(
                        // HACK: impl FromStr
                        "\"{}\"",
                        jsonwebtoken::decode::<Claims>(
                            res.token.as_str(),
                            &DecodingKey::from_secret(&[]),
                            &{
                                let mut v = Validation::new(Algorithm::RS256);
                                v.insecure_disable_signature_validation();
                                v.set_audience(&["https://verseghy-gimnazium.net"]);
                                v
                            },
                        )
                        .unwrap()
                        .claims
                        .subject
                        .as_str()
                    )
                    .as_str(),
                )
                .unwrap(),
                _iam: iam.clone(),
            }),
        })
    }

    pub fn token(&self) -> &str {
        &self.inner.token
    }

    pub fn id(&self) -> &Id {
        &self.inner.id
    }
}
