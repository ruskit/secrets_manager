// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! AWS Secret client builder module.
//!
//! This module provides the `AWSSecretClientBuilder` for constructing instances of
//! `AWSSecretClient` with appropriate configuration.

use crate::{AWSSecretClient, errors::SecretsManagerError};
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager as secretsmanager;
#[cfg(test)]
use mockall::*;
#[cfg(feature = "mocks")]
use mockall::*;
use secretsmanager::Client;
use tracing::error;

/// Builder for AWS Secret Client instances.
///
/// This builder facilitates the creation of `AWSSecretClient` instances
/// by handling the AWS SDK configuration and secret retrieval.
#[derive(Default)]
pub struct AWSSecretClientBuilder {
    /// The secret key to retrieve from AWS Secrets Manager
    secret_key: String,
}

#[cfg_attr(test, automock)]
#[cfg_attr(feature = "mocks", automock)]
impl AWSSecretClientBuilder {
    /// Creates a new builder with the specified secret key.
    ///
    /// # Arguments
    ///
    /// * `secret_key` - The name/id of the secret to retrieve from AWS Secrets Manager
    ///
    /// # Returns
    ///
    /// A new instance of `AWSSecretClientBuilder`
    pub fn new(secret_key: String) -> AWSSecretClientBuilder {
        AWSSecretClientBuilder { secret_key }
    }

    /// Returns the secret ID to be used in AWS API calls.
    ///
    /// # Returns
    ///
    /// The configured secret ID as a String
    fn secret_id(&self) -> String {
        self.secret_key.to_string()
    }

    /// Builds and returns an `AWSSecretClient` instance.
    ///
    /// This asynchronous method:
    /// 1. Configures the AWS SDK
    /// 2. Creates a Secrets Manager client
    /// 3. Retrieves the secret
    /// 4. Parses the secret JSON
    /// 5. Creates an `AWSSecretClient` with the parsed secrets
    ///
    /// # Returns
    ///
    /// * `Ok(AWSSecretClient)` - If the secret was successfully retrieved and parsed
    /// * `Err(SecretsManagerError)` - If any step failed
    pub async fn build(&self) -> Result<AWSSecretClient, SecretsManagerError> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        let id = self.secret_id();

        let output = match client.get_secret_value().secret_id(&id).send().await {
            Err(err) => {
                error!(
                    error = err.to_string(),
                    "failure send request to secret manager"
                );
                Err(SecretsManagerError::RequestFailure {})
            }
            Ok(s) => Ok(s),
        }?;

        let Some(string) = output.secret_string() else {
            error!("secret was not found");
            return Err(SecretsManagerError::AwsSecretWasNotFound {});
        };

        match serde_json::from_str(string) {
            Err(err) => {
                error!(error = err.to_string(), "error mapping secrets");
                Err(SecretsManagerError::InternalError {})
            }
            Ok(v) => Ok(AWSSecretClient { secrets: v }),
        }
    }
}
