mod login;
mod register;
mod invite;

use actix_web::web::{service, ServiceConfig};

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(login::login)
        .service(register::register)
        .service(invite::invite);
}
