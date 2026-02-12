use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub template_path: PathBuf,
    pub default_template: String,
}

impl Config {
    pub fn new(template_path: PathBuf) -> Self {
        Self {
            template_path,
            default_template: "Programming-Team".to_string(),
        }
    }
}

/// 설정 파일 경로 가져오기
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("설정 디렉토리를 찾을 수 없습니다")?;

    let init_ai_dir = config_dir.join("initai");

    // 디렉토리가 없으면 생성
    if !init_ai_dir.exists() {
        fs::create_dir_all(&init_ai_dir)
            .context("설정 디렉토리를 생성할 수 없습니다")?;
    }

    Ok(init_ai_dir.join("config.toml"))
}

/// 설정 파일 로드
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        anyhow::bail!("설정 파일이 존재하지 않습니다. 템플릿 경로를 먼저 설정해주세요.");
    }

    let content = fs::read_to_string(&config_path)
        .context("설정 파일을 읽을 수 없습니다")?;

    let config: Config = toml::from_str(&content)
        .context("설정 파일 형식이 올바르지 않습니다")?;

    Ok(config)
}

/// 설정 파일 저장
pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    let content = toml::to_string_pretty(config)
        .context("설정을 직렬화할 수 없습니다")?;

    fs::write(&config_path, content)
        .context("설정 파일을 저장할 수 없습니다")?;

    Ok(())
}

/// 설정 파일 존재 여부 확인
pub fn config_exists() -> Result<bool> {
    let config_path = get_config_path()?;
    Ok(config_path.exists())
}

/// 템플릿 경로 설정
pub fn set_template_path(path: PathBuf) -> Result<()> {
    // 경로를 절대 경로로 변환
    let absolute_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };

    // 디렉토리가 없으면 생성
    if !absolute_path.exists() {
        fs::create_dir_all(&absolute_path)
            .with_context(|| format!("디렉토리를 생성할 수 없습니다: {:?}", absolute_path))?;
    }

    // 설정 저장
    let config = Config::new(absolute_path);
    save_config(&config)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let path = PathBuf::from("/test/path");
        let config = Config::new(path.clone());

        assert_eq!(config.template_path, path);
        assert_eq!(config.default_template, "Programming-Team");
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::new(PathBuf::from("/test/path"));
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.template_path, deserialized.template_path);
        assert_eq!(config.default_template, deserialized.default_template);
    }
}
