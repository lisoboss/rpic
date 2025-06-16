use serde::Deserialize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct OssConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket: String,
    pub endpoint: String,
}

impl OssConfig {
    /// 优先从环境变量读取，失败则回退读取配置文件
    pub fn from_env_or_file() -> io::Result<Self> {
        match Self::from_env() {
            Ok(cfg) => Ok(cfg),
            Err(_) => Self::from_config_file(),
        }
    }

    /// 从环境变量读取配置
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            access_key_id: env::var("RPIC_OSS_ACCESS_KEY_ID")?,
            access_key_secret: env::var("RPIC_OSS_ACCESS_KEY_SECRET")?,
            bucket: env::var("RPIC_OSS_BUCKET")?,
            endpoint: env::var("RPIC_OSS_ENDPOINT")?,
        })
    }

    /// 从配置文件中读取配置
    pub fn from_config_file() -> io::Result<Self> {
        let paths = [
            PathBuf::from("/etc/rpic/config.toml"),
            dirs::config_dir()
                .map(|d| d.join("rpic/config.toml"))
                .unwrap_or_else(|| PathBuf::from("~/.config/rpic/config.toml")),
        ];

        for path in &paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                let config: ConfigFile = toml::from_str(&content)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
                return Ok(config.oss);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No valid OSS config found in env or config files",
        ))
    }
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    pub oss: OssConfig,
}
