#[derive(Serialize)]
pub struct Index {
    pub success: bool,
    pub message: String,
    pub users: Option<Vec<Show>>,
}

#[derive(Serialize)]
pub struct Show {
    pub success: bool,
    pub message: String,
    pub user_id: Option<i32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}
// 
// #[derive(Serialize)]
// pub struct Store {
//     pub success: bool,
//     pub message: String,
//     pub user_id: Option<i32>,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub email: Option<String>,
// }
//
// #[derive(Serialize)]
// pub struct Update {
//     pub success: bool,
//     pub message: String,
//     pub user_id: Option<i32>,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub email: Option<String>,
// }
//
// #[derive(Serialize)]
// pub struct Delete {
//     pub success: bool,
//     pub message: String,
// }
