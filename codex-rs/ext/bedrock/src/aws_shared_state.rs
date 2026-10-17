//! Process-wide cache for Bedrock AWS credential state.
//!
//! The neutral `ModelProviderSharedState` cannot own Bedrock's AWS types
//! without reintroducing the vendor dependency this crate extraction removes.

use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::Weak;

use codex_model_provider_info::ModelProviderAwsAuthInfo;

use crate::bedrock::AwsAuthRecovery;
use crate::bedrock::AwsCredentialExport;

/// Process-wide Bedrock AWS credential state.
#[derive(Debug, Default)]
pub struct BedrockSharedState {
    credential_exports: Mutex<Vec<(ModelProviderAwsAuthInfo, Weak<AwsCredentialExport>)>>,
    auth_recoveries: Mutex<Vec<(ModelProviderAwsAuthInfo, Weak<AwsAuthRecovery>)>>,
}

static BEDROCK_SHARED_STATE: LazyLock<BedrockSharedState> =
    LazyLock::new(BedrockSharedState::default);

/// Returns the process-wide Bedrock AWS credential state.
pub fn process_bedrock_shared_state() -> &'static BedrockSharedState {
    &BEDROCK_SHARED_STATE
}

impl BedrockSharedState {
    /// Returns the shared credential exporter for `aws`, creating it on first use.
    pub fn aws_credential_export(
        &self,
        aws: &ModelProviderAwsAuthInfo,
    ) -> Option<Arc<AwsCredentialExport>> {
        let config = aws.credential_export.as_ref()?;
        let mut exports = self
            .credential_exports
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        exports.retain(|(_, export)| export.strong_count() != 0);

        if let Some(export) = exports
            .iter()
            .find(|(cached_aws, _)| cached_aws == aws)
            .and_then(|(_, export)| export.upgrade())
        {
            return Some(export);
        }

        let export = Arc::new(AwsCredentialExport::new(config.clone()));
        exports.push((aws.clone(), Arc::downgrade(&export)));
        Some(export)
    }

    /// Returns the shared AWS auth recovery for `aws`, creating it on first use.
    pub fn aws_auth_recovery(
        &self,
        aws: &ModelProviderAwsAuthInfo,
    ) -> Option<Arc<AwsAuthRecovery>> {
        let config = aws.auth_refresh.as_ref()?;
        let mut recoveries = self
            .auth_recoveries
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        recoveries.retain(|(_, recovery)| recovery.strong_count() != 0);

        if let Some(recovery) = recoveries
            .iter()
            .find(|(cached_aws, _)| cached_aws == aws)
            .and_then(|(_, recovery)| recovery.upgrade())
        {
            return Some(recovery);
        }

        let recovery = Arc::new(AwsAuthRecovery::new(config.clone()));
        recoveries.push((aws.clone(), Arc::downgrade(&recovery)));
        Some(recovery)
    }

    #[cfg(test)]
    pub(crate) fn aws_auth_recovery_for_test(
        &self,
        aws: &ModelProviderAwsAuthInfo,
    ) -> Option<Arc<AwsAuthRecovery>> {
        self.aws_auth_recovery(aws)
    }
}
