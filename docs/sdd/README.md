# Spec-Driven Development

## Metadata

- Status: accepted
- Owners: repository maintainers
- Last Updated: 2026-04-07
- Applies To: entire repository
- Related Docs: `docs/foundation/README.md`, `docs/specs/README.md`, `README.md`, `AGENTS.md`

## Purpose

This repository uses spec-driven development for all non-trivial work.

The purpose of SDD here is simple:

- define intent before implementation
- keep long-lived contracts separate from change-specific plans
- make it obvious which docs must be updated when behavior changes
- give humans and agents the same source of truth before coding starts

## Artifact Types

This repo uses two artifact classes.

### Foundation Docs

Foundation docs capture long-lived truths that multiple changes can depend on.

Canonical location:

- `docs/foundation/`

Examples:

- architecture
- protocol
- environment and runtime assumptions when they are project-wide

Foundation docs should change only when the underlying contract or assumption changes.

### Change Specs

Change specs capture the intent and execution path for one initiative.

Canonical location:

- `docs/specs/<slug>/`

Every non-trivial initiative must have exactly these files:

- `spec.md`
- `plan.md`
- `tasks.md`

Folder names must use lowercase kebab-case.

Examples:

- `docs/specs/termux-bridge-v1/`
- `docs/specs/plugin-terminal-view/`

## Required Metadata Block

Every template-backed SDD document must begin with a plain Markdown metadata block, not YAML frontmatter.

Required fields:

- `Status`
- `Owners`
- `Last Updated`
- `Applies To`
- `Related Docs`

Allowed status values:

- `draft`
- `accepted`
- `in-progress`
- `done`
- `superseded`

## Workflow

Use this sequence for non-trivial work:

1. create or update `docs/specs/<slug>/`
2. write `spec.md` before implementation starts
3. write `plan.md` after the spec is stable enough to implement
4. write `tasks.md` as the ordered execution checklist
5. update impacted foundation docs in the same change when contracts or assumptions move
6. update document statuses as the work moves from `draft` to `done`

## Source-Of-Truth Rules

- no implementation starts without a spec folder unless the change is explicitly trivial
- the spec is the source of truth for intent, scope, constraints, and acceptance criteria
- the plan is the source of truth for implementation shape and subsystem decisions
- tasks are execution-oriented and must be derived from the plan
- protocol, architecture, and environment changes must update the relevant foundation docs in the same change

## Trivial Changes

The following changes are exempt from creating a new spec folder:

- typo fixes
- link fixes
- wording-only documentation edits
- non-behavioral formatting changes

Anything that changes behavior, APIs, contracts, architecture, runtime assumptions, or delivery order requires a spec folder.

## Document Ownership

Use `Owners` to record who is responsible for keeping a document current.

Recommended values in this repo:

- `repository maintainers`
- `termux-bridge maintainers`
- `obsidian-plugin maintainers`

Use `Applies To` to name affected paths, packages, or runtime surfaces.

## How To Use The Templates

- start from `docs/sdd/templates/spec.md` for a new initiative
- add `plan.md` only after the spec is stable enough to guide implementation
- keep `tasks.md` small, ordered, and completion-oriented
- link foundation docs in `Related Docs` instead of duplicating contract text

## Migration Notes

Legacy top-level `docs/` files remain as temporary compatibility stubs for one transition cycle.

Canonical content now lives under:

- `docs/foundation/`
- `docs/specs/`
