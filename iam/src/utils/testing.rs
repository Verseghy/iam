use axum::body::HttpBody;
use serde_json::Value;

pub async fn body_to_json<B>(body: B) -> Value
where
    B: HttpBody,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
