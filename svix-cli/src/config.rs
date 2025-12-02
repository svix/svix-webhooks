use std::{
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context as _, Result};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use fs_err::{File, OpenOptions};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    debug_url: Option<String>,

    // Relay stuff relates to the `listen` command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_token: Option<String>,
    #[serde(alias = "relay_debug_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_debug_hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_disable_security: Option<bool>,
}

fn config_file_open_opts() -> OpenOptions {
    let mut opts = File::options();
    opts.create(true).truncate(true).write(true);
    opts
}

#[cfg(windows)]
fn open_config_file(path: &Path) -> Result<File> {
    Ok(config_file_open_opts().open(path)?)
}

#[cfg(unix)]
fn open_config_file(path: &Path) -> Result<File> {
    use std::os::unix::fs::OpenOptionsExt;
    const FILE_MODE: u32 = 0o600;
    let mut open_opts = config_file_open_opts();
    open_opts.options_mut().mode(FILE_MODE);
    Ok(open_opts.open(path)?)
}

impl Config {
    pub fn load() -> Result<Config> {
        let cfg_file = get_config_file_path()?;
        let config: Config = Figment::new()
            .merge(Toml::file(cfg_file))
            .merge(Env::prefixed("SVIX_"))
            .extract()?;
        Ok(config)
    }

    pub fn save_to_disk(&self, path: &Path) -> Result<()> {
        let mut fh = open_config_file(path)?;

        let source = &toml::to_string_pretty(self)?;
        fh.write_all(source.as_bytes())?;
        Ok(())
    }

    /// Gives the `server_url` for a Svix client with fallback to the legacy `SVIX_DEBUG_URL` variable/config.
    pub fn server_url(&self) -> Option<&str> {
        match self.server_url.as_deref() {
            Some(s) if s.trim().is_empty() => self.debug_url.as_deref(),
            server_url @ Some(_) => server_url,
            None => self.debug_url.as_deref(),
        }
    }
}

const FILE_NAME: &str = "config.toml";

fn get_folder() -> Result<PathBuf> {
    Ok(dirs::config_dir()
        .context("unable to find config path")?
        .join("svix"))
}

pub fn get_config_file_path() -> Result<PathBuf> {
    Ok(get_folder()?.join(FILE_NAME))
}
