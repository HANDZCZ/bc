use actix_web::web::{self, ServiceConfig};

mod register;
mod login;
mod edit;
mod delete;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register::register)
            .service(login::login)
            .service(edit::edit)
            .service(delete::delete)
    );
}
