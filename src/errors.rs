// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Error types for the secrets manager module.
//!
//! This module defines the various error types that can occur when working with
//! secrets and AWS Secrets Manager.

use thiserror::Error;

/// Represents errors that can occur during secret management operations.
///
/// This enum provides specific error variants for different failure scenarios
/// when interacting with secrets and the AWS Secrets Manager service.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum SecretsManagerError {
    /// An internal error occurred in the secrets manager
    #[error("internal error")]
    InternalError,

    /// Failed to send a request to the secrets service
    #[error("failure to send request")]
    RequestFailure,

    /// The requested secret was not found in the local cache
    #[error("secret not found")]
    SecretNotFound,

    /// The requested secret was not found in AWS Secrets Manager
    #[error("aws secret was not found")]
    AwsSecretWasNotFound,
}
