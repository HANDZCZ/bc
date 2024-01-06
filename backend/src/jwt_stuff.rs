use actix_web_grants::authorities::AuthDetails;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    cell::RefCell,
    collections::HashSet,
    future::{ready, Ready},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use actix_web::{
    dev::{forward_ready, Payload, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{self, HeaderValue},
        StatusCode,
    },
    web::Data,
    Error, FromRequest, HttpMessage, HttpRequest,
};
use futures_util::future::LocalBoxFuture;

use crate::common::{self, JsonError};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: Uuid,
    #[serde(skip)]
    pub roles: HashSet<String>,
}

impl UserData {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            roles: HashSet::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: u64,
    iat: u64,
    nbf: u64,
    data: UserData,
}

pub struct JwtMiddleware {
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
    token_ttl: u64,
    validation: Validation,
}

impl JwtMiddleware {
    pub fn new(secret: String, token_ttl: u64) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            token_ttl,
            validation: Validation::default(),
        }
    }
}

impl<S: 'static, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareInner {
            service: Rc::new(service),
            decoding_key: self.decoding_key.clone(),
            encoding_key: self.encoding_key.clone(),
            token_ttl: self.token_ttl,
            validation: self.validation.clone(),
        }))
    }
}

pub struct JwtMiddlewareInner<S> {
    service: Rc<S>,
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
    token_ttl: u64,
    validation: Validation,
}

fn decode_jwt(
    header_value: Option<&HeaderValue>,
    decoding_key: &DecodingKey,
    validation: &Validation,
) -> Option<Claims> {
    let Some(val) = header_value else {
        return None;
    };
    let Ok(val) = val.to_str() else {
        return None;
    };
    if !val.starts_with("Bearer ") {
        return None;
    }
    match jsonwebtoken::decode::<Claims>(&val[7..], decoding_key, validation) {
        Ok(data) => Some(data.claims),
        Err(_) => None,
    }
}

fn encode_jwt(user_data: UserData, encoding_key: &EncodingKey, token_ttl: u64) -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let claims = Claims {
        exp: now.as_secs() + token_ttl,
        iat: now.as_secs(),
        nbf: now.as_secs(),
        data: user_data,
    };
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, encoding_key).unwrap()
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        let auth_header_value = req.headers().get(header::AUTHORIZATION).map(|v| v.clone());
        let mut claims = decode_jwt(
            auth_header_value.as_ref(),
            &self.decoding_key,
            &self.validation,
        );

        let encoding_key = self.encoding_key.clone();
        let token_ttl = self.token_ttl;

        Box::pin(async move {
            if claims.is_some() {
                let pool = req
                    .app_data::<Data<PgPool>>()
                    .expect("Failed to find database pool");
                if let Ok(row) = sqlx::query!(
                    r#"select roles "roles!" from users_and_roles where id = $1"#,
                    claims.as_ref().unwrap().data.id
                )
                .fetch_one(pool.get_ref())
                .await
                {
                    let roles: HashSet<String> = serde_json::from_value(row.roles).unwrap();
                    claims.as_mut().unwrap().data.roles = roles;
                }
            }

            let ext = Rc::new(RefCell::new(AuthDataInner {
                changed: false,
                data: claims.map(|c| c.data),
            }));

            req.extensions_mut().insert(ext.clone());

            let mut res = svc.call(req).await?;

            let inner_data = ext.borrow();
            if inner_data.changed {
                if let Some(user_data) = inner_data.data.as_ref() {
                    let token = encode_jwt(user_data.to_owned(), &encoding_key, token_ttl);
                    res.headers_mut().insert(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
                    );
                } else {
                    res.headers_mut().remove(header::AUTHORIZATION);
                }
            } else if let Some(val) = auth_header_value {
                res.headers_mut().insert(header::AUTHORIZATION, val);
            }

            Ok(res)
        })
    }
}

pub struct AuthDataInner {
    changed: bool,
    data: Option<UserData>,
}

impl AuthDataInner {
    pub fn get_data(&self) -> Option<&UserData> {
        self.data.as_ref()
    }
    pub fn get_data_mut(&mut self) -> &mut Option<UserData> {
        self.changed = true;
        &mut self.data
    }
}

pub struct AuthData {
    inner: Rc<RefCell<AuthDataInner>>,
}

impl<'a> AuthData {
    pub fn into_inner(self) -> Rc<RefCell<AuthDataInner>> {
        self.inner
    }
}

impl FromRequest for AuthData {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(AuthData {
            inner: req
                .extensions()
                .get::<Rc<RefCell<AuthDataInner>>>()
                .expect("You most likely forgot to add JwtMiddleware")
                .to_owned(),
        }))
    }
}

pub async fn extract(req: &ServiceRequest) -> Result<HashSet<String>, Error> {
    let extensions = req.extensions();
    let auth_data = extensions
        .get::<Rc<RefCell<AuthDataInner>>>()
        .expect("You most likely forgot to add JwtMiddleware");
    let borrow = auth_data.borrow();
    let data = borrow.data.as_ref();
    let mut set = data
        .map(|data| {
            data.roles
                .clone()
                .into_iter()
                .map(|e| format!("role::{e}"))
                .collect()
        })
        .or(Some(HashSet::new()))
        .unwrap();
    if data.is_some() {
        set.insert("state::LoggedIn".to_owned());
    } else {
        set.insert("state::LoggedOut".to_owned());
    }
    Ok(set)
}

pub struct LoggedInUser {
    pub id: Uuid,
}

impl FromRequest for LoggedInUser {
    type Error = JsonError;
    type Future = Ready<Result<Self, JsonError>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let auth_data = extensions
            .get::<Rc<RefCell<AuthDataInner>>>()
            .expect("You most likely forgot to add JwtMiddleware");
        let borrow = auth_data.borrow();
        let data = borrow.data.as_ref();

        let res = if let Some(user_data) = data {
            Ok(LoggedInUser { id: user_data.id })
        } else {
            let err = common::Error::new("not logged in");
            Err(JsonError::new(err, StatusCode::UNAUTHORIZED))
        };

        ready(res)
    }
}

pub struct LoggedInUserWithAuthorities {
    pub id: Uuid,
    pub authorities: AuthDetails,
}

impl FromRequest for LoggedInUserWithAuthorities {
    type Error = JsonError;
    type Future = Ready<Result<Self, JsonError>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let extensions = req.extensions();
        let auth_data = extensions
            .get::<Rc<RefCell<AuthDataInner>>>()
            .expect("You most likely forgot to add JwtMiddleware");
        let borrow = auth_data.borrow();
        let data = borrow.data.as_ref();

        let res = if let Some(user_data) = data {
            let authorities = extensions.get::<AuthDetails>().unwrap().clone();
            Ok(LoggedInUserWithAuthorities {
                id: user_data.id,
                authorities,
            })
        } else {
            let err = common::Error::new("not logged in");
            Err(JsonError::new(err, StatusCode::UNAUTHORIZED))
        };

        ready(res)
    }
}
