mod get;
mod put;
mod post;
mod delete;
mod gets;

use actix_web::web::{self, ServiceConfig};
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use crate::auth::permission::permissions;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(
            web::resource("/action")
                .route(web::get().to(get::get).wrap(permissions!["iam.action.get"]))
                .route(
                    web::post()
                        .to(post::post)
                        .wrap(permissions!["iam.action.update"]),
                )
                .route(web::put().to(put::put).wrap(permissions!["iam.action.add"]))
                .route(
                    web::delete()
                        .to(delete::delete)
                        .wrap(permissions!["iam.action.delete"]),
                ),
        )
        .service(
            web::resource("/actions")
                .route(web::get().to(gets::gets))
                .wrap(permissions!["iam.action.list"]),
        );
}
