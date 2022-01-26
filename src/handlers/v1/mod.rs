mod add_action;
mod get_action;

use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config.service(add_action::add_action).service(get_action::get_action);
}
