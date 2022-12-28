use sea_orm::{DbErr, RuntimeErr};

fn get_error_msg(err: &DbErr) -> String {
    match err {
        DbErr::Exec(RuntimeErr::SqlxError(error)) => error.to_string(),
        _ => unimplemented!(),
    }
}

fn get_code(err: &DbErr) -> Option<u32> {
    let msg = get_error_msg(err);
    let striped_msg = msg.strip_prefix("error returned from database: ")?;
    let number_str = striped_msg.split_terminator(&[':', ' ']).next()?;
    number_str.parse().ok()
}

fn is_code(err: &DbErr, code: u32) -> bool {
    if let Some(c) = get_code(err) {
        c == code
    } else {
        false
    }
}

pub trait DatabaseErrorType {
    fn is_duplicate_entry(&self) -> bool;
}

impl DatabaseErrorType for DbErr {
    fn is_duplicate_entry(&self) -> bool {
        is_code(self, 1062)
    }
}
