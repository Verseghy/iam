use super::Api;
use anyhow::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod register {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Request<'a> {
        pub name: &'a str,
        pub email: &'a str,
        pub password: &'a str,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub id: String,
    }
}

pub async fn register(api: &Api, req: &register::Request<'_>) -> Result<register::Response> {
    api.request(Method::POST, "/v1/users/register", Some(req))
        .await
}

pub mod login {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Request<'a> {
        pub email: &'a str,
        pub password: &'a str,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub token: String,
    }
}

pub async fn login(api: &Api, req: &login::Request<'_>) -> Result<login::Response> {
    api.request(Method::POST, "/v1/users/login", Some(req))
        .await
}

pub mod get_user {
    use iam_common::user::UserInfo;

    pub type Response = UserInfo;
}

pub async fn get_user(api: &Api, id: &str) -> Result<get_user::Response> {
    api.request(Method::GET, &format!("/v1/users/{id}"), None::<&()>)
        .await
}
