# Repository Guidelines

## Project Structure & Module Organization

Warp is a Cargo workspace. The main binary lives in `app/`; shared libraries and subsystems live under `crates/`. Key crates include `crates/warpui/`, `crates/warpui_core/`, `crates/editor/`, `crates/graphql/`, `crates/warp_core/`, and `crates/integration/`. Static assets are in `images/` and `resources/`. Specs belong in `specs/GH<issue-number>/` with `product.md` and `tech.md`.

## Build, Test, and Development Commands

- `./script/bootstrap`: install platform dependencies and common agent skills.
- `./script/run` or `cargo run`: build and run Warp locally.
- `cargo run --features with_local_server`: run against a local server; set `SERVER_ROOT_URL` and `WS_SERVER_URL` for custom ports.
- `./script/presubmit`: run standard pre-PR formatting, Clippy, and tests.
- `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2`: run workspace tests with nextest.
- `cargo test --doc`: run documentation tests.

## Coding Style & Naming Conventions

Use Rust 2018 formatting from `.rustfmt.toml`; run `cargo fmt` before review. `cargo clippy --workspace --all-targets --all-features --tests -- -D warnings` must pass. Prefer imports over long path qualifiers, inline format args such as `format!("{name}")`, exhaustive `match` arms, and `ctx` as the final context parameter. Do not submit `dbg!()`. For C/C++/Obj-C under `crates/warpui/src/` or `app/src/`, run `./script/run-clang-format.py -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/`.

## Testing Guidelines

Add regression tests for bug fixes and unit tests for non-trivial logic. Place Rust unit tests in files named `foo_tests.rs` or `mod_test.rs`, then include them with `#[cfg(test)]` and `#[path = "foo_tests.rs"] mod tests;`. User-facing flows should use `crates/integration/` when practical. Manual testing is expected for local changes; use `./script/run` and capture screenshots or video for UI changes.

## Commit & Pull Request Guidelines

Recent commits use concise, imperative summaries, often with PR numbers, for example `Fix skill link in agent details panel (#11231)`. Prefix branches with your handle, such as `alice/fix-parser`. PRs should target issues labeled `ready-to-spec` or `ready-to-implement`, use `.github/pull_request_template.md`, describe testing, include UI screenshots, and add `CHANGELOG-NEW-FEATURE`, `CHANGELOG-IMPROVEMENT`, `CHANGELOG-BUG-FIX`, or `CHANGELOG-NONE` when appropriate.

## Agent-Specific Instructions

Read `WARP.md` before modifying code; it contains architecture notes, UI rules, terminal model locking cautions, and feature flag guidance. This fork tracks fast-moving upstream, so keep customizations isolated and upstream-friendly. Prefer plugin-like extension points, config, adapters, or small wrappers over broad core edits. Ask before large, invasive, or hard-to-rebase changes. Prefer `.agents/skills/` workflows for feature flags, telemetry, UI, Rust tests, and integration tests. When relaying command results or console output to users, use Chinese whenever practical; preserve exact commands, paths, and errors when needed, with Chinese explanation.
