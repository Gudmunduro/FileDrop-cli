use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDropResponse {
    pub id: String,
    pub access_token: Option<String>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropFile {
    pub id: i32,
    pub filename: String,
    pub file_size: i64,
    pub upload_date: String,
    pub drop_id: String
}