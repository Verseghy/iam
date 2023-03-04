use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub fn create_client(token: &str) -> Client {
    let mut header_map = HeaderMap::with_capacity(1);
    header_map.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    Client::builder()
        .default_headers(header_map)
        .build()
        .expect("failed to create reqwest client")
}
