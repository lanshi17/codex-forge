//! Forward explicit programs while leaving entitlement and model policy to the server.

use codex_api::AccessPrograms;
use codex_features::Feature;
use codex_model_provider::ProviderCredential;
use codex_model_provider_info::DEFAULT_PROVIDER_ID;
use codex_model_provider_info::OPENAI_PROVIDER_ID;
use codex_protocol::error::CodexErr;
use codex_protocol::error::Result;
use codex_protocol::turn_input::CyberAccessProgram;

/// Configured support for explicit Cyber programs on API-key requests.
#[derive(Clone, Copy, Debug)]
pub enum ApiKeyCyberAccessPrograms {
    UnsupportedProvider,
    Disabled,
    Enabled,
}

impl ApiKeyCyberAccessPrograms {
    /// Captures the same provider and feature policy used by ordinary model requests.
    ///
    /// Both the neutral `default` template and the legacy `openai` reserved ID
    /// select the first-party provider path; third-party providers stay
    /// unsupported.
    pub fn from_config(config: &crate::config::Config) -> Self {
        if !matches!(
            config.model_provider_id.as_str(),
            DEFAULT_PROVIDER_ID | OPENAI_PROVIDER_ID
        ) {
            Self::UnsupportedProvider
        } else if config.features.enabled(Feature::ApiKeyCyberAccessPrograms) {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

pub(crate) fn for_provider(
    provider_id: &str,
    program: Option<CyberAccessProgram>,
) -> Option<CyberAccessProgram> {
    program.filter(|_| matches!(provider_id, DEFAULT_PROVIDER_ID | OPENAI_PROVIDER_ID))
}

/// Resolves an explicit program using the request's current authentication.
pub fn for_auth(
    auth: Option<&ProviderCredential>,
    program: Option<CyberAccessProgram>,
    policy: ApiKeyCyberAccessPrograms,
) -> Result<Option<AccessPrograms>> {
    let Some(program) = program else {
        return Ok(None);
    };
    let Some(auth) = auth else {
        return Ok(None);
    };
    if auth.is_chatgpt_auth {
        return Ok(Some(program.into()));
    }
    if !auth.is_api_key {
        return Ok(None);
    }
    match policy {
        ApiKeyCyberAccessPrograms::UnsupportedProvider => Ok(None),
        ApiKeyCyberAccessPrograms::Disabled => Err(CodexErr::InvalidRequest(
            "Cyber access programs are disabled for this API-key session.".to_owned(),
        )),
        ApiKeyCyberAccessPrograms::Enabled => Ok(Some(program.into())),
    }
}
