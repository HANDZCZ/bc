use actix_web::web::{self, ServiceConfig};

mod register;
mod login;
mod edit;
mod delete;
mod get;
mod get_all;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_all::get_all)
            .service(get::get)
            .service(register::register)
            .service(login::login)
            .service(edit::edit)
            .service(delete::delete)
    );
}
