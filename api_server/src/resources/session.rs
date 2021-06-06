#[derive(Serialize)]
pub struct Login {
    pub success: bool,
    pub user_id: Option<i32>,
    pub token: Option<String>,
    pub message: String,
}
