use actix_web::web::{self, ServiceConfig};

mod get;
mod get_all;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/signed_up_teams")
            .service(get_all::get_all)
            .service(get::get),
    );
}
