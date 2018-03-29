#[derive(Serialize)]
pub struct Folder {
    pub folder_id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub user_id: i32,
}
