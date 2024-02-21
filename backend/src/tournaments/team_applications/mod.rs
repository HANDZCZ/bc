use actix_web::web::{self, ServiceConfig};

mod get;
mod get_all;
mod handle_application;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/team_applications")
            .service(handle_application::handle_application)
            .service(get_all::get_all)
            .service(get::get),
    );
}
