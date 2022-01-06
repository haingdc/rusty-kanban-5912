#[derive(serde::Deserialize)]
// #[table_name = "boards"]
pub struct CreateBoard {
    pub name: String,
}