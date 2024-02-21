use actix_web::web::{self, ServiceConfig};

mod delete;
mod edit;
mod get;
mod get_all;
mod leave_tournament;
mod managers;
mod new;
mod players;
mod with_players_and_managers;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/teams")
            .configure(with_players_and_managers::configure)
            .configure(players::configure)
            .configure(managers::configure)
            .service(get_all::get_all)
            .service(get::get)
            .service(new::new)
            .service(edit::edit)
            .service(delete::delete)
            .service(leave_tournament::leave_tournament),
    );
}
