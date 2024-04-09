use std::path::PathBuf;

pub struct SteamUser {
    pub user: String,
    pub config_path: PathBuf,
}

pub struct SteamConfig {
    pub config_path: PathBuf,
    pub users: Vec<SteamUser>,
}

fn get_users(steam_dir: &PathBuf) -> Vec<SteamUser> {
    let mut users: Vec<SteamUser> = Vec::new();

    let mut user_dir = steam_dir.clone();
    user_dir.push("userdata");

    user_dir.read_dir().unwrap().for_each(|user| {
        let user = user.unwrap();

        if user.file_type().unwrap().is_dir() == false {
            return;
        }

        let mut config_path = user.path();
        config_path.push("config");
        config_path.push("localconfig.vdf");

        if config_path.exists() == false {
            return;
        }

        let user = user.file_name().into_string().unwrap();
        users.push(SteamUser { user, config_path });
    });

    users
}

impl SteamConfig {
    pub fn new(steam_dir: &PathBuf) -> SteamConfig {
        let mut config = steam_dir.clone();

        config.push("config");
        config.push("config.vdf");

        let users = get_users(steam_dir);

        SteamConfig {
            config_path: config,
            users,
        }
    }
}
