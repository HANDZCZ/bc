use paste::paste;

/*macro_rules! build_resp_inner {
    ($name:ident ,$path:ident) => {
        #[macro_export]
        macro_rules! $name {
            () => {
                actix_web::HttpResponse::$path().finish()
            };
            ($message:expr) => {
                actix_web::HttpResponse::$path().body($message)
            };
        }
    };
}*/

macro_rules! build_resp {
    ($name:ident, $path:ident) => {
        paste! {
            /*build_resp_inner!([<$name _macro>], $path);
            #[allow(unused_imports)]
            pub use [<$name _macro>] as $name;*/
            #[macro_export]
            macro_rules! [<$name _json_macro>] {
                () => {
                    actix_web::HttpResponse::$path().insert_header((actix_web::http::header::ContentType::json())).body("{}")
                };
                ($message:expr) => {
                    actix_web::HttpResponse::$path().json($message)
                };
            }
            #[allow(unused_imports)]
            pub use [<$name _json_macro>] as [<$name _json>];
        }
    };
}

build_resp!(resp_500_IntSerErr, InternalServerError);
build_resp!(resp_400_BadReq, BadRequest);
build_resp!(resp_200_Ok, Ok);
build_resp!(resp_401_Unauth, Unauthorized);
build_resp!(resp_403_Forbidden, Forbidden);

#[macro_export]
macro_rules! begin_transaction_macro {
    ($pool:ident, $tx_name:ident) => {
        let Ok(mut $tx_name) = $pool.get_ref().begin().await else {
            return resp_500_IntSerErr_json!();
        };

        paste::paste! {
            macro_rules! [<rollback_ $tx_name>] {
                () => {
                  if $tx_name.rollback().await.is_err() {
                        return resp_500_IntSerErr_json!();
                    }
                };
            }

            macro_rules! [<commit_ $tx_name>]  {
                () => {
                    if $tx_name.commit().await.is_err() {
                        return resp_500_IntSerErr_json!();
                    };
                }
            }
        }
    };
}
pub use begin_transaction_macro as begin_transaction;

#[macro_export]
macro_rules! check_user_authority_macro {
    ($user:ident, $a:expr) => {
        use actix_web_grants::authorities::AuthoritiesCheck;
        if !$user.authorities.has_authority($a) {
            let err = crate::common::Error::new(format!("user is missing \"{}\" authority", $a));
            return crate::macros::resp_401_Unauth_json!(err);
        }
    };
}

pub use check_user_authority_macro as check_user_authority;