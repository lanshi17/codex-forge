//! Neutral provider credentials consumed by `core`.
//!
//! `core` must not depend on vendor auth shapes (`CodexAuth`). Providers
//! convert their internal auth into [`ProviderCredential`] at the
//! `ModelProvider::auth()` boundary; `core` only reads the neutral fields.

use codex_login::CodexAuth;
use codex_protocol::account::PlanType;
use codex_protocol::auth::AuthMode;
use http::HeaderMap;

/// Backend family backing a credential snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    /// First-party OpenAI-compatible backend (ChatGPT/Codex auth, API keys,
    /// header auth, agent identity, personal access tokens).
    OpenAiCompatible,
    /// Amazon Bedrock backend.
    Bedrock,
    /// Any other third-party/custom backend.
    Custom,
}

/// Neutral identity attached to a credential snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AccountIdentity {
    pub account_id: Option<String>,
    pub chatgpt_user_id: Option<String>,
    pub email: Option<String>,
    pub plan_type: Option<PlanType>,
    pub is_workspace: bool,
}

/// Neutral credential snapshot returned by `ModelProvider::auth()`.
///
/// Headers carry request credentials (API key / bearer token materialized by
/// the provider). Boolean/enum fields drive routing and feature gating that
/// previously matched on `CodexAuth` variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderCredential {
    /// Request credential headers materialized by the provider.
    pub headers: HeaderMap,
    /// Backend family this credential targets.
    pub backend: BackendKind,
    /// Whether this credential rides first-party Codex backend routes.
    pub uses_backend_routes: bool,
    /// Effective auth mode for telemetry and request setup.
    pub auth_mode: Option<AuthMode>,
    /// Whether the credential is an API-key credential.
    pub is_api_key: bool,
    /// Whether the credential is ChatGPT-managed (ChatGPT/ChatgptAuthTokens).
    pub is_chatgpt_managed: bool,
    /// Whether the credential carries a ChatGPT account.
    pub is_chatgpt_auth: bool,
    /// Optional account identity for cache keys and telemetry.
    pub account: Option<AccountIdentity>,
}

impl ProviderCredential {
    /// Bearer token materialized in the credential headers, if any.
    pub fn bearer_token(&self) -> Option<&str> {
        self.headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
    }

    /// API key embedded in the credential headers, if any.
    pub fn api_key(&self) -> Option<&str> {
        self.is_api_key.then(|| self.bearer_token()).flatten()
    }

    pub fn uses_codex_backend(&self) -> bool {
        self.uses_backend_routes
    }

    pub fn account_id(&self) -> Option<&str> {
        self.account.as_ref().and_then(|a| a.account_id.as_deref())
    }

    pub fn chatgpt_user_id(&self) -> Option<&str> {
        self.account
            .as_ref()
            .and_then(|a| a.chatgpt_user_id.as_deref())
    }

    pub fn is_workspace_account(&self) -> bool {
        self.account.as_ref().is_some_and(|a| a.is_workspace)
    }
}

impl From<&CodexAuth> for ProviderCredential {
    fn from(auth: &CodexAuth) -> Self {
        let backend = match auth {
            CodexAuth::BedrockApiKey(_) | CodexAuth::BedrockAccessKeys(_) => BackendKind::Bedrock,
            _ => BackendKind::OpenAiCompatible,
        };
        let headers = crate::auth::auth_provider_from_auth(auth).to_auth_headers();
        let account_id = auth.get_account_id();
        let chatgpt_user_id = auth.get_chatgpt_user_id();
        let email = auth.get_account_email();
        let plan_type = auth.account_plan_type();
        let is_workspace = plan_type.is_some_and(|plan| plan.is_workspace_account());
        let has_identity = account_id.is_some()
            || chatgpt_user_id.is_some()
            || email.is_some()
            || plan_type.is_some();
        Self {
            headers,
            backend,
            uses_backend_routes: auth.uses_codex_backend(),
            auth_mode: Some(auth.auth_mode()),
            is_api_key: auth.is_api_key_auth(),
            is_chatgpt_managed: matches!(
                auth,
                CodexAuth::Chatgpt(_) | CodexAuth::ChatgptAuthTokens(_)
            ),
            is_chatgpt_auth: auth.is_chatgpt_auth(),
            account: has_identity.then(|| AccountIdentity {
                account_id,
                chatgpt_user_id,
                email,
                plan_type,
                is_workspace,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_credential_maps_headers_and_flags() {
        let auth = CodexAuth::from_api_key("sk-test");
        let credential = ProviderCredential::from(&auth);
        assert_eq!(credential.backend, BackendKind::OpenAiCompatible);
        assert!(!credential.uses_backend_routes);
        assert_eq!(credential.auth_mode, Some(AuthMode::ApiKey));
        assert!(credential.is_api_key);
        assert_eq!(credential.api_key(), Some("sk-test"));
    }

    #[test]
    fn chatgpt_testing_credential_uses_backend_routes() {
        let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
        let credential = ProviderCredential::from(&auth);
        assert!(credential.uses_backend_routes);
        assert!(credential.is_chatgpt_managed);
    }
}
