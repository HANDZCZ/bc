use actix_web::web::{self, ServiceConfig};

mod invite;
mod remove;
mod handle_invite;
mod set_playing;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/players")
            .service(invite::invite)
            .service(handle_invite::handle_invite)
            .service(remove::remove)
            .service(set_playing::set_playing)
    );
}