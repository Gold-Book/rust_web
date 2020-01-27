extern crate diesel;

use actix_web::{HttpResponse, Responder};
use serde::Serialize;

use crate::errors::ApplicationError;
use crate::errors::ErrorKind::{BadRequest, Runtime};
use crate::errors::Result;
use crate::middlewares::db_pool::pool_con;


#[derive(Debug, Serialize)]
struct ResultMessage { message: String }


pub fn index() -> impl Responder {
    HttpResponse::Ok().json(ResultMessage { message: "hello world actex web json".to_string() })
}

pub fn connect_db() -> Result<HttpResponse> {
    use crate::models::models::Card;
    use diesel::RunQueryDsl;
    use diesel::QueryDsl;
    use crate::models::schema::cards::dsl::*;

    assert!(pool_con().is_ok());

    let connection = pool_con()?;
    let cards_result = cards.limit(10).load::<Card>(&*connection)?;

    Ok(HttpResponse::Ok().json(cards_result))
}


pub fn runtime() -> Result<HttpResponse> { Err(ApplicationError::from(Runtime)) }

pub fn bad_request() -> Result<HttpResponse> { Err(ApplicationError::from(BadRequest("bad request error".to_string()))) }
