use crate::api::{self, Api};
pub use iam_common::keys::jwt::Claims;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use tokio::sync::RwLock;

pub struct Jwt {
    keys: RwLock<Vec<DecodingKey>>,
    api: Api,
}

impl Jwt {
    pub async fn new(api: &Api) -> anyhow::Result<Self> {
        let keys = query_keys(api).await?;

        Ok(Self {
            keys: RwLock::new(keys),
            api: api.clone(),
        })
    }

    pub async fn get_claims(&self, token: &str) -> anyhow::Result<Claims> {
        if let Ok(claims) = self.try_decode_all(token).await {
            return Ok(claims);
        }

        self.refresh_keys().await?;
        self.try_decode_all(token).await
    }

    async fn try_decode_all(&self, token: &str) -> anyhow::Result<Claims> {
        for key in &*self.keys.read().await {
            if let Ok(claims) = self.decode(key, token) {
                return Ok(claims);
            }
        }

        anyhow::bail!("Could not decode token");
    }

    fn decode(&self, key: &DecodingKey, token: &str) -> anyhow::Result<Claims> {
        // TODO: set audience
        let validation = Validation::new(Algorithm::ES256);

        jsonwebtoken::decode(token, key, &validation)
            .map(|decoded| decoded.claims)
            .inspect_err(|err| tracing::warn!(token, error = err.to_string(), "decoding error"))
            .map_err(|err| err.into())
    }

    async fn refresh_keys(&self) -> anyhow::Result<()> {
        tracing::info!("Refreshing IAM keys");

        let mut keys = self.keys.write().await;
        *keys = query_keys(&self.api).await?;

        Ok(())
    }
}

async fn query_keys(api: &Api) -> anyhow::Result<Vec<DecodingKey>> {
    let keys: Vec<DecodingKey> = api::well_known::jwks(api)
        .await?
        .keys
        .iter()
        .filter_map(|jwk| DecodingKey::from_jwk(jwk).ok())
        .collect();

    if keys.is_empty() {
        tracing::warn!("IAM jwks is empty");
    }

    Ok(keys)
}
