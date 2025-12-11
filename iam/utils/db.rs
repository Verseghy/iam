use sea_orm::{
    DbErr, RuntimeErr,
    sqlx::{self, postgres::PgDatabaseError},
};

fn unwrap_to_sqlx_err(err: &DbErr) -> Option<&sqlx::Error> {
    match err {
        DbErr::Conn(RuntimeErr::SqlxError(error)) => Some(error),
        DbErr::Exec(RuntimeErr::SqlxError(error)) => Some(error),
        DbErr::Query(RuntimeErr::SqlxError(error)) => Some(error),
        _ => None,
    }
}

fn is_code(err: &DbErr, code: &str) -> bool {
    let Some(err) = unwrap_to_sqlx_err(err) else {
        return false;
    };

    let sqlx::Error::Database(err) = err else {
        return false;
    };

    let Some(err) = err.try_downcast_ref::<PgDatabaseError>() else {
        return false;
    };

    err.code() == code
}

pub trait DatabaseErrorType {
    fn is_duplicate_entry(&self) -> bool;
}

impl DatabaseErrorType for DbErr {
    fn is_duplicate_entry(&self) -> bool {
        is_code(self, "23505")
    }
}
