#[derive(Debug, Clone)]
pub struct Game {
    pub name: String,
    pub app_id: String,
    pub executable_path: String,
    pub platform: String,
    pub icon_path: Option<String>,
}
