use actix_web::web::{self, ServiceConfig};

mod register;
mod login;

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register::register)
            .service(login::login)
    );
}
