use actix_web::web::{self, ServiceConfig};

mod delete;
mod edit;
mod get;
mod get_all;
mod invites;
mod login;
mod register;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .configure(invites::configure)
            .service(get_all::get_all)
            .service(get::logged_in_user)
            .service(get::get)
            .service(register::register)
            .service(login::login)
            .service(edit::edit)
            .service(delete::delete),
    );
}
