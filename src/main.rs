mod cli;
mod config;
mod embedded;
mod init;
mod interactive;
mod template;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use dialoguer::Confirm;

use cli::{Cli, Commands, TemplateAction};

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "✗".red(), e.to_string().red());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // 인수 없이 실행 시 대화형 모드
            interactive::run_interactive_mode()?;
        }
        Some(Commands::List) => {
            handle_list()?;
        }
        Some(Commands::Init {
            template,
            template_flag,
        }) => {
            let template_name = template.or(template_flag);
            handle_init(template_name)?;
        }
        Some(Commands::Template { action }) => match action {
            TemplateAction::Install { file_path, name } => {
                handle_template_install(&file_path, name)?;
            }
            TemplateAction::Remove { template_name } => {
                handle_template_remove(&template_name)?;
            }
        },
        Some(Commands::Config { set_template_path }) => {
            if let Some(path) = set_template_path {
                handle_config_set_path(path)?;
            } else {
                println!("{}", "사용법: init-ai config --set-template-path <경로>".yellow());
            }
        }
    }

    Ok(())
}

/// list 명령 처리
fn handle_list() -> Result<()> {
    let config = config::load_config()?;
    let templates = template::list_templates(&config)?;

    if templates.is_empty() {
        println!(
            "{} 사용 가능한 템플릿이 없습니다.",
            "⚠️".yellow()
        );
        println!("'init-ai template install' 명령으로 템플릿을 추가하세요.");
    } else {
        println!("{}", "사용 가능한 템플릿:".cyan());
        for (i, template) in templates.iter().enumerate() {
            println!("  {}. {}", i + 1, template.name.cyan());
        }
    }

    Ok(())
}

/// init 명령 처리
fn handle_init(template_name: Option<String>) -> Result<()> {
    let config = config::load_config()?;

    // 템플릿 이름 결정
    let template_name = if let Some(name) = template_name {
        name
    } else {
        // 템플릿이 지정되지 않았으면 대화형 모드로 선택
        return interactive::run_interactive_mode();
    };

    // 파일 존재 시 덮어쓰기 확인
    if init::team_rules_exists() {
        let overwrite = Confirm::new()
            .with_prompt("rules/TEAM_RULES.md 파일이 이미 존재합니다. 덮어쓰시겠습니까?")
            .default(false)
            .interact()?;

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

/// template install 명령 처리
fn handle_template_install(
    file_path: &std::path::Path,
    name: Option<String>,
) -> Result<()> {
    let config = config::load_config()?;

    // 템플릿 이름 결정 (지정되지 않았으면 파일명 사용)
    let template_name = if let Some(name) = name {
        name
    } else {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("파일명을 확인할 수 없습니다"))?
            .to_string()
    };

    // 템플릿 설치
    template::install_template(file_path, &template_name, &config)?;

    println!(
        "{} 템플릿 '{}'이(가) 설치되었습니다.",
        "✓".green(),
        template_name.cyan()
    );

    Ok(())
}

/// template remove 명령 처리
fn handle_template_remove(template_name: &str) -> Result<()> {
    let config = config::load_config()?;

    // 삭제 확인
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

    // 템플릿 삭제
    template::remove_template(template_name, &config)?;

    println!(
        "{} 템플릿 '{}'이(가) 삭제되었습니다.",
        "✓".green(),
        template_name.cyan()
    );

    Ok(())
}

/// config --set-template-path 명령 처리
fn handle_config_set_path(path: std::path::PathBuf) -> Result<()> {
    config::set_template_path(path.clone())?;

    println!(
        "{} 템플릿 저장 경로가 설정되었습니다: {:?}",
        "✓".green(),
        path
    );

    // 기본 템플릿 설치
    let config = config::load_config()?;
    template::install_default_template(&config)?;

    println!(
        "{} 기본 템플릿 '{}'이(가) 설치되었습니다.",
        "✓".green(),
        "Programming-Team".cyan()
    );

    Ok(())
}
