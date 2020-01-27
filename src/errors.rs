use std::fmt;
use std::fmt::Display;

use actix_web::{error, HttpResponse};
use failure::{Backtrace, Context, Fail};
use r2d2::Error as RdError;
use diesel::result::Error as DieselResultError;
use serde::Serialize;
use base64::DecodeError;

pub type Result<T> = ::std::result::Result<T, ApplicationError>;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display = "{:?}", _0)]
    BadRequest(String),

    #[fail(display = "application server error")]
    Runtime,
}

#[derive(Debug, Serialize)]
struct ApiErrorResult { message: String }

impl error::ResponseError for ApplicationError {
    fn render_response(&self) -> HttpResponse {
        match self.kind() {
            ErrorKind::BadRequest(_s) => {
                HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(ApiErrorResult { message: "bad request".to_string() })
            }

            _ => {
                HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .json(ApiErrorResult { message: "runtime error".to_string() })
            }
        }
    }
}

impl From<RdError> for ApplicationError {
    fn from(error: RdError) -> ApplicationError {
        error!("Unable to locate a razor: {}, retrying", error);

        ApplicationError {
            inner: error.context(ErrorKind::Runtime),
        }
    }
}

impl From<DieselResultError> for ApplicationError {
    fn from(error: DieselResultError) -> ApplicationError {
        error!("Unable to locate a razor: {}, retrying", error);

        ApplicationError {
            inner: error.context(ErrorKind::Runtime),
        }
    }
}

impl From<reqwest::Error> for ApplicationError {
    fn from(error: reqwest::Error) -> ApplicationError {
        error!("Unable to locate a razor: {}, retrying", error);

        ApplicationError {
            inner: error.context(ErrorKind::Runtime),
        }
    }
}

impl From<DecodeError> for ApplicationError {
    fn from(error: DecodeError) -> ApplicationError {
        error!("Unable to locate a razor: {}, retrying", error);

        ApplicationError {
            inner: error.context(ErrorKind::Runtime),
        }
    }
}

impl From<rusoto_core::RusotoError<rusoto_s3::PutObjectError>> for ApplicationError {
    fn from(error: rusoto_core::RusotoError<rusoto_s3::PutObjectError>) -> ApplicationError {
        error!("Unable to locate a razor: {}, retrying", error);

        ApplicationError {
            inner: error.context(ErrorKind::Runtime),
        }
    }
}


// ----------------------------------------- boilerplate -----------------------------------------

#[derive(Debug)]
pub struct ApplicationError { inner: Context<ErrorKind> }

impl ApplicationError {
    pub fn kind(&self) -> &ErrorKind { self.inner.get_context() }
}

impl Fail for ApplicationError {
    fn cause(&self) -> Option<&dyn Fail> { self.inner.cause() }
    fn backtrace(&self) -> Option<&Backtrace> { self.inner.backtrace() }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { Display::fmt(&self.inner, f) }
}

impl From<ErrorKind> for ApplicationError {
    fn from(kind: ErrorKind) -> Self { ApplicationError { inner: Context::new(kind) } }
}

impl From<Context<ErrorKind>> for ApplicationError {
    fn from(inner: Context<ErrorKind>) -> Self { ApplicationError { inner } }
}
