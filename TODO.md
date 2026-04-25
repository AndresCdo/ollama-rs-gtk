# TODO

This list is intentionally limited to items verified from the current repository state on April 25, 2026.

## Completed In Current Worktree

- Fix Ollama error handling so non-2xx API responses surface the actual server error instead of `Error: No response`.
- Support chat history during the app session by using the Ollama chat API by default.
- Allow the app to run without a `config.toml` and provide a checked-in `config.toml.example`.
- Document missing Linux build prerequisites for GtkSourceView 4 and WebKitGTK.

## Verified Next Work

- Add integration tests for the Ollama HTTP layer.
  Reason: current coverage is limited to unit tests in `src/api.rs`, `src/config.rs`, and `src/utils.rs`; there are no request/response tests against a mock server for `/api/chat`, `/api/generate`, or non-2xx error bodies.

- Add UI-level tests or a repeatable manual test script for chat history and error rendering.
  Reason: the GTK event flow in `src/ui.rs` is not exercised by tests, and the current validation was code-level plus CLI checks only.

- Decide whether to keep or replace the `cargo bench` CI job.
  Reason: `cargo bench` currently performs a full optimized build but there are no custom benchmarks in the repository, so it adds cost without measuring app behavior.

- Expand installation guidance beyond Debian/Ubuntu package names.
  Reason: the README now documents the required system libraries, but only Debian/Ubuntu package names are provided even though the project describes itself as cross-platform.
