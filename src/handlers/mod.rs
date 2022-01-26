mod v1;

use actix_web::web::ServiceConfig;

pub fn routes(config: &mut ServiceConfig) {
    config.configure(v1::routes);
}
