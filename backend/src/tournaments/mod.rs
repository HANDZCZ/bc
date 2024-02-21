use actix_web::web::{self, ServiceConfig};

mod apply_for;
mod delete;
mod edit;
mod get;
mod get_all;
mod new;
mod with_bracket_trees;
mod playing;
mod signed_up_teams;
mod team_applications;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/tournaments")
            .configure(with_bracket_trees::configure)
            .configure(team_applications::configure)
            .configure(signed_up_teams::configure)
            .configure(playing::configure)
            .service(get_all::get_all)
            .service(get::get)
            .service(new::new)
            .service(edit::edit)
            .service(delete::delete)
            .service(apply_for::apply_for),
    );
}
