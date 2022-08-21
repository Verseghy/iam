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

macro_rules! json_body {
    ($($tt:tt)*) => {
        ::axum::body::Body::from(
            ::serde_json::to_vec(
                &::serde_json::json!($($tt)*)
            ).unwrap(),
        )
    }
}

pub(crate) use json_body;
