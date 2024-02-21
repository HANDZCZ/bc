use actix_web::web::{self, ServiceConfig};

mod handle_invite;
mod invite;
mod remove;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/managers")
            .service(invite::invite)
            .service(handle_invite::handle_invite)
            .service(remove::remove),
    );
}
