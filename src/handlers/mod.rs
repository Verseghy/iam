mod internal;
mod v1;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(web::scope("/v1").configure(v1::routes))
        .service(web::scope("/internal").configure(internal::routes));
}
