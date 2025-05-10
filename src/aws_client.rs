// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! AWS Secrets Manager client implementation.
//!
//! This module provides the `AWSSecretClient` which implements the `SecretClient` trait
//! for retrieving secrets from AWS Secrets Manager.

use crate::{SecretClient, errors::SecretsManagerError};
#[cfg(test)]
use mockall::*;
#[cfg(feature = "mocks")]
use mockall::*;
use serde_json::Value;
use tracing::error;

/// Client for accessing secrets from AWS Secrets Manager.
///
/// This client stores secrets in memory after they've been retrieved from AWS
/// and provides methods to access them by key.
#[derive(Default)]
pub struct AWSSecretClient {
    /// The cached secrets as a JSON Value
    pub(crate) secrets: Value,
}

#[cfg_attr(test, automock)]
#[cfg_attr(feature = "mocks", automock)]
impl SecretClient for AWSSecretClient {
    /// Retrieves a secret from the cached secrets by its key.
    ///
    /// If the key starts with '!', the prefix is removed before lookup.
    ///
    /// # Arguments
    ///
    /// * `key` - The key identifying the secret to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The secret value as a string
    /// * `Err(SecretsManagerError::SecretNotFound)` - If the secret is not found
    fn get_by_key(&self, key: &str) -> Result<String, SecretsManagerError> {
        let key = key.strip_prefix("!").unwrap_or_default();
        let value = self.secrets[key].clone();

        let Value::String(secret) = value else {
            error!(key = key, "secret {} was not found", key);
            return Err(SecretsManagerError::SecretNotFound {});
        };

        Ok(secret)
    }
}
