use actix_web::web::{self, ServiceConfig};

mod invite;
mod remove;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/managers")
            .service(invite::invite)
            .service(remove::remove),
    );
}