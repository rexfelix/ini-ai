use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::embedded;

/// 템플릿 정보
#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    #[allow(dead_code)] // 향후 기능 확장을 위해 유지
    pub path: PathBuf,
}

/// 템플릿 목록 조회
pub fn list_templates(config: &Config) -> Result<Vec<Template>> {
    let mut templates = Vec::new();

    // 템플릿 디렉토리 확인
    if !config.template_path.exists() {
        return Ok(templates);
    }

    // 디렉토리 내 .md 파일 검색
    let entries = fs::read_dir(&config.template_path)
        .with_context(|| format!("템플릿 디렉토리를 읽을 수 없습니다: {:?}", config.template_path))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                templates.push(Template {
                    name: stem.to_string(),
                    path: path.clone(),
                });
            }
        }
    }

    // 이름순 정렬
    templates.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(templates)
}

/// 템플릿 설치
pub fn install_template(source: &Path, name: &str, config: &Config) -> Result<()> {
    // 파일 존재 확인
    if !source.exists() {
        anyhow::bail!("파일을 찾을 수 없습니다: {:?}", source);
    }

    // .md 확장자 확인
    if source.extension().and_then(|s| s.to_str()) != Some("md") {
        anyhow::bail!("마크다운 파일(.md)만 설치할 수 있습니다");
    }

    // 파일 크기 제한 (10MB)
    let metadata = fs::metadata(source)?;
    if metadata.len() > 10 * 1024 * 1024 {
        anyhow::bail!("파일 크기가 10MB를 초과합니다");
    }

    // 심볼릭 링크 차단
    if metadata.file_type().is_symlink() {
        anyhow::bail!("심볼릭 링크는 지원하지 않습니다");
    }

    // 템플릿 디렉토리 생성
    if !config.template_path.exists() {
        fs::create_dir_all(&config.template_path)?;
    }

    // 대상 경로
    let dest = config.template_path.join(format!("{}.md", name));

    // 중복 확인
    if dest.exists() {
        anyhow::bail!("템플릿 '{}'이(가) 이미 존재합니다", name);
    }

    // 파일 복사
    fs::copy(source, &dest)
        .with_context(|| format!("템플릿 파일을 복사할 수 없습니다: {:?} -> {:?}", source, dest))?;

    Ok(())
}

/// 템플릿 삭제
pub fn remove_template(name: &str, config: &Config) -> Result<()> {
    let template_path = config.template_path.join(format!("{}.md", name));

    if !template_path.exists() {
        anyhow::bail!("템플릿 '{}'을(를) 찾을 수 없습니다", name);
    }

    fs::remove_file(&template_path)
        .with_context(|| format!("템플릿을 삭제할 수 없습니다: {:?}", template_path))?;

    Ok(())
}

/// 템플릿 내용 가져오기
pub fn get_template_content(name: &str, config: &Config) -> Result<String> {
    let template_path = config.template_path.join(format!("{}.md", name));

    if !template_path.exists() {
        anyhow::bail!("템플릿 '{}'을(를) 찾을 수 없습니다", name);
    }

    let content = fs::read_to_string(&template_path)
        .with_context(|| format!("템플릿 파일을 읽을 수 없습니다: {:?}", template_path))?;

    Ok(content)
}

/// 기본 템플릿 설치
pub fn install_default_template(config: &Config) -> Result<()> {
    // 템플릿 디렉토리 생성
    if !config.template_path.exists() {
        fs::create_dir_all(&config.template_path)?;
    }

    // 내장 템플릿 설치
    for (name, content) in embedded::get_embedded_templates() {
        let dest = config.template_path.join(format!("{}.md", name));

        // 이미 존재하면 건너뛰기
        if dest.exists() {
            continue;
        }

        fs::write(&dest, content)
            .with_context(|| format!("기본 템플릿을 설치할 수 없습니다: {:?}", dest))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_list_templates_empty() {
        let temp_dir = tempdir().unwrap();
        let config = Config::new(temp_dir.path().to_path_buf());

        let templates = list_templates(&config).unwrap();
        assert_eq!(templates.len(), 0);
    }

    #[test]
    fn test_install_default_template() {
        let temp_dir = tempdir().unwrap();
        let config = Config::new(temp_dir.path().to_path_buf());

        install_default_template(&config).unwrap();

        let templates = list_templates(&config).unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "Programming-Team");
    }

    #[test]
    fn test_get_template_content() {
        let temp_dir = tempdir().unwrap();
        let config = Config::new(temp_dir.path().to_path_buf());

        install_default_template(&config).unwrap();

        let content = get_template_content("Programming-Team", &config).unwrap();
        assert!(content.contains("AI Software Engineering Team System"));
    }
}
