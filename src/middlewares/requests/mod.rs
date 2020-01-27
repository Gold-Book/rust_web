extern crate reqwest;

use reqwest::Client;

use crate::errors::Result;

pub mod cloud_storage;
pub mod authentication_infrastructure;

fn request<T>(req: fn(&Client) -> Result<T>) -> Result<T> {
    lazy_static! { static ref HTTP_CLIENT: Client = Client::new(); }
    req(&*HTTP_CLIENT) // TODO: Futures 0.3 での非同期化
}
