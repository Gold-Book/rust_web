use actix_web::web;

use crate::apis::card::{begin};
use crate::apis::cards::search;
use crate::apis::examples::{bad_request, connect_db, index, runtime};

mod examples;
mod card;
mod cards;

pub fn endpoint_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/card", web::post().to(begin))
        .route("/cards", web::get().to(search));
}

pub fn example_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/examples")
            .route("", web::get().to(index))
            .route("/connect", web::get().to(connect_db))
            .service(web::scope("/errors")
                .route("/runtime", web::get().to(runtime))
                .route("/bad", web::get().to(bad_request))
            )
        )
    ;
}
