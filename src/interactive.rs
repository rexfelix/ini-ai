use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, Input, Select};
use std::path::PathBuf;

use crate::config::{self, Config};
use crate::init;
use crate::template;

/// 메인 메뉴 옵션
enum MainMenuOption {
    StartProject,
    ManageTemplates,
    Exit,
}

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

    loop {
        // 메인 메뉴 표시
        match show_main_menu()? {
            MainMenuOption::StartProject => {
                if let Err(e) = start_project(&config) {
                    eprintln!("{} {}", "✗".red(), e.to_string().red());
                }
            }
            MainMenuOption::ManageTemplates => {
                if let Err(e) = manage_templates(&config) {
                    eprintln!("{} {}", "✗".red(), e.to_string().red());
                }
            }
            MainMenuOption::Exit => {
                println!("{}", "종료합니다.".yellow());
                break;
            }
        }
        println!();
    }

    Ok(())
}

/// 메인 메뉴 표시
fn show_main_menu() -> Result<MainMenuOption> {
    let options = vec![
        "프로젝트 시작하기",
        "템플릿 관리",
        "종료",
    ];

    let selection = Select::new()
        .with_prompt("원하는 작업을 선택하세요")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(MainMenuOption::StartProject),
        1 => Ok(MainMenuOption::ManageTemplates),
        2 => Ok(MainMenuOption::Exit),
        _ => unreachable!(),
    }
}

/// 프로젝트 시작하기
fn start_project(config: &Config) -> Result<()> {
    // 템플릿 선택
    let Some(template_name) = prompt_template_selection(config)? else {
        return Ok(());
    };

    // 파일 존재 시 덮어쓰기 확인
    if init::team_rules_exists() {
        let overwrite = confirm_overwrite()?;
        if !overwrite {
            println!("{}", "작업이 취소되었습니다.".yellow());
            return Ok(());
        }
    }

    // 프로젝트 초기화
    init::init_project(&template_name, config)?;

    println!(
        "{} rules/TEAM_RULES.md 파일이 생성되었습니다. (템플릿: {})",
        "✓".green(),
        template_name.cyan()
    );

    Ok(())
}

/// 템플릿 관리 메뉴
fn manage_templates(config: &Config) -> Result<()> {
    loop {
        let options = vec![
            "템플릿 등록",
            "템플릿 삭제",
            "템플릿 목록 보기",
            "돌아가기",
        ];

        let selection = Select::new()
            .with_prompt("템플릿 관리")
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => install_template_interactive(config)?,
            1 => remove_template_interactive(config)?,
            2 => list_templates_interactive(config)?,
            3 => break,
            _ => unreachable!(),
        }
        println!();
    }

    Ok(())
}

/// 템플릿 등록 (대화형)
fn install_template_interactive(config: &Config) -> Result<()> {
    let file_path: String = Input::new()
        .with_prompt("템플릿 파일 경로를 입력하세요 (취소: 빈 입력 후 Enter)")
        .allow_empty(true)
        .interact_text()?;

    // 빈 입력 시 취소
    if file_path.trim().is_empty() {
        println!("{}", "작업이 취소되었습니다.".yellow());
        return Ok(());
    }

    let path = PathBuf::from(&file_path);

    // 파일 존재 확인
    if !path.exists() {
        anyhow::bail!("파일을 찾을 수 없습니다: {}", file_path);
    }

    let template_name: String = Input::new()
        .with_prompt("템플릿 이름을 입력하세요 (취소: 빈 입력, 기본값: 파일명)")
        .allow_empty(true)
        .interact_text()?;

    let name = if template_name.trim().is_empty() {
        // 빈 입력인 경우 파일명 사용 (취소가 아님)
        path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("파일명을 확인할 수 없습니다"))?
            .to_string()
    } else {
        template_name
    };

    template::install_template(&path, &name, config)?;

    println!(
        "{} 템플릿 '{}'이(가) 설치되었습니다.",
        "✓".green(),
        name.cyan()
    );

    Ok(())
}

/// 템플릿 삭제 (대화형)
fn remove_template_interactive(config: &Config) -> Result<()> {
    let templates = template::list_templates(config)?;

    if templates.is_empty() {
        println!("{}", "사용 가능한 템플릿이 없습니다.".yellow());
        return Ok(());
    }

    let mut template_names: Vec<String> = templates.iter().map(|t| t.name.clone()).collect();
    template_names.push("돌아가기".to_string());

    let selection = Select::new()
        .with_prompt("삭제할 템플릿을 선택하세요")
        .items(&template_names)
        .default(0)
        .interact()?;

    if selection == template_names.len() - 1 {
        return Ok(());
    }

    let template_name = &template_names[selection];

    let confirm = Confirm::new()
        .with_prompt(format!(
            "템플릿 '{}'을(를) 삭제하시겠습니까?",
            template_name
        ))
        .default(false)
        .interact()?;

    if !confirm {
        println!("{}", "작업이 취소되었습니다.".yellow());
        return Ok(());
    }

    template::remove_template(template_name, config)?;

    println!(
        "{} 템플릿 '{}'이(가) 삭제되었습니다.",
        "✓".green(),
        template_name.cyan()
    );

    Ok(())
}

/// 템플릿 목록 보기 (대화형)
fn list_templates_interactive(config: &Config) -> Result<()> {
    let templates = template::list_templates(config)?;

    if templates.is_empty() {
        println!("{}", "사용 가능한 템플릿이 없습니다.".yellow());
    } else {
        println!("{}", "사용 가능한 템플릿:".cyan());
        for (i, template) in templates.iter().enumerate() {
            println!("  {}. {}", i + 1, template.name.cyan());
        }
    }

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
fn prompt_template_selection(config: &Config) -> Result<Option<String>> {
    let templates = template::list_templates(config)?;

    if templates.is_empty() {
        anyhow::bail!("사용 가능한 템플릿이 없습니다. 'initai template install' 명령으로 템플릿을 추가하세요.");
    }

    println!("{}", "사용 가능한 템플릿:".cyan());

    let mut template_names: Vec<String> = templates.iter().map(|t| t.name.clone()).collect();
    template_names.push("돌아가기".to_string());

    let selection = Select::new()
        .with_prompt("템플릿을 선택하세요")
        .items(&template_names)
        .default(0)
        .interact()?;

    if selection == template_names.len() - 1 {
        return Ok(None);
    }

    Ok(Some(template_names[selection].clone()))
}

/// 덮어쓰기 확인
fn confirm_overwrite() -> Result<bool> {
    let confirm = Confirm::new()
        .with_prompt("rules/TEAM_RULES.md 파일이 이미 존재합니다. 덮어쓰시겠습니까?")
        .default(false)
        .interact()?;

    Ok(confirm)
}
