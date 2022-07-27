use uuid::{
    v1::{Context, Timestamp},
    Uuid,
};

static CONTEXT: Context = Context::new(123);

fn create_id() -> String {
    let date = chrono::Utc::now();
    let timestamp = Timestamp::from_unix(
        &CONTEXT,
        date.timestamp() as u64,
        date.timestamp_subsec_nanos(),
    );
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "dev".to_string());

    let mut buf = [0u8; 6];
    let mut buf_iter = buf.iter_mut();
    let mut iter = hostname.as_bytes().iter();

    while let Some(x) = iter.next_back() {
        if let Some(y) = buf_iter.next() {
            *y = *x;
        } else {
            break;
        }
    }

    Uuid::new_v1(timestamp, &buf)
        .as_hyphenated()
        .encode_lower(&mut Uuid::encode_buffer())
        .to_owned()
}

#[inline]
pub fn create_action_id() -> String {
    format!("ActionID-{}", create_id())
}

#[inline]
pub fn create_group_id() -> String {
    format!("GroupID-{}", create_id())
}

#[inline]
pub fn create_user_id() -> String {
    format!("UserID-{}", create_id())
}
