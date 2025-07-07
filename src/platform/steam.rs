use std::{ fs, path::Path, process::Command };

use steamlocate::SteamDir;

use crate::{ data::game::Game, platform::{ Platform, PlatformError } };

pub struct SteamPlatform {
    steam_dir: SteamDir,
}

impl SteamPlatform {
    pub fn new() -> Result<Self, PlatformError> {
        let dir = SteamDir::locate().map_err(|_| PlatformError::NotInstalled)?;
        Ok(Self { steam_dir: dir })
    }
}

impl Platform for SteamPlatform {
    fn name(&self) -> &str {
        "Steam"
    }

    fn list_games(&self) -> Result<Vec<Game>, PlatformError> {
        let mut games: Vec<Game> = Vec::new();

        let libraries = self.steam_dir.libraries().map_err(|_| PlatformError::FetchFailed)?;
        for lib in libraries {
            let lib = lib.map_err(|e| PlatformError::FetchFailed)?;
            for app_iter in lib.apps() {
                let app = app_iter.unwrap();
                let executable_path = app.launcher_path
                    .as_ref()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                let game = Game {
                    name: app.name.clone().unwrap_or_else(|| app.install_dir.clone()),
                    app_id: app.app_id.to_string(),
                    executable_path,
                    platform: "Steam".to_string(),
                    icon_path: None,
                };

                games.push(game);
            }
        }

        Ok(games)
    }

    fn launch_game(&self, app_id: &str) -> Result<(), PlatformError> {
        let steam_path = self.steam_dir.path();

        let steam_exe = steam_path.join("steam.exe");
        if !steam_exe.exists() {
            return Err(PlatformError::NotInstalled);
        }

        Command::new(steam_exe)
            .arg(format!("steam://rungameid/{}", app_id))
            .spawn()
            .map_err(|e| PlatformError::LaunchFailed)?;

        Ok(())
    }
}
