use actix_web::web::{self, ServiceConfig};

mod handle_invite;
mod invite;
mod playing;
mod remove;
mod set_playing;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/players")
            .configure(playing::configure)
            .service(invite::invite)
            .service(handle_invite::handle_invite)
            .service(remove::remove)
            .service(set_playing::set_playing),
    );
}
