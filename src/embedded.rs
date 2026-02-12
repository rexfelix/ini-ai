/// Programming-Team 템플릿 (기본 템플릿)
pub const PROGRAMMING_TEAM: &str = include_str!("../templates/programming-team.md");

/// 기본 템플릿 이름
pub const DEFAULT_TEMPLATE_NAME: &str = "Programming-Team";

/// 사용 가능한 내장 템플릿 목록
pub fn get_embedded_templates() -> Vec<(&'static str, &'static str)> {
    vec![(DEFAULT_TEMPLATE_NAME, PROGRAMMING_TEAM)]
}
