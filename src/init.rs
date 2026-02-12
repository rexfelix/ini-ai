use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::template;

/// 프로젝트 초기화
pub fn init_project(template_name: &str, config: &Config) -> Result<()> {
    // rules 디렉토리 생성
    create_rules_directory()?;

    // 템플릿 내용 가져오기
    let content = template::get_template_content(template_name, config)
        .with_context(|| format!("템플릿 '{}'을(를) 찾을 수 없습니다", template_name))?;

    // TEAM_RULES.md 작성
    write_team_rules(&content)?;

    Ok(())
}

/// rules 디렉토리 생성
fn create_rules_directory() -> Result<()> {
    let rules_dir = PathBuf::from("rules");

    if !rules_dir.exists() {
        fs::create_dir(&rules_dir)
            .context("rules 디렉토리를 생성할 수 없습니다")?;
    }

    Ok(())
}

/// TEAM_RULES.md 파일 작성
fn write_team_rules(content: &str) -> Result<()> {
    let file_path = PathBuf::from("rules/TEAM_RULES.md");

    fs::write(&file_path, content)
        .context("TEAM_RULES.md 파일을 작성할 수 없습니다")?;

    Ok(())
}

/// TEAM_RULES.md 파일 존재 여부 확인
pub fn team_rules_exists() -> bool {
    PathBuf::from("rules/TEAM_RULES.md").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::install_default_template;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_create_rules_directory() {
        let temp_dir = tempdir().unwrap();
        let _original_dir = env::current_dir().unwrap();

        env::set_current_dir(&temp_dir).unwrap();
        create_rules_directory().unwrap();

        assert!(PathBuf::from("rules").exists());

        // 원래 디렉토리로 복원
        env::set_current_dir(&_original_dir).ok();
        // temp_dir는 함수가 끝날 때까지 유지됨
    }

    #[test]
    fn test_write_team_rules() {
        let temp_dir = tempdir().unwrap();
        let _original_dir = env::current_dir().unwrap();

        env::set_current_dir(&temp_dir).unwrap();
        create_rules_directory().unwrap();
        write_team_rules("Test content").unwrap();

        let content = fs::read_to_string("rules/TEAM_RULES.md").unwrap();
        assert_eq!(content, "Test content");

        // 원래 디렉토리로 복원
        env::set_current_dir(&_original_dir).ok();
        // temp_dir는 함수가 끝날 때까지 유지됨
    }

    #[test]
    fn test_init_project() {
        let temp_dir = tempdir().unwrap();
        let template_dir = tempdir().unwrap();
        let _original_dir = env::current_dir().unwrap();

        let config = Config::new(template_dir.path().to_path_buf());
        install_default_template(&config).unwrap();

        // 작업 디렉토리를 변경한 후 테스트 수행
        env::set_current_dir(&temp_dir).unwrap();

        init_project("Programming-Team", &config).unwrap();

        assert!(team_rules_exists());

        let content = fs::read_to_string("rules/TEAM_RULES.md").unwrap();
        assert!(content.contains("AI Software Engineering Team System"));

        // 원래 디렉토리로 복원
        env::set_current_dir(&_original_dir).ok();
        // temp_dir와 template_dir는 함수가 끝날 때까지 유지됨
    }
}
