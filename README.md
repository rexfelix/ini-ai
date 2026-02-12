# initai

> AI 팀 협업 규칙 파일을 프로젝트에 손쉽게 추가하는 CLI 도구

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 📋 목차

- [소개](#소개)
- [주요 기능](#주요-기능)
- [설치 방법](#설치-방법)
- [빠른 시작](#빠른-시작)
- [사용법](#사용법)
- [템플릿 관리](#템플릿-관리)
- [명령어 레퍼런스](#명령어-레퍼런스)
- [예제](#예제)
- [개발](#개발)
- [라이선스](#라이선스)

---

## 소개

`initai`는 AI 기반 개발 팀 협업을 위한 규칙 파일(`TEAM_RULES.md`)을 프로젝트에 자동으로 생성해주는 CLI 도구입니다. 여러 템플릿 중 선택하거나 커스텀 템플릿을 추가하여 프로젝트에 맞는 팀 규칙을 손쉽게 적용할 수 있습니다.

### 왜 initai를 사용해야 하나요?

- ✅ **빠른 프로젝트 설정**: 몇 초 만에 AI 팀 규칙 파일 생성
- ✅ **템플릿 관리**: 여러 템플릿을 저장하고 관리
- ✅ **사용자 친화적**: 대화형 모드와 명령줄 모드 모두 지원
- ✅ **크로스 플랫폼**: macOS, Linux, Windows 모두 지원

---

## 주요 기능

### 🎯 핵심 기능

- **대화형 모드**: 직관적인 프롬프트로 템플릿 선택
- **명령줄 모드**: 스크립트 자동화를 위한 CLI 옵션
- **템플릿 관리**: 템플릿 설치, 삭제, 조회
- **기본 템플릿 제공**: `Programming-Team` 템플릿 내장
- **파일 검증**: 마크다운 파일(.md)만 허용, 크기 제한 (10MB)

### 🔒 보안 기능

- 파일 확장자 검증
- 파일 크기 제한
- 심볼릭 링크 차단
- 경로 순회 공격 방지

---

## 설치 방법

### 방법 1: 소스에서 빌드 (권장)

```bash
# 저장소 클론
git clone https://github.com/yourusername/initai.git
cd initai

# 빌드 및 설치
cargo build --release
sudo cp target/release/initai /usr/local/bin/

# 또는 cargo install 사용
cargo install --path .
```

### 방법 2: 바이너리 다운로드

[Releases 페이지](https://github.com/yourusername/initai/releases)에서 플랫폼에 맞는 바이너리를 다운로드하세요.

```bash
# macOS/Linux
chmod +x initai
sudo mv initai /usr/local/bin/
```

### 설치 확인

```bash
initai --help
```

---

## 빠른 시작

### 1. 최초 설정

처음 실행 시 템플릿 저장 경로를 설정합니다:

```bash
initai config --set-template-path ~/.config/initai/templates
```

> 💡 **경로는 자유롭게 선택 가능합니다!**
> - `~/.config/initai/templates` - 표준 설정 위치 (권장)
> - `~/Documents/templates` - 찾기 쉬운 위치
> - `~/Dropbox/ai-templates` - 클라우드 동기화
> - `/opt/company/templates` - 팀 공유 경로
> - `./templates` - 현재 프로젝트 내 (상대 경로)
>
> 어떤 경로든 지정 가능하며, 나중에 언제든 변경할 수 있습니다.

**결과**:
```
✓ 템플릿 저장 경로가 설정되었습니다: "/Users/user/.config/initai/templates"
✓ 기본 템플릿 'Programming-Team'이(가) 설치되었습니다.
```

### 2. 프로젝트 초기화

프로젝트 루트 디렉토리에서 실행:

```bash
# 방법 1: 대화형 모드
initai

# 방법 2: 직접 템플릿 지정
initai init Programming-Team
```

**결과**:
```
✓ rules/TEAM_RULES.md 파일이 생성되었습니다.
```

생성된 파일 구조:
```
your-project/
└── rules/
    └── TEAM_RULES.md
```

---

## 사용법

### 대화형 모드

`initai`를 인수 없이 실행하면 대화형 모드가 시작됩니다:

```bash
initai
```

**진행 과정**:
```
=== initai 대화형 모드 ===

사용 가능한 템플릿:
  1. Programming-Team
  2. CustomTemplate

템플릿을 선택하세요: [1]
✓ rules/TEAM_RULES.md 파일이 생성되었습니다.
```

### 명령줄 모드

스크립트나 자동화에 적합한 명령줄 옵션:

```bash
# 특정 템플릿으로 초기화
initai init Programming-Team

# 템플릿 목록 보기
initai list

# 도움말
initai --help
```

---

## 템플릿 관리

### 템플릿 목록 조회

```bash
initai list
# 또는
initai ls
```

**출력 예시**:
```
사용 가능한 템플릿:
  1. Programming-Team
  2. CustomTemplate
  3. MinimalSetup
```

### 템플릿 설치

외부 마크다운 파일을 템플릿으로 추가:

```bash
# 파일 이름으로 자동 설정
initai template install ~/Documents/my-rules.md

# 커스텀 이름 지정
initai template install ~/Documents/my-rules.md --name MyCustomTemplate
```

**결과**:
```
✓ 템플릿 'MyCustomTemplate'이(가) 설치되었습니다.
```

### 템플릿 삭제

```bash
initai template remove MyCustomTemplate
# 또는
initai template rm MyCustomTemplate
```

**확인 프롬프트**:
```
템플릿 'MyCustomTemplate'을(를) 삭제하시겠습니까? (y/N): y
✓ 템플릿 'MyCustomTemplate'이(가) 삭제되었습니다.
```

### 템플릿 저장 경로 변경

언제든지 템플릿 저장 경로를 변경할 수 있습니다:

```bash
# 원하는 경로로 변경
initai config --set-template-path /path/to/templates

# 예시: 홈 디렉토리 하위로 변경
initai config --set-template-path ~/my-templates

# 예시: 절대 경로
initai config --set-template-path /Users/username/Documents/ai-rules

# 예시: 상대 경로 (현재 디렉토리 기준)
initai config --set-template-path ./project-templates
```

**경로 선택 가이드**:

| 경로 타입 | 예시 | 장점 | 추천 대상 |
|----------|------|------|----------|
| 표준 설정 | `~/.config/initai/templates` | 시스템 표준, 깔끔함 | 개인 사용자 |
| 홈 디렉토리 | `~/Documents/templates` | 찾기 쉬움, 백업 용이 | 초보자 |
| 클라우드 동기화 | `~/Dropbox/ai-templates` | 여러 PC 동기화 | 멀티 디바이스 사용자 |
| 시스템 전역 | `/opt/company/templates` | 팀원 전체 공유 | 서버/팀 환경 |
| 프로젝트 내 | `./templates` | Git으로 버전 관리 | 프로젝트별 관리 |

---

## 명령어 레퍼런스

### `initai`
대화형 모드 시작

### `initai list` / `initai ls`
설치된 모든 템플릿 목록 표시

**옵션**: 없음

### `initai init [템플릿명]`
프로젝트 초기화 (rules/TEAM_RULES.md 생성)

**인수**:
- `[템플릿명]` (선택): 사용할 템플릿 이름. 생략 시 대화형 모드로 선택

**옵션**:
- `-t, --template <템플릿명>`: 템플릿 이름 지정 (인수 대신 사용 가능)

**예시**:
```bash
initai init                        # 대화형 선택
initai init Programming-Team       # 직접 지정
initai init --template MyTemplate  # 플래그 사용
```

### `initai template install <파일경로> [옵션]`
새 템플릿 설치

**인수**:
- `<파일경로>` (필수): 설치할 마크다운 파일 경로

**옵션**:
- `-n, --name <이름>`: 템플릿 이름 지정 (기본: 파일명)

**예시**:
```bash
initai template install ~/rules.md
initai template install ~/rules.md --name CustomRules
```

**제약사항**:
- `.md` 확장자만 허용
- 최대 파일 크기: 10MB
- 심볼릭 링크 불가

### `initai template remove <템플릿명>` / `initai template rm <템플릿명>`
템플릿 삭제

**인수**:
- `<템플릿명>` (필수): 삭제할 템플릿 이름

**예시**:
```bash
initai template remove OldTemplate
```

### `initai config --set-template-path <경로>`
템플릿 저장 경로 설정

**옵션**:
- `--set-template-path <경로>`: 템플릿을 저장할 디렉토리 경로

**예시**:
```bash
initai config --set-template-path ~/.initai/templates
```

### `initai --help` / `initai -h`
도움말 표시

---

## 예제

### 시나리오 1: 새 프로젝트에 AI 팀 규칙 추가

```bash
# 프로젝트 디렉토리로 이동
cd my-new-project

# 대화형 모드로 템플릿 선택 및 생성
initai
```

### 시나리오 2: 커스텀 팀 규칙 템플릿 만들기

```bash
# 1. 마크다운 파일 작성
vim ~/my-team-rules.md

# 2. 템플릿으로 설치
initai template install ~/my-team-rules.md --name MyTeamRules

# 3. 확인
initai list

# 4. 새 프로젝트에 적용
cd another-project
initai init MyTeamRules
```

### 시나리오 3: CI/CD 스크립트에서 자동화

```bash
#!/bin/bash
# setup-project.sh

# 프로젝트 초기화
git init
echo "# My Project" > README.md

# AI 팀 규칙 자동 추가
initai init Programming-Team

# Git 커밋
git add .
git commit -m "Initial commit with team rules"
```

### 시나리오 4: 여러 프로젝트 유형별 템플릿 관리

```bash
# 다양한 템플릿 설치
initai template install ~/templates/frontend-team.md --name Frontend
initai template install ~/templates/backend-team.md --name Backend
initai template install ~/templates/fullstack-team.md --name FullStack

# 프로젝트 유형에 따라 선택
cd frontend-project && initai init Frontend
cd ../backend-project && initai init Backend
```

---

## 개발

### 요구사항

- Rust 1.70 이상
- Cargo

### 빌드

```bash
# 개발 빌드
cargo build

# 릴리스 빌드
cargo build --release

# 테스트 실행
cargo test

# 코드 품질 검사
cargo clippy
```

### 프로젝트 구조

```
initai/
├── src/
│   ├── main.rs          # 진입점
│   ├── cli.rs           # CLI 정의
│   ├── config.rs        # 설정 관리
│   ├── template.rs      # 템플릿 관리
│   ├── init.rs          # 프로젝트 초기화
│   ├── interactive.rs   # 대화형 모드
│   └── embedded.rs      # 내장 템플릿
├── templates/           # 내장 템플릿 파일
├── docs/                # 프로젝트 문서
├── Cargo.toml
└── README.md
```

### 의존성

```toml
clap = "4.5"          # CLI 프레임워크
dialoguer = "0.11"    # 대화형 프롬프트
dirs = "5.0"          # 크로스 플랫폼 경로
serde = "1.0"         # 직렬화
toml = "0.8"          # 설정 파일 형식
colored = "2.1"       # 터미널 색상
anyhow = "1.0"        # 에러 처리
```

---

## 설정 파일

### 위치

- **macOS/Linux**: `~/.config/initai/config.toml`
- **Windows**: `%APPDATA%\initai\config.toml`

### 형식

```toml
template_path = "/Users/user/.config/initai/templates"
default_template = "Programming-Team"
```

### 수동 편집

설정 파일을 직접 수정할 수도 있습니다:

```bash
vim ~/.config/initai/config.toml
```

---

## 문제 해결

### 템플릿이 표시되지 않음

```bash
# 템플릿 경로 확인
cat ~/.config/initai/config.toml

# 템플릿 디렉토리 확인
ls -la ~/.config/initai/templates/
```

### "설정 파일이 존재하지 않습니다" 오류

최초 설정이 필요합니다:

```bash
initai config --set-template-path ~/.config/initai/templates
```

### 파일 덮어쓰기 확인

기존 `rules/TEAM_RULES.md` 파일이 있는 경우 덮어쓰기 확인 프롬프트가 표시됩니다:

```
rules/TEAM_RULES.md 파일이 이미 존재합니다. 덮어쓰시겠습니까? (y/N):
```

---

## 기여하기

기여를 환영합니다! 다음 단계를 따라주세요:

1. 저장소 포크
2. 기능 브랜치 생성 (`git checkout -b feature/amazing-feature`)
3. 변경사항 커밋 (`git commit -m 'Add amazing feature'`)
4. 브랜치 푸시 (`git push origin feature/amazing-feature`)
5. Pull Request 생성

### 코드 스타일

- Rust 표준 포맷팅 사용 (`cargo fmt`)
- Clippy 경고 없음 (`cargo clippy`)
- 모든 테스트 통과 (`cargo test`)

---

## 라이선스

이 프로젝트는 MIT 라이선스 하에 배포됩니다. 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

---

## 지원

- **이슈**: [GitHub Issues](https://github.com/yourusername/initai/issues)
- **문서**: [docs/](docs/) 디렉토리
- **이메일**: your.email@example.com

---

## 감사의 말

이 프로젝트는 다음 오픈소스 프로젝트를 사용합니다:

- [clap](https://github.com/clap-rs/clap) - CLI 파싱
- [dialoguer](https://github.com/console-rs/dialoguer) - 대화형 프롬프트
- [dirs](https://github.com/dirs-dev/dirs-rs) - 플랫폼별 디렉토리
- [serde](https://github.com/serde-rs/serde) - 직렬화
- [colored](https://github.com/colored-rs/colored) - 터미널 색상

---

Made with ❤️ by the initai team
