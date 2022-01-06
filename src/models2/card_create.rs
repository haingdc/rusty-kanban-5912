#[derive(serde::Deserialize/* , diesel::Insertable */)]
#[serde(rename_all = "camelCase")]
// #[table_name = "cards"]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}