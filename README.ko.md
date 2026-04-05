# obsidian-termux

`obsidian-termux`는 Android용 Obsidian 안에서 실제 Termux 셸 세션을 사용할 수 있게 만들기 위한 프로젝트입니다.

구조는 두 부분으로 나뉩니다. 하나는 Obsidian 앱 안의 UI를 담당하는 플러그인이고, 다른 하나는 Termux 안에서 실행되며 실제 셸 세션을 소유하는 작은 로컬 브리지 프로세스입니다. 현재 검증된 운영 모델은 Debian `proot` 안에서 빌드한 단일 `aarch64-unknown-linux-musl` 브리지 바이너리를 Debian `proot`와 native Termux 양쪽에서 공용으로 실행하는 방식입니다.

영문 문서: [README.md](README.md)

## 왜 이 프로젝트가 필요한가

Android의 Obsidian은 데스크톱 Electron 앱처럼 플러그인에 네이티브 터미널 환경을 직접 제공하지 않습니다. 플러그인이 실제 인터랙티브 셸, PTY 처리, 프로세스 수명주기 관리를 쓰려면, 그 책임을 플러그인 바깥의 별도 프로세스로 분리해야 합니다.

이 저장소는 그 역할 분리를 명확하게 유지하기 위해 존재합니다.

- Obsidian 쪽은 뷰, 설정, 연결 상태, 터미널 UI를 담당
- Termux 쪽은 로컬 전송, 셸 실행, PTY/세션 수명주기, 스트림 포워딩을 담당

## 저장소 구성

- `packages/obsidian-plugin`
  앱 내부 사용자 경험을 담당하는 TypeScript 기반 Obsidian 플러그인
- `crates/termux-bridge`
  Termux 쪽 셸 런타임과 통신하는 Rust 기반 로컬 브리지 프로세스

## 현재 상태

이미 있는 것:

- 저장소 기본 구조
- Obsidian 플러그인용 `pnpm` 워크스페이스
- 최소 플러그인 스켈레톤
- 최소 Rust bridge 크레이트
- `musl` 중심 브리지 빌드 경로

아직 없는 것:

- 실제 WebSocket 브리지 동작
- 셸/세션 관리 구현
- 터미널 렌더링 연동
- 완성된 end-to-end 프로토콜 구현

## 검증된 환경

검증일: `2026-04-05`

현재 검증된 흐름:

- Debian `proot` 안에서 브리지를 빌드
- 같은 바이너리를 Debian `proot` 안에서 실행
- 같은 바이너리를 native Termux에서 실행

검증된 기준:

- 바깥 런타임: native Termux
- 빌드 환경: Debian `proot`
- Rust 툴체인: Debian 쪽 `rustup`
- 브리지 타깃: `aarch64-unknown-linux-musl`

현재 문서화된 런타임 경로에서는 native Termux 쪽 Rust 툴체인이 없어도 브리지 실행에는 문제가 없습니다.

## 빌드

플러그인:

```bash
corepack enable
corepack pnpm install
corepack pnpm --filter @obsidian-termux/obsidian-plugin build
```

브리지:

```bash
proot-distro login debian --user harry --termux-home -- bash -lc '
cd /data/data/com.termux/files/home/Projects/obsidian-termux
rustup toolchain install stable --profile minimal
rustup default stable
rustup target add aarch64-unknown-linux-musl
cargo build -p termux-bridge --target aarch64-unknown-linux-musl --release
'
```

생성되는 산출물:

```bash
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```

## 실행

Debian `proot`에서 실행:

```bash
proot-distro login debian --user harry --termux-home -- bash -lc '
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
'
```

native Termux에서 실행:

```bash
/data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```

## 바이너리 검증

```bash
file /data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
ldd /data/data/com.termux/files/home/Projects/obsidian-termux/target/aarch64-unknown-linux-musl/release/termux-bridge
```

기대 결과:

- `file`: `statically linked`
- `ldd`: `not a dynamic executable`

## 관련 문서

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/protocol.md`](docs/protocol.md)

## 보안 경고

이 저장소의 모든 코드는 AI(`ChatGPT Codex`)에 의해 생성되었습니다.

보안은 보장되지 않습니다. 독립적인 리뷰, 테스트, 하드닝 없이 이 코드베이스를 안전하다고 가정하면 안 됩니다.
