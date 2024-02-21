use actix_web::web::{self, ServiceConfig};

mod register;
mod login;
mod edit;
mod delete;
mod get;
mod get_all;
mod invites;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .configure(invites::configure)
            .service(get_all::get_all)
            .service(get::get)
            .service(register::register)
            .service(login::login)
            .service(edit::edit)
            .service(delete::delete)
    );
}
