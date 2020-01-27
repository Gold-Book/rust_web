use actix_web::{HttpRequest, Responder};
use serde::Serialize;
use crate::middlewares::requests::authentication_infrastructure::{User};

#[derive(Debug, Serialize)]
struct Cards {
    id: u64,
    name: String,
    name_kana: String,
    office_name: String,
    department_name: String,
}

#[derive(Debug, Serialize)]
struct ResultCards {
    count_all: usize,
    card_list: Vec<Cards>,
}

pub fn search(_req: HttpRequest) -> impl Responder {
    let mut cards: Vec<Cards> = Vec::new();

    let ext = _req.extensions();
    let user = ext.get::<User>().unwrap();
    println!("user_id: {:?}", user.user_id);

    for x in 1..10 {
        let card = Cards {
            id: x,
            name: "Test".to_string(),
            name_kana: "テスト".to_string(),
            office_name: "test111".to_string(),
            department_name: "test222".to_string(),
        };

        cards.push(card);
    }

    let result = ResultCards {
        count_all: cards.len(),
        card_list: cards,
    };

    serde_json::to_string(&result).unwrap()
}
