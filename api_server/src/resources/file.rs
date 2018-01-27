use ::chrono::*;

#[derive(Serialize)]
pub struct File {
    pub file_id: i32,
    pub folder_id: i32,
    pub name: String,
    pub extension: String,
    pub created_at: NaiveDateTime,
}
