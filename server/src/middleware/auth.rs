use actix_web::{
    body::EitherBody,
    cookie::Cookie,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use nanoid::nanoid;
use sha2::Sha256;
use std::{
    collections::BTreeMap,
    future::{ready, Ready},
};

pub struct Authenticate;

impl<S, B> Transform<S, ServiceRequest> for Authenticate
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticateMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticateMiddleware { service }))
    }
}
pub struct AuthenticateMiddleware<S> {
    service: S,
}
pub struct User {
    pub username: String,
    need_refresh: bool,
}
impl Default for User {
    fn default() -> Self {
        Self {
            username: format!("Guest_{}", nanoid!()),
            need_refresh: true,
        }
    }
}

impl<S, B> Service<ServiceRequest> for AuthenticateMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

        let user = match parse_cookie(&request, &key) {
            ParsedCookie::User(user) => user.unwrap_or_default(),
            ParsedCookie::InvalidToken => {
                return Box::pin(async {
                    Ok(request
                        .into_response(HttpResponse::Unauthorized().finish())
                        .map_into_right_body())
                })
            }
        };

        let res = self.service.call(request);
        Box::pin(async move {
            res.await
                .map(|mut res| {
                    if user.need_refresh {
                        let mut claims = BTreeMap::new();
                        claims.insert("username", user.username);
                        let token_str = claims.sign_with_key(&key).unwrap();
                        let mut cookie = Cookie::new("token", token_str);
                        cookie.set_path("/");
                        res.response_mut().add_cookie(&cookie).unwrap();
                    }
                    return res;
                })
                .map(ServiceResponse::map_into_left_body)
        })
    }
}

enum ParsedCookie {
    User(Option<User>),
    InvalidToken,
}
fn parse_cookie(request: &ServiceRequest, key: &Hmac<Sha256>) -> ParsedCookie {
    match request.cookie("token") {
        None => ParsedCookie::User(None),
        Some(token_cookie) => match VerifyWithKey::<BTreeMap<String, String>>::verify_with_key(
            token_cookie.value(),
            key,
        ) {
            Err(_) => ParsedCookie::InvalidToken,
            Ok(claims) => match claims.get("username") {
                Some(username) => ParsedCookie::User(Some(User {
                    username: username.clone(),
                    need_refresh: false,
                })),
                None => ParsedCookie::User(None),
            },
        },
    }
}
