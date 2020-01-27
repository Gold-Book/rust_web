use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Card {
    pub card_id: i64,
    pub name: String,
    pub kana: String,
    pub company_name: String,
    pub department: String,
    pub position: String,
    pub email: String,
    pub cell_phone_number: String,
    pub phone_number: String,
    pub fax: String,
    pub zip_code: String,
    pub address: String,
    pub url: String,
    pub owner: String,
}
