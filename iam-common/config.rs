use envconfig::Envconfig;
use std::net::SocketAddr;

#[derive(Debug, Envconfig)]
pub struct Config {
    #[envconfig(from = "LISTEN_ADDR", default = "0.0.0.0:3001")]
    pub listen_addr: SocketAddr,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(from = "IAM_SIGNING_KEY_FILE")]
    pub iam_signing_key_file: String,
    #[envconfig(from = "IAM_ISSUER_HOST", default = "localhost")]
    pub issuer_host: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(envconfig::Envconfig::init_from_env()?)
    }
}
