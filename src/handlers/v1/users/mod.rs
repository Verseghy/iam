mod invite;
mod login;

use crate::auth::permission::permissions;
use actix_web::web::{self, ServiceConfig};
use lettre::{AsyncSmtpTransport, Tokio1Executor};

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(login::login)
        .service(
            web::resource("/invite")
                .route(web::post().to(invite::invite::<
                    redis::aio::ConnectionManager,
                    AsyncSmtpTransport<Tokio1Executor>,
                    rand::rngs::SmallRng,
                >))
                .wrap(permissions!("iam.user.invite")),
        );
}
