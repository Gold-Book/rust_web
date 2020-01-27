use actix_web::dev::{Transform, Service, ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::{HttpMessage};
use futures::{future, prelude::Future};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthenticationInfrastructure;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AuthenticationInfrastructure
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationInfrastructureMiddleware<S, B>;
    type Future = future::FutureResult<Self::Transform, Self::InitError>;

    // New Middlware Instance
    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(AuthenticationInfrastructureMiddleware { service })
    }
}

// Middleware Instance.
// In here, you write actual process.
pub struct AuthenticationInfrastructureMiddleware<S, B>
    where
    // This is not necessary, but make it easier to understand.
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
{
    service: S,
}

impl<S, B> Service for AuthenticationInfrastructureMiddleware<S, B>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item=Self::Response, Error=Self::Error>>;

    fn poll_ready(&mut self) -> Result<futures::Async<()>, Self::Error> {
        Ok(futures::Async::Ready(()))
    }

    fn call(&mut self, service_request: Self::Request) -> Self::Future {

        println!("Extensions Stract \"User\" is Mock Data");

        service_request.extensions_mut().insert(
            User {
                user_id: "8ed043ac-b6de-467c-b7d5-379913bfa31a".to_string(),
            }
        );

        Box::new(self.service.call(service_request).map(|res| {
            res
        }))
    }
}

pub struct User {
    pub user_id: String,
}

