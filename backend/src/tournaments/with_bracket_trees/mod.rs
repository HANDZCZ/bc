use actix_web::web::{ServiceConfig, self};

mod get_all;
mod get;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/with_bracket_trees")
            .service(get_all::get_all)
            .service(get::get)
    );
}