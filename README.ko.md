# Termux Terminal

Android Obsidian 안에서 네이티브 Termux의 실제 셸을 쓰는 플러그인이다.
플러그인은 Obsidian 탭에 터미널을 렌더링하고, 로컬 Rust bridge가 PTY, 셸,
프로세스 수명을 소유한다.

English: [README.md](README.md)

## 요구 사항

- Community plugin을 허용한 Android Obsidian
- `aarch64` 네이티브 Termux
- 지원하는 터미널 작업을 위한 하드웨어 키보드

bridge는 기기 내부에서만 동작한다. SSH 클라이언트나 원격 셸 서버,
code-server 대체품이 아니다.

## 설치

첫 GitHub Release가 공개된 뒤 native Termux에서 다음을 실행한다.

```sh
curl -fsSL https://raw.githubusercontent.com/Glaysia/termux-terminal/main/scripts/install-termux-bridge.sh | sh
```

설치기는 공개된 체크섬을 검증하고 bridge를 Termux `runit` 서비스로 설치한 뒤
연결 토큰을 한 번 출력한다.

Obsidian에서는 다음 순서로 연결한다.

1. Community plugins에서 **Termux Terminal**을 설치한다.
2. 설정을 열고 출력된 bridge 토큰을 붙여넣는다.
3. 터미널 리본 아이콘 또는 `Open terminal` 명령을 사용한다.

## 셸 시작

각 터미널 탭은 새 인터랙티브 Bash를 시작한다. bridge가 시작한 Bash는
`~/.obsidianrc`만 source하며, `~/.bashrc`는 자동 실행하지 않는다.

설치기는 `.obsidianrc`에 주석 처리된 `source ~/.bashrc` 줄을 만든다.
Obsidian에서 연 터미널에도 일반 Bash 설정이 필요할 때만 주석을 해제한다.

## 보안

- bridge는 `127.0.0.1`에만 바인딩한다.
- 모든 연결은 권한 `0600`의 `~/.termux_terminal_token`에 저장된 설치 토큰을
  요구한다.
- 토큰은 6개월 뒤 만료되며 마지막 7일에는 셸 경고를 표시한다.
- 기본적으로 터미널 입력, 출력, 토큰을 기록하지 않는다.
- 포트 포워딩은 사용자가 직접 구성하고 보호한다. 플러그인이 loopback 전용
  bridge 바인딩을 바꾸지 않는다.

## 운영

Termux에서 native 서비스를 확인한다.

```sh
SVDIR="$PREFIX/var/service" sv status termux-terminal-bridge
```

bridge를 갱신한 뒤 재시작한다.

```sh
SVDIR="$PREFIX/var/service" sv restart termux-terminal-bridge
```

## 개발

브랜치와 검증 규칙은 [CONTRIBUTING.md](CONTRIBUTING.md)에 있다. 릴리스 기록은
[CHANGELOG.md](CHANGELOG.md), 보안 제보 방식은 [SECURITY.md](SECURITY.md)를
참고한다.

## 라이선스

AGPL-3.0-or-later. [LICENSE](LICENSE)를 참고한다.
