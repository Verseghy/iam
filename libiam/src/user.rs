use crate::{
    api::{self, Api},
    Iam,
};
use iam_common::token::Claims;
use iam_common::Id;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use std::sync::Arc;

#[derive(Debug)]
pub struct UserInner {
    token: String,
    id: Id,
    _api: Api,
}

#[derive(Debug, Clone)]
pub struct User {
    inner: Arc<UserInner>,
}

impl User {
    pub async fn register(
        iam: &Iam,
        name: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<Self> {
        api::user::register(
            &iam.inner.api,
            &api::user::register::Request {
                name,
                email,
                password,
            },
        )
        .await?;

        Self::login(iam, email, password).await
    }

    pub async fn login(iam: &Iam, email: &str, password: &str) -> anyhow::Result<Self> {
        let token = api::user::login(
            &iam.inner.api,
            &api::user::login::Request { email, password },
        )
        .await?
        .token;

        let api = iam.inner.api.with_token(token.clone());

        Ok(Self {
            inner: Arc::new(UserInner {
                token: token.clone(),
                id: serde_json::from_str::<Id>(
                    format!(
                        // HACK: impl FromStr
                        "\"{}\"",
                        jsonwebtoken::decode::<Claims>(
                            token.as_str(),
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
                _api: api,
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
