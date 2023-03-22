/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_credential_types::provider::{error::CredentialsError, ProvideCredentials, Result as ProvideCredentialsResult};

/// Stub credentials provider for use when no credentials provider is used.
#[non_exhaustive]
#[derive(Debug)]
pub struct NoCredentials;

impl ProvideCredentials for NoCredentials {
    async fn provide_credentials(&self) -> ProvideCredentialsResult
    {
        Err(CredentialsError::not_loaded(
            "No credentials provider was enabled for the service. \
        hint: use aws-config to provide a credentials chain.",
        ))
    }
}
