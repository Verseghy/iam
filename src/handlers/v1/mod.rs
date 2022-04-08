mod add_action;
mod get_action;
mod users;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(add_action::add_action)
        .service(get_action::get_action)
        .service(web::scope("/users").configure(users::routes));
}
