mod invite;
mod login;
mod register;

use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(login::login)
        .service(register::register)
        .service(invite::invite);
}
