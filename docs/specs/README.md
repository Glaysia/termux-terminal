# Change Specs

## Metadata

- Status: accepted
- Owners: repository maintainers
- Last Updated: 2026-04-07
- Applies To: entire repository
- Related Docs: `docs/sdd/README.md`, `docs/foundation/README.md`

## Purpose

This folder contains initiative-specific SDD documents.

Use this folder for change-scoped docs.

Use `docs/foundation/` for long-lived project truths.

## Folder Contract

Every non-trivial initiative must live in its own folder:

- `docs/specs/<slug>/spec.md`
- `docs/specs/<slug>/plan.md`
- `docs/specs/<slug>/tasks.md`

Do not add extra canonical files at the top level of `docs/specs/`.

## Active Specs

- [termux-bridge-v1](termux-bridge-v1/spec.md)
- [obsidian-terminal-workbench](obsidian-terminal-workbench/spec.md)
- [public-release-preparation](public-release-preparation/spec.md)
