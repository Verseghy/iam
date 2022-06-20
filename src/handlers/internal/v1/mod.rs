mod decision;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config.service(web::resource("/decision").route(web::post().to(decision::decision)));
}
