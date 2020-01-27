use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};

use crate::errors::Result;
use crate::middlewares::requests::cloud_storage::{CloudStorage};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestMessage { b64_image: String }

pub fn begin(item: web::Json<RequestMessage>) -> Result<HttpResponse> {
    CloudStorage::put(&item.b64_image)?;
    Ok(HttpResponse::Ok().finish())
}
