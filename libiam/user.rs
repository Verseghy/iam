use crate::{
    api::{self, Api},
    Iam,
};
use iam_common::Id;
use std::{str::FromStr, sync::Arc};

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
        let claims = iam.inner.jwt.get_claims(&token).await?;

        Ok(Self {
            inner: Arc::new(UserInner {
                token: token.clone(),
                id: Id::from_str(&claims.sub)?,
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
