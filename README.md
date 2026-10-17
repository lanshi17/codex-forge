# Hermes

Vendor-neutral agent core framework (library). Forked from [openai/codex](https://github.com/openai/codex), stripped of all vendor bindings: no hardcoded models, endpoints, or auth flows. Any provider, any plugin.

## What it is

- `codex-rs/core` — the agent loop (turns, tools, sessions, context). Depends only on neutral traits.
- `codex-rs/core-api` — host-facing facade: thread management, provider registration, the entry point for embedding Hermes.
- `codex-rs/ext/*` — capability plugins (mcp, skills, memories, web-search, bedrock, …), all through one interface.
- `codex-rs/model-provider` — neutral `ModelProvider` trait + `ProviderFactory` registry. Vendors self-register; core never names one.

What was removed vs upstream: `cli`, `tui`, `exec`, `app-server/*`, `cloud-*`, `chatgpt`, `code-mode-host/runtime`, `v8-poc`, `voice-host`, `windows-sandbox-service`. Entry points are yours to build; the framework is the library.

## Architecture

```text
your host  ──► core-api ──► core (agent loop)
                              │ ▲
              ┌───────────────┘ └───────────────┐
              ▼                                 ▼
  ProviderFactory registry          extension-api contributors
  (bedrock/openai/custom/oss…)      (tools/context/turn/lifecycle)
```

Core consumes credentials only as `ProviderCredential { headers, backend, uses_backend_routes, account }`. Vendor auth shapes (`CodexAuth`, OAuth, SigV4) live inside provider crates and never leak into `core`.

## Plug in a provider

```rust
use codex_model_provider::{ModelProvider, ProviderFactory, SharedModelProvider};

#[derive(Debug)]
struct MyProvider { info: ModelProviderInfo }

impl ModelProvider for MyProvider {
    fn info(&self) -> &ModelProviderInfo { &self.info }
    fn auth(&self) -> ModelProviderFuture<'_, Option<ProviderCredential>> {
        Box::pin(async move {
            // resolve your credentials, return neutral snapshot
            Some(ProviderCredential { /* … */ })
        })
    }
    fn models_manager(&self, home: PathBuf, catalog: Option<ModelsResponse>)
        -> SharedModelsManager { /* … */ }
    // defaults cover api_provider / transport / recovery
}

fn create(info: ModelProviderInfo, _auth: Option<Arc<AuthManager>>) -> SharedModelProvider {
    Arc::new(MyProvider { info })
}

// in your host binary (NOT in core):
codex_model_provider::register_provider_factory(ProviderFactory {
    // match on info, e.g. info.name == "mine" or a URL scheme
    create: |info, auth| (info.name == "mine").then(|| create(info, auth)),
});
```

Reference implementations: `ext/bedrock` (full vendor plugin), `model-provider/src/provider.rs` (`ConfiguredModelProvider` for OpenAI-compatible endpoints).

Register a catalog entry so users can select it:

```toml
# config.toml
model_provider = "mine"
[model_providers.mine]
name = "mine"
base_url = "https://my-endpoint/v1"
wire_api = "responses"
```

Built-ins are neutral only (`default`, `ollama`, `lmstudio` templates). Set `OPENAI_BASE_URL` or pass `openai_base_url` to point the `default` template at any OpenAI-compatible endpoint.

## Plug in a capability

Implement the contributor traits in `ext/extension-api` (`ToolContributor`, `ContextContributor`, `TurnLifecycleContributor`, …) and register on `ExtensionRegistryBuilder`. See `ext/mcp` and `ext/skills` for canonical examples.

## Build & verify

```shell
cd codex-rs
cargo check -p codex-core --tests          # framework + tests
cargo check --workspace \
  --exclude codex-bwrap --exclude codex-linux-sandbox   # everything (沙箱需系统 libcap)
cargo test -p codex-bedrock-provider -p codex-model-provider -p codex-model-provider-info
```

Requires a Rust toolchain per `codex-rs/rust-toolchain.toml`. No network, no accounts, no vendor keys needed for the default build.

## Origin & license

Derived from [openai/codex](https://github.com/openai/codex) at `e89b7c4`. This repository is licensed under the [Apache-2.0 License](LICENSE).
Per Apache-2.0 §4: modified files carry change notices; original copyright retained.
`Hermes` is an independent fork, not affiliated with or endorsed by OpenAI.
