# awb-rs

**[English](../../../README.md)**

[![CI](https://github.com/somethingwithproof/awb-rs/workflows/CI/badge.svg)](https://github.com/somethingwithproof/awb-rs/actions)
[![Release](https://img.shields.io/github/v/release/somethingwithproof/awb-rs)](https://github.com/somethingwithproof/awb-rs/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/somethingwithproof/awb-rs/badge)](https://scorecard.dev/viewer/?uri=github.com/somethingwithproof/awb-rs)

Rust로 재구현된 현대적이고 고성능인 AutoWikiBrowser (AWB)입니다. 안전성, 성능, 확장성에 중점을 두고 Wikipedia 자동 편집 기능을 제공합니다.

## 아키텍처

```
awb-rs/
├── awb-core/           # 핵심 비즈니스 로직 및 추상화 계층
├── awb-api/            # MediaWiki API 클라이언트 구현
├── awb-cli/            # 명령줄 인터페이스
├── awb-rules/          # 자동 편집용 규칙 엔진
├── awb-lua-plugin/     # Lua 플러그인 런타임
├── awb-wasm-plugin/    # WebAssembly 플러그인 런타임
├── awb-security/       # 보안 및 샌드박싱
├── awb-db/             # 영구 상태 관리
├── awb-ui/             # 터미널 UI 구성 요소
├── awb-fixtures/       # 테스트 픽스처 및 모의 데이터
└── awb-integration/    # 통합 테스트 스위트
```

**데이터 흐름:**
```
CLI → 코어 → API → MediaWiki
       ↓
    규칙 엔진 → 플러그인 (Lua/WASM)
       ↓
    보안 계층 → 샌드박스 실행
       ↓
    데이터베이스 → 상태 영속화
```

## 주요 기능

- **현대적인 아키텍처**: Tokio 런타임 기반 async/await를 활용한 고성능 처리
- **다중 위키 지원**: 모든 MediaWiki 기반 위키와 호환
- **규칙 기반 편집**: 선언적 TOML 기반 규칙 정의
- **플러그인 시스템**: Lua 스크립트 및 WebAssembly 모듈을 통한 확장 지원
- **강화된 보안**: 리소스 제한이 적용된 샌드박스 환경에서 플러그인 실행
- **일괄 작업**: 효율적인 대량 문서 처리
- **대화형 TUI**: 모니터링 및 제어를 위한 풍부한 터미널 인터페이스
- **드라이 런 모드**: 변경 사항을 적용하기 전에 미리 보기 제공
- **상태 관리**: 중단된 세션 재개 기능
- **속도 제한**: 위키 API 제한 및 모범 사례 준수
- **국제화**: `--lang`, `AWB_LANG` 또는 POSIX 로캘(`LC_ALL`/`LC_MESSAGES`/`LANG`)을 통해 45개 표시 언어를 지원하며, 아랍어·페르시아어·히브리어·우르두어의 RTL 레이아웃 지원

## 지원 언어

CLI, 봇 보고서 및 GTK4 UI는 45개 언어(6개 RTL 로캘 `ar`, `ar-EG`, `ar-MA`, `fa`, `he`, `ur` 포함)를 지원합니다. 시스템 로캘 자동 감지를 재정의하려면 `--lang <LANG>` 옵션을 전달하거나 `AWB_LANG` 환경 변수를 설정하십시오. 전체 언어 목록은 [README.md](../../../README.md)를 참조하십시오.

## 사전 요구 사항

- **Rust**: 1.85 이상 (MSRV)
- **Cargo**: Rust 설치 시 함께 포함됨

## 설치

### 소스에서 빌드

```bash
# 저장소 복제
git clone https://github.com/somethingwithproof/awb-rs.git
cd awb-rs

# 전체 워크스페이스 크레이트 빌드
cargo build --workspace --release

# 바이너리 설치
cargo install --path awb-cli
```

## 개발

### 빌드

```bash
# 디버그 빌드
cargo build --workspace

# 릴리스 빌드
cargo build --workspace --release
```

### 테스트

```bash
# 전체 테스트 실행
cargo test --workspace

# 출력을 표시하며 테스트 실행
cargo test --workspace -- --nocapture
```

### 린트

```bash
# 린트 문제 검사
cargo clippy --workspace --all-targets --all-features

# 자동 수정 적용
cargo clippy --fix --allow-dirty --workspace
```

### 포맷

```bash
# 코드 포맷 검사
cargo fmt --all -- --check

# 코드 포맷 적용
cargo fmt --all
```

## 사용 예시

### 인증

```bash
# 위키 로그인(자격 증명을 안전하게 저장)
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --username MyBot \
  --password-stdin

# OAuth를 사용한 로그인
awb-rs login \
  --wiki https://en.wikipedia.org/w/api.php \
  --oauth-consumer-key YOUR_KEY \
  --oauth-consumer-secret YOUR_SECRET
```

### 문서 목록 조회

```bash
# 분류에서 문서 목록 가져오기
awb-rs list \
  --source category \
  --query "Category:Stubs" \
  --wiki https://en.wikipedia.org/w/api.php

# 검색 결과에서 문서 목록 가져오기
awb-rs list \
  --source search \
  --query "hastemplate:cleanup" \
  --wiki https://en.wikipedia.org/w/api.php

# 파일에서 문서 목록 가져오기
awb-rs list \
  --source file \
  --query pages.txt \
  --wiki https://en.wikipedia.org/w/api.php
```

### 편집 규칙 실행

```bash
# 프로필을 사용하여 규칙 실행(드라이 런 모드)
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --dry-run

# 실제 문서에 규칙 적용
awb-rs run \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --source category \
  --query "Category:Articles needing cleanup"
```

### 자동 봇 모드

```bash
# 제한 설정을 적용한 무인 봇 모드 실행
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --max-edits 100 \
  --rate-limit 6 \
  --dry-run

# 이전 세션에서 재개
awb-rs bot \
  --wiki https://en.wikipedia.org/w/api.php \
  --profile my-rules.toml \
  --resume session-12345.db
```

### 규칙 프로필 예시

`my-rules.toml` 파일을 생성합니다.

```toml
[profile]
name = "정리 봇"
description = "일반적인 서식 오류 수정"
edit_summary = "자동 정리 (봇)"

[[rules]]
name = "연속 공백 수정"
pattern = "  +"
replacement = " "
regex = true

[[rules]]
name = "외부 링크 수정"
pattern = "http://en.wikipedia.org"
replacement = "https://en.wikipedia.org"

[[rules]]
name = "누락된 분류 추가"
plugin = "add_categories.lua"
```

## 플러그인 시스템

awb-rs는 두 가지 유형의 플러그인을 지원합니다.

### Lua 플러그인

```lua
-- add_categories.lua
function transform(page)
  if not page.content:match("%[%[Category:") then
    page.content = page.content .. "\n[[Category:Uncategorized]]"
  end
  return page
end
```

### WebAssembly 플러그인

```rust
// WASM으로 컴파일: cargo build --target wasm32-wasi
#[no_mangle]
pub extern "C" fn transform(content: &str) -> String {
    content.replace("foo", "bar")
}
```

**플러그인 기능:**
- 리소스 제한(메모리, CPU 시간)이 적용된 샌드박스 실행
- 안전한 API 하위 집합 접근(파일 시스템 및 네트워크 접근 차단)
- 버전 호환성 검사
- 개발 모드에서의 핫 리로드 지원

## 보안 모델

- **자격 증명 저장**: OS 키링을 활용한 안전한 비밀번호 저장
- **플러그인 샌드박싱**: 격리된 환경에서 Lua 및 WASM 실행
- **리소스 제한**: 플러그인별 CPU 시간, 메모리 및 명령어 수 제한
- **API 속도 제한**: `Retry-After` 헤더 준수 및 사용자 지정 지연 시간 지원
- **편집 검증**: 적용 전 변경 사항 미리 보기 및 감사 로그 기록
- **OAuth 지원**: 봇에 권장되는 인증 방식

## 라이선스

다음 라이선스 중 하나를 선택하여 사용할 수 있습니다.

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../LICENSE-APACHE) 또는 http://www.apache.org/licenses/LICENSE-2.0)
- MIT 라이선스 ([LICENSE-MIT](../../../LICENSE-MIT) 또는 http://opensource.org/licenses/MIT)

## 기여하기

기여를 환영합니다! 자세한 지침은 [CONTRIBUTING.md](../../../CONTRIBUTING.md)를 참고해 주십시오.

## 프로젝트 현황

이 프로젝트는 Rust로 AutoWikiBrowser를 활발히 재구현하고 있습니다. 현재 진행 상황은 다음과 같습니다.

- [x] 핵심 API 클라이언트
- [x] 규칙 엔진
- [x] CLI 인터페이스
- [x] 플러그인 시스템 (Lua/WASM)
- [x] 보안 샌드박싱
- [ ] 터미널 UI (진행 중)
- [ ] GUI (계획됨)
- [ ] 고급 차이점(diff) 시각화 (계획됨)

## 링크

- [문서](https://docs.rs/awb-rs)
- [이슈 트래커](https://github.com/somethingwithproof/awb-rs/issues)
- [원본 AWB](https://en.wikipedia.org/wiki/Wikipedia:AutoWikiBrowser)
