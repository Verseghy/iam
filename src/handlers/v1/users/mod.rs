mod invite;
mod login;
mod register;

use actix_web::web::{self, ServiceConfig};
use lettre::{AsyncSmtpTransport, Tokio1Executor};

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(login::login)
        .service(register::register)
        .route(
            "/invite",
            web::post().to(invite::invite::<
                redis::aio::ConnectionManager,
                AsyncSmtpTransport<Tokio1Executor>,
            >),
        );
}
