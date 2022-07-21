mod actions;
mod users;
//
// use actix_web::web;
// use actix_web::web::ServiceConfig;
//
// pub fn routes(config: &mut ServiceConfig) {
//     config
//         .service(web::scope("/users").configure(users::routes))
//         .configure(actions::routes);
// }

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(actions::routes())
        .nest("/users", users::routes())
    // .nest("/action", actions::action_routes())
    // .nest("/actions", actions::actions_routes())
}
