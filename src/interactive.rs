use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, Input, Select};
use std::path::PathBuf;

use crate::config::{self, Config};
use crate::init;
use crate::template;

/// 대화형 모드 실행
pub fn run_interactive_mode() -> Result<()> {
    println!("{}", "=== initai 대화형 모드 ===".cyan().bold());
    println!();

    // 설정 파일 확인
    let config = if config::config_exists()? {
        config::load_config()?
    } else {
        // 설정 파일이 없으면 템플릿 경로 설정
        println!("{}", "⚠️  템플릿 저장 경로가 설정되지 않았습니다.".yellow());
        setup_template_path()?
    };

    // 템플릿 선택
    let template_name = prompt_template_selection(&config)?;

    // 파일 존재 시 덮어쓰기 확인
    if init::team_rules_exists() {
        let overwrite = confirm_overwrite()?;
        if !overwrite {
            println!("{}", "작업이 취소되었습니다.".yellow());
            return Ok(());
        }
    }

    // 프로젝트 초기화
    init::init_project(&template_name, &config)?;

    println!(
        "{} rules/TEAM_RULES.md 파일이 생성되었습니다. (템플릿: {})",
        "✓".green(),
        template_name.cyan()
    );

    Ok(())
}

/// 템플릿 경로 설정
fn setup_template_path() -> Result<Config> {
    let default_path = dirs::config_dir()
        .context("설정 디렉토리를 찾을 수 없습니다")?
        .join("initai")
        .join("templates");

    let path_str: String = Input::new()
        .with_prompt("템플릿을 저장할 경로를 입력하세요")
        .default(default_path.to_string_lossy().to_string())
        .interact_text()?;

    let path = PathBuf::from(path_str);

    config::set_template_path(path.clone())?;
    println!("{} 템플릿 저장 경로 설정 완료", "✓".green());

    let config = config::load_config()?;

    // 기본 템플릿 설치
    template::install_default_template(&config)?;
    println!(
        "{} 기본 템플릿 '{}' 설치 완료",
        "✓".green(),
        "Programming-Team".cyan()
    );
    println!();

    Ok(config)
}

/// 템플릿 선택 프롬프트
fn prompt_template_selection(config: &Config) -> Result<String> {
    let templates = template::list_templates(config)?;

    if templates.is_empty() {
        anyhow::bail!("사용 가능한 템플릿이 없습니다. 'initai template install' 명령으로 템플릿을 추가하세요.");
    }

    println!("{}", "사용 가능한 템플릿:".cyan());

    let template_names: Vec<String> = templates.iter().map(|t| t.name.clone()).collect();

    let selection = Select::new()
        .with_prompt("템플릿을 선택하세요")
        .items(&template_names)
        .default(0)
        .interact()?;

    Ok(template_names[selection].clone())
}

/// 덮어쓰기 확인
fn confirm_overwrite() -> Result<bool> {
    let confirm = Confirm::new()
        .with_prompt("rules/TEAM_RULES.md 파일이 이미 존재합니다. 덮어쓰시겠습니까?")
        .default(false)
        .interact()?;

    Ok(confirm)
}
