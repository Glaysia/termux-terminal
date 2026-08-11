# Marketplace Release Preparation Spec

## Metadata

- Status: accepted
- Last Updated: 2026-08-11
- Applies To: public documentation, release metadata, GitHub Actions

## Goal

Prepare the first public Termux Terminal release for an Obsidian Community
directory submission without claiming support that has not been validated.

## Scope

- Rewrite public README files around installation, operation, security, and
  support boundaries.
- Add contributor, security, and changelog documentation.
- Make plugin, package, and bridge release versions mechanically consistent.
- Validate the exact assets required by an Obsidian GitHub Release.

## Non-Goals

- Change the bridge ABI or complete the pending Android Bionic migration.
- Publish a release, submit the directory form, or modify `main` directly.

## Acceptance Criteria

- Public docs describe native Termux only and do not present `proot` as a
  product runtime or installation requirement.
- A single semantic version is checked across all release metadata.
- Release automation rejects a tag that does not match `manifest.json` and
  attaches `main.js`, `manifest.json`, and `styles.css`.
