# Marketplace Compliance Spec

## Metadata

- Status: accepted
- Last Updated: 2026-08-11
- Applies To: plugin metadata, settings UI, release workflow, installer

## Goal

Make the `1.0.4` release comply with the Obsidian Community automated review
errors and recommendations recorded in the current marketplace review.

## Requirements

- Plugin descriptions do not refer to Obsidian.
- Plugin unload leaves existing terminal leaves in place.
- The declared minimum application version supports every used API and exposes
  settings to settings search.
- The plugin release contains only `main.js`, `manifest.json`, and `styles.css`.
- Release assets have GitHub build-provenance attestations.
- The Termux bridge remains installable from a separately tagged bridge release.

## Compatibility

- Plugin `minAppVersion` is `1.13.0`.
- Plugin version remains `1.0.4` for this pre-release correction.
