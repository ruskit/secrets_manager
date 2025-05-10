// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Fake secret client implementation for testing and development.
//!
//! This module provides a `FakeSecretClient` that implements the `SecretClient` trait
//! but returns empty strings instead of actual secrets. Useful for testing and
//! development environments.

use crate::{SecretClient, errors::SecretsManagerError};

/// A fake implementation of the `SecretClient` trait for testing.
///
/// This client always returns an empty string for any secret key, making it
/// useful for tests and development environments where actual secrets
/// are not required or available.
#[derive(Default)]
pub struct FakeSecretClient;

impl SecretClient for FakeSecretClient {
    /// Always returns an empty string for any key.
    ///
    /// # Arguments
    ///
    /// * `_key` - The key is ignored in this implementation
    ///
    /// # Returns
    ///
    /// Always `Ok(String::new())`
    fn get_by_key(&self, _key: &str) -> Result<String, SecretsManagerError> {
        Ok(String::new())
    }
}

impl FakeSecretClient {
    /// Creates a new `FakeSecretClient` instance.
    ///
    /// # Returns
    ///
    /// A new `FakeSecretClient`
    pub fn new() -> FakeSecretClient {
        FakeSecretClient {}
    }
}
