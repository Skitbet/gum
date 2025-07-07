use crate::{ data::game::Game, platform::steam::SteamPlatform };

pub mod steam;

pub enum PlatformError {
    NotInstalled,
    LaunchFailed,
    FetchFailed,
}

pub trait Platform: Send + Sync {
    fn name(&self) -> &str;
    fn list_games(&self) -> Result<Vec<Game>, PlatformError>;
    fn launch_game(&self, app_id: &str) -> Result<(), PlatformError>;
}

pub fn load_platforms() -> Vec<Box<dyn Platform>> {
    let mut platforms: Vec<Box<dyn Platform>> = Vec::new();

    if let Ok(steam) = SteamPlatform::new() {
        platforms.push(Box::new(steam));
    }

    platforms
}
