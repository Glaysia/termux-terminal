# Termux Terminal

Android Obsidian 안에서 실제 Termux 인터랙티브 셸을 사용하는 플러그인이다.
플러그인은 xterm.js를 렌더링하고, Rust bridge가 로컬 PTY와 셸을 소유한다.

English: [README.md](README.md)

## 상태

`1.0.0`은 Android 릴리스 후보 테스트 중이다. 공개 런타임은 native aarch64
Termux만 지원하며 bridge는 `127.0.0.1:11557`에만 바인딩한다.

## 설치

첫 GitHub Release 뒤 native Termux에서 실행한다.

```sh
curl -fsSL https://raw.githubusercontent.com/Glaysia/termux-terminal/main/scripts/install-termux-bridge.sh | sh
```

설치기는 release 바이너리와 `SHA256SUMS`를 검증하고, `runit` 서비스,
`0600` 권한의 `~/.termux_terminal_token`을 만든다. 출력된 토큰을 Obsidian
설정의 Termux Terminal에 붙여넣은 뒤 리본 아이콘이나 `Open terminal` 명령을 쓴다.

## 셸 시작

bridge가 시작한 Bash는 `~/.obsidianrc`만 source한다. `.bashrc`는 자동으로
실행하지 않는다. 생성되는 템플릿의 주석 처리된 `source ~/.bashrc`를 풀면
기존 환경을 선택적으로 포함할 수 있다.

## 보안

- bridge는 loopback만 사용한다.
- production 연결은 첫 WebSocket 메시지에 설치 토큰을 포함해야 한다.
- 토큰은 6개월 유효하며, 만료 전 7일 동안 셸 경고를 표시한다.
- 기본 로그에는 토큰, 입력, 터미널 출력이 남지 않는다.

## 개발

개발과 Android 테스트는 `feat/terminal-vertical-slice`에서 한다. 검증된
릴리스 후보만 `main`에 squash merge한다. [GOAL.md](GOAL.md)와
`docs/specs/public-release-preparation/`을 참고한다.

## 라이선스

AGPL-3.0-or-later. [LICENSE](LICENSE)를 참고한다.
