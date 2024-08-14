use super::Api;
use anyhow::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub mod login {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Request<'a> {
        pub token: &'a str,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub token: String,
    }
}

pub async fn login(api: &Api, req: &login::Request<'_>) -> Result<login::Response> {
    api.request(Method::POST, "/v1/apps/login", Some(req)).await
}
