use hyper::{Request, header::HeaderValue};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::{Uuid, fmt::Simple};

#[derive(Copy, Clone, Debug)]
pub struct MakeUuidRequestId;

impl MakeRequestId for MakeUuidRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let mut buf = [0u8; Simple::LENGTH];
        let id = Uuid::new_v4().as_simple().encode_lower(&mut buf);
        Some(RequestId::new(HeaderValue::from_str(id).unwrap()))
    }
}
