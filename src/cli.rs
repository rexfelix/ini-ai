use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "init-ai")]
#[command(about = "AI 팀 협업 규칙 파일을 프로젝트에 추가하는 CLI 도구", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 템플릿 목록 보기
    #[command(alias = "ls")]
    List,

    /// 프로젝트 초기화
    Init {
        /// 사용할 템플릿 이름
        template: Option<String>,

        /// 템플릿 이름 (--template 플래그 사용)
        #[arg(short, long)]
        template_flag: Option<String>,
    },

    /// 템플릿 관리
    Template {
        #[command(subcommand)]
        action: TemplateAction,
    },

    /// 설정 관리
    Config {
        /// 템플릿 저장 경로 설정
        #[arg(long)]
        set_template_path: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum TemplateAction {
    /// 템플릿 설치
    Install {
        /// 설치할 템플릿 파일 경로
        file_path: PathBuf,

        /// 템플릿 이름 (지정하지 않으면 파일명 사용)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// 템플릿 삭제
    #[command(alias = "rm")]
    Remove {
        /// 삭제할 템플릿 이름
        template_name: String,
    },
}
