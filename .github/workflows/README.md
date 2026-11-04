# Workflow Strategy

The workflows in this directory are split so that pull requests get fast, review-friendly signal while `main` still gets the full cross-platform verification pass.

## Independent Rust workspaces

`forge-ci.yml` checks `btae-eval` and `superagent-forge` on Ubuntu 24.04 with
Rust 1.95.0: repository formatting rules, Clippy with warnings denied, all-target
tests, and a clean worktree. BTAE tests require working Bubblewrap namespaces;
the setup probe fails the job if they are unavailable. Model credentials and
live model calls are not used in CI.

The workflow is a required dependency of `blocking-ci` alongside the retained
upstream checks. It can also be dispatched manually for diagnosis; a successful
manual run alone does not satisfy the full merge gate. See [CI qualification](../../docs/CI.md).

## Hosted runners

CI and release workflows use GitHub-hosted runners instead of deriving private
runner groups or labels from the repository name. No `codex-forge-runners`
group needs to be provisioned.

| Platform      | Hosted label       |
| ------------- | ------------------ |
| Linux x64     | `ubuntu-24.04`     |
| Linux ARM64   | `ubuntu-24.04-arm` |
| Windows x64   | `windows-latest`   |
| Windows ARM64 | `windows-11-arm`   |
| macOS ARM64   | `macos-15`         |
| macOS Intel   | `macos-15-intel`   |

Windows ARM64 nextest archives are still built on x64 and executed on ARM64.
Build targets, test shards, commands, and required dependencies are unchanged.
Reusable nextest workflows retain optional runner-group inputs for callers
that explicitly provision their own infrastructure; this repository does not
pass those inputs. Standard hosted machines may take longer on cold builds
than the previous private runners. macOS ARM64 uses the standard `macos-15`
image instead of the paid `macos-15-xlarge` runner. Existing Intel runner
selections and ARM64-hosted cross-build targets are retained. The matching
architectures are listed in the [official runner image inventory](https://github.com/actions/runner-images#available-images).

Windows Bazel targets that materialize the MSVC runtime require explicit
Visual Studio license acknowledgement. After the repository owner confirms
acceptance of the [Microsoft terms](https://visualstudio.microsoft.com/license-terms),
an administrator may set repository Actions variable
`BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA` to `1`. Windows CI workflows pass that
variable explicitly to `prepare-bazel-ci` through its `msvc-runtime-eula`
input, including the nested argument-comment lint action. The action exports
the acknowledgement only on Windows. An absent or different value leaves the
existing license check closed; Linux and macOS receive no acknowledgement.
No workflow accepts those terms automatically.

Hosted Windows Bazel and argument-comment lint jobs opt into pinned voice tool
provisioning through `windows-voice-tools`. The existing offline installer
verifies Cygwin packages and pkgconf; archives come from the hash-pinned public
`openai/codex` release. Local Windows builds receive the declared tool repository
and fixed `SystemRoot` and host architecture values. Windows paths are not passed
to Linux remote build actions. Tool installations stay in the runner's temporary
directory and are not added to release packages.

## Pull Requests

- Required checks run against GitHub's synthetic merge commit, not the pull
  request head alone. This includes changes already on `main` and catches
  conflicts before they reach the branch.
- `bazel.yml` is the main pre-merge verification path for Rust code.
  It runs Bazel `test` and Bazel `clippy` on the supported Bazel targets,
  including the generated Rust test binaries needed to lint inline `#[cfg(test)]`
  code.
- `rust-ci.yml` keeps the Cargo-native PR checks intentionally small:
  - `cargo fmt --check`
  - `cargo shear`
  - `argument-comment-lint` on Linux, macOS, and Windows
  - `tools/argument-comment-lint` package tests when the lint or its workflow wiring changes

## Post-Merge On `main`

- `bazel.yml` also runs on pushes to `main`.
  This re-verifies the merged Bazel path and helps keep the BuildBuddy caches warm.
- `rust-ci-full.yml` is the full Cargo-native verification workflow.
  It keeps the heavier checks off the PR path while still validating them after merge:
  - the full Cargo `clippy` matrix
  - the full Cargo `nextest` matrix via per-platform archive-backed shards
  - Windows ARM64 nextest archives cross-compiled on Windows x64, then replayed on native Windows ARM64 shards
  - release-profile Cargo builds
  - cross-platform `argument-comment-lint`
  - Linux remote-env tests

## Rule Of Thumb

- If a build/test/clippy check can be expressed in Bazel, prefer putting the PR-time version in `bazel.yml`.
- Keep `rust-ci.yml` fast enough that it usually does not dominate PR latency.
- Reserve `rust-ci-full.yml` for heavyweight Cargo-native coverage that Bazel does not replace yet.
