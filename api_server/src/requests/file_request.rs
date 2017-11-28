#[derive(Deserialize)]
pub struct Store {
    pub name: String,
    pub file_name: String,
    pub extension: String,
}

#[derive(Deserialize)]
pub struct Update {
    pub name: String,
    pub extension: String,
}
