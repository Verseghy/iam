mod actions;
mod add_action;
mod get_action;
mod users;

use actix_web::web;
use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(web::scope("/users").configure(users::routes))
        .configure(actions::routes);
}
