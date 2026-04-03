# obsidian-termux

`obsidian-termux`는 Android용 Obsidian 안에서 `Termux` 세션을 자연스럽게 붙여 쓰기 위한 프로젝트다.
초기 단계에서는 구현보다 구조를 먼저 고정한다. 현재 저장소는 `pnpm workspaces` 기반 TypeScript 모노레포 골격만 제공한다.

## 목표

- Android Obsidian에서 앱 내부 pane/view 형태의 터미널 UX를 제공한다.
- `Termux` 측 로컬 브리지 프로세스와 연결하는 구조를 전제로 한다.
- 웹 터미널을 단순 임베드하는 대신 Obsidian UI에 통합된 경험을 지향한다.

## 아키텍처 방향

- `packages/obsidian-plugin`
  - Obsidian 플러그인 패키지
  - 모바일 UI 통합, 뷰 수명주기, 설정, 세션 attach 진입점을 담당
- `packages/termux-bridge`
  - Termux 측 브리지 패키지
  - 향후 `localhost:11557` 계열 엔드포인트에서 세션 접근을 중계하는 역할

예정된 방향:

- 통신: WebSocket 기반 브리지 모델
- 터미널 렌더링: `xterm.js` 계열 우선 검토

아직 구현하지 않은 것:

- 실제 WebSocket 서버
- TTY attach/resize/reconnect 로직
- xterm 통합
- Obsidian 뷰/탭 UX 완성

## 왜 모노레포인가

- 플러그인과 브리지는 책임이 다르지만, 같은 제품 아키텍처 안에서 함께 진화한다.
- 프로토콜, 문서, 타입, 개발 흐름을 한 저장소에서 맞추기 쉽다.
- 초기에는 기능보다 경계 정의가 중요하므로, 패키지 분리가 먼저다.

## 패키지 구성

```text
.
├─ packages/
│  ├─ obsidian-plugin/
│  └─ termux-bridge/
├─ package.json
├─ pnpm-workspace.yaml
└─ tsconfig.base.json
```

## 시작

```bash
corepack enable
corepack pnpm install
corepack pnpm build
corepack pnpm typecheck
```

## 다음 단계

1. `obsidian-plugin`에 실제 커스텀 view와 설정 UI를 추가한다.
2. `termux-bridge`에 최소 헬스체크 및 세션 브리지 서버를 구현한다.
3. 플러그인과 브리지 사이의 연결 계약을 명시적인 프로토콜로 고정한다.
