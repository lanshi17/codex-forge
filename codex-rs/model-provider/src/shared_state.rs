use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;

use codex_keyring_store::DefaultKeyringStore;
use codex_login::AuthRuntimeConfig;
use codex_login::GatewayAuthConfig;
use codex_login::GatewayAuthManager;
use codex_model_provider_info::GatewayOAuthConfig;

/// Provider-owned runtime state shared across independently configured sessions.
#[derive(Debug, Default)]
pub struct ModelProviderSharedState {
    gateway_managers: Mutex<
        Vec<(
            GatewayAuthConfig,
            AuthRuntimeConfig,
            Weak<GatewayAuthManager>,
        )>,
    >,
}

impl ModelProviderSharedState {
    pub fn gateway_auth(
        &self,
        config: &GatewayOAuthConfig,
        runtime: &AuthRuntimeConfig,
    ) -> std::io::Result<Arc<GatewayAuthManager>> {
        let oauth = GatewayAuthConfig {
            authorization_url: config.authorization_url.clone(),
            token_url: config.token_url.clone(),
            client_id: config.client_id.clone(),
            resource: config.resource.clone(),
            scopes: config.scopes.clone(),
            redirect_port: config.redirect_port,
        };
        let mut managers = self
            .gateway_managers
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        managers.retain(|(_, _, manager)| manager.strong_count() != 0);
        if let Some(manager) = managers
            .iter()
            .find(|(cached_config, cached_runtime, _)| {
                cached_config == &oauth && cached_runtime == runtime
            })
            .and_then(|(_, _, manager)| manager.upgrade())
        {
            return Ok(manager);
        }
        let manager = Arc::new(GatewayAuthManager::new(
            oauth.clone(),
            runtime.codex_home.clone(),
            runtime.auth_route_config.http_client_factory(),
            Arc::new(DefaultKeyringStore),
        )?);
        managers.push((oauth, runtime.clone(), Arc::downgrade(&manager)));
        Ok(manager)
    }
}

static PROCESS_SHARED_STATE: std::sync::LazyLock<ModelProviderSharedState> =
    std::sync::LazyLock::new(ModelProviderSharedState::default);

pub fn process_shared_state() -> &'static ModelProviderSharedState {
    &PROCESS_SHARED_STATE
}
#[cfg(test)]
#[path = "shared_state_tests.rs"]
mod tests;

#[path = "shared_state_test_support.rs"]
pub(crate) mod test_support;
