//! Amazon Bedrock model provider plugin.
//!
//! This crate owns the Bedrock-specific [`ModelProvider`] implementation.
//! `codex-model-provider` stays vendor-neutral and consults a provider
//! factory registry in [`create_model_provider`]; this crate registers its
//! factory via `ctor` so linking the crate preserves Bedrock behavior.

mod aws_shared_state;
mod bedrock;

use std::sync::Arc;

use codex_login::AuthManager;
use codex_model_provider::SharedModelProvider;
use codex_model_provider_info::ModelProviderInfo;

pub use aws_shared_state::process_bedrock_shared_state;
pub use bedrock::AmazonBedrockModelProvider;
pub use bedrock::is_amazon_bedrock_gov_cloud_region;
pub use bedrock::is_supported_amazon_bedrock_region;
pub use bedrock::resolve_amazon_bedrock_region;

/// Constructs the Bedrock provider when `provider_info` selects Bedrock.
///
/// Returns `None` for non-Bedrock providers so the registry can fall through
/// to the next factory or the neutral default.
pub fn create_bedrock_provider(
    provider_info: ModelProviderInfo,
    auth_manager: Option<Arc<AuthManager>>,
) -> Option<SharedModelProvider> {
    if !provider_info.is_amazon_bedrock() {
        return None;
    }
    Some(Arc::new(AmazonBedrockModelProvider::new(
        provider_info,
        auth_manager,
    )))
}

/// Registers the Bedrock provider factory with `codex-model-provider`.
pub fn register() {
    codex_model_provider::register_provider_factory(create_bedrock_provider);
}

#[ctor::ctor]
fn register_bedrock_provider_factory() {
    register();
}
