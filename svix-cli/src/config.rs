use anyhow::Result;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub auth_token: Option<String>,
    pub server_url: Option<String>,

    // Relay stuff relates to the `listen` command.
    pub relay_token: Option<String>,
    // FIXME: "url" isn't right. We expect a hostname, default is: `api.play.svix.com`
    pub relay_debug_url: Option<String>,
    pub relay_disable_security: Option<bool>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let cfg_file = get_folder()?.join(FILE_NAME);
        let config: Config = Figment::new()
            .merge(Toml::file(cfg_file))
            .merge(Env::prefixed("SVIX_"))
            .extract()?;
        Ok(config)
    }

    pub fn save_to_disk(&self, path: &Path) -> Result<()> {
        let mut fh = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .mode(FILE_MODE)
            .open(path)?;

        let source = &toml::to_string_pretty(self)?;
        fh.write_all(source.as_bytes())?;
        Ok(())
    }
}

const FILE_NAME: &str = "config.toml";
const FILE_MODE: u32 = 0o600;

fn get_folder() -> Result<PathBuf> {
    let config_path = if cfg!(windows) {
        std::env::var("APPDATA")
    } else {
        std::env::var("XDG_CONFIG_HOME")
    };

    let pb = match config_path {
        Ok(path) => PathBuf::from(path),
        Err(_e) => {
            // FIXME: home_dir() can give incorrect results on Windows. Docs recommend "use a crate instead"
            #[allow(deprecated)]
            std::env::home_dir().ok_or_else(|| anyhow::anyhow!("unable to find config path"))?
        }
    };
    Ok(pb.join(".config").join("svix"))
}

pub fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_folder()?.join(FILE_NAME))
}
