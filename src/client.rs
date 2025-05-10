// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines the core `SecretClient` trait that all secret client implementations must implement.
//!
//! This trait provides the main abstraction for retrieving secrets regardless of the underlying
//! implementation (AWS, fake, etc.).

use crate::errors::SecretsManagerError;
use async_trait::async_trait;
#[cfg(test)]
use mockall::*;
#[cfg(feature = "mocks")]
use mockall::*;

/// Trait that defines the core functionality for retrieving secrets.
///
/// This trait is implemented by both the real AWS client and the fake client,
/// allowing for dependency injection and easier testing.
///
/// # Examples
///
/// ```rust
/// use secrets_manager::{SecretClient, FakeSecretClient};
///
/// fn use_client(client: &dyn SecretClient) -> Result<(), Box<dyn std::error::Error>> {
///     let secret = client.get_by_key("api-key")?;
///     // Use the secret...
///     Ok(())
/// }
///
/// // Use with fake client
/// let fake_client = FakeSecretClient::new();
/// use_client(&fake_client);
/// ```
#[cfg_attr(test, automock)]
#[cfg_attr(feature = "mocks", automock)]
#[async_trait]
pub trait SecretClient: Send + Sync {
    /// Retrieves a secret value by its key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key identifying the secret to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The secret value as a string
    /// * `Err(SecretsManagerError)` - If the secret couldn't be retrieved
    fn get_by_key(&self, key: &str) -> Result<String, SecretsManagerError>;
}
