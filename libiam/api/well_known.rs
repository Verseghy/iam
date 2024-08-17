use super::Api;
use anyhow::Result;
use reqwest::Method;

pub mod jwks {
    use jsonwebtoken::jwk::JwkSet;

    pub type Response = JwkSet;
}

pub async fn jwks(api: &Api) -> Result<jwks::Response> {
    api.request(Method::GET, "/.well-known/jwks.json", None::<&()>)
        .await
}
