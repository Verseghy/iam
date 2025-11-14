use axum::body::{Body, to_bytes};
use serde_json::Value;

pub async fn body_to_json(body: Body) -> Value {
    let bytes = to_bytes(body, 1024 * 1024 * 8).await.unwrap();
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

macro_rules! assert_error {
    ($res:expr, $error:expr) => {
        assert_eq!(($res).status(), ($error).status());

        let res_json = $crate::utils::testing::body_to_json(($res).into_body()).await;
        assert_eq!(res_json["code"], ($error).code());
    };
}

pub(crate) use assert_error;
