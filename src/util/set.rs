use sea_orm::{ActiveValue, Value};

pub fn set_option<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<Value>,
{
    match value {
        Some(v) => ActiveValue::Set(v),
        None => ActiveValue::NotSet,
    }
}
