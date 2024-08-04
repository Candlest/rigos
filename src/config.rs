use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::Write};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub site_title: String,
    pub site_link: String,
    pub site_description: String,
    pub theme: String,
    pub pages: Vec<String>,
    pub rss_page: Option<bool>
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&config_str)?;
    Ok(config)
}

// 写入配置文件的函数
pub fn write_config(path: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // 将 Config 结构体序列化为 TOML 格式的字符串
    let config_toml = toml::to_string(config)?;

    // 创建或覆盖文件，并获取可写对象
    let mut file = File::create(path)?;

    // 将 TOML 格式的字符串写入文件
    file.write_all(config_toml.as_bytes())?;

    // 刷新文件以确保内容被写入磁盘
    file.flush()?;

    Ok(())
}