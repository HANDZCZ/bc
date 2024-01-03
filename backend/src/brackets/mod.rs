use actix_web::web::{self, ServiceConfig};

mod delete;
mod edit;
mod get;
mod get_all;
mod new;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/brackets")
            .service(get_all::get_all)
            .service(get::get)
            .service(new::new)
            .service(edit::edit)
            .service(delete::delete),
    );
}
