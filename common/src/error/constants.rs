use super::const_error;

const_error! {
    #[error("I000", INTERNAL_SERVER_ERROR)]
    #[message("internal server error")]
    const INTERNAL;
}
const_error! {
    #[error("I001", INTERNAL_SERVER_ERROR)]
    #[message("failed to hash the password")]
    const FAILED_PASSWORD_HASH;
}
const_error! {
    #[error("I002", BAD_REQUEST)]
    #[message("invalid jwt token")]
    const JWT_INVALID_TOKEN;
}
const_error! {
    #[error("I003", BAD_REQUEST)]
    #[message("invalid token")]
    const APP_INVALID_TOKEN;
}
const_error! {
    #[error("I004", UNPROCESSABLE_ENTITY)]
    #[message("user not found")]
    const USER_NOT_FOUND;
}
const_error! {
    #[error("I005", INTERNAL_SERVER_ERROR)]
    #[message("database error")]
    const DATABASE_ERROR;
}
const_error! {
    #[error("I006", BAD_REQUEST)]
    #[message("invalid email or password")]
    const INVALID_EMAIL_OR_PASSWORD;
}
const_error! {
    #[error("I007", BAD_REQUEST)]
    #[message("this email is already registered")]
    const EMAIL_ALREADY_REGISTERED;
}
const_error! {
    #[error("I008", FORBIDDEN)]
    #[message("not enough permission")]
    const NO_PERMISSION;
}
const_error! {
    #[error("I009", UNPROCESSABLE_ENTITY)]
    #[message("action not found")]
    const ACTION_NOT_FOUND;
}
const_error! {
    #[error("I010", UNPROCESSABLE_ENTITY)]
    #[message("group not found")]
    const GROUP_NOT_FOUND;
}
const_error! {
    #[error("I011", UNAUTHORIZED)]
    #[message("missing or invalid authorization header")]
    const INVALID_AUTH_HEADER;
}
const_error! {
    #[error("I012", BAD_REQUEST)]
    #[message("no action or group")]
    const ASSIGN_NO_ACTION_OR_GROUP;
}
const_error! {
    #[error("I013", BAD_REQUEST)]
    #[message("cannot set action and group at the same time")]
    const ASSIGN_ACTION_AND_GROUP_SAME_TIME;
}
const_error! {
    #[error("I014", UNPROCESSABLE_ENTITY)]
    #[message("missing fields")]
    const JSON_MISSING_FIELDS;
}
const_error! {
    #[error("I015", BAD_REQUEST)]
    #[message("syntax error")]
    const JSON_SYNTAX_ERROR;
}
const_error! {
    #[error("I016", BAD_REQUEST)]
    #[message("missing or wrong content-type")]
    const JSON_CONTENT_TYPE;
}
const_error! {
    #[error("I017", BAD_REQUEST)]
    #[message("invalid data")]
    const JSON_VALIDATE_INVALID;
}
