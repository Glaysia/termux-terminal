# Marketplace Release Preparation Tasks

- [x] Define public documentation and release-asset scope.
- [ ] Authenticate GitHub CLI and push the feature branch.
- [ ] Open and merge `feat/marketplace-release-prep` into `dev`.
- [ ] Open `dev` into `main` PR after the public release checklist passes.
- [ ] Create the matching GitHub Release before directory submission.
- [ ] Submit the plugin through the Obsidian Community directory.
- [x] Diagnose the failed `v1.0.2` Release workflow: the GitHub Rust toolchain
  lacked the musl target.
- [x] Add a pull-request CI workflow that validates the native musl release
  build before merge.
- [ ] Publish `v1.0.3` after the corrected workflow is merged.
