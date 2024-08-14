use crate::{
    api::{self, Api},
    Iam,
};
use iam_common::user::UserInfo;
use std::sync::Arc;

#[derive(Debug)]
pub struct AppInner {
    secret: String,
    token: String,
    api: Api,
}

#[derive(Debug, Clone)]
pub struct App {
    inner: Arc<AppInner>,
}

impl App {
    pub async fn login(iam: &Iam, secret: &str) -> anyhow::Result<Self> {
        let token = api::app::login(&iam.inner.api, &api::app::login::Request { token: secret })
            .await?
            .token;

        Ok(Self {
            inner: Arc::new(AppInner {
                secret: secret.to_owned(),
                token: token.clone(),
                api: iam.inner.api.with_token(token),
            }),
        })
    }

    pub fn token(&self) -> &str {
        &self.inner.token
    }

    pub fn id(&self) -> String {
        let (id, _) = iam_common::app::parse_token(&self.inner.secret).unwrap();
        id
    }

    pub async fn get_user_info(&self, id: &str) -> anyhow::Result<UserInfo> {
        let res = api::user::get_user(&self.inner.api, id).await?;
        Ok(res)
    }
}
