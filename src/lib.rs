// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Secrets Manager
//!
//! A Rust library for securely retrieving and managing secrets from AWS Secrets Manager.
//! This crate provides a trait-based abstraction for secret clients with both AWS
//! implementation and a fake client for testing/development purposes.
//!
//! ## Features
//!
//! - Trait-based API design for flexibility and testability
//! - AWS Secrets Manager integration
//! - Mock implementations for testing
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust
//! use secrets_manager::{AWSSecretClientBuilder, SecretClient};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an AWS Secrets Manager client
//!     let client = AWSSecretClientBuilder::new("my-secret-key".to_string())
//!         .build()
//!         .await?;
//!     
//!     // Retrieve a secret by key
//!     let secret = client.get_by_key("database-password")?;
//!     
//!     Ok(())
//! }
//! ```

mod aws_client;
mod aws_client_builder;
mod client;
mod fake_client;

pub mod errors;
pub use aws_client::AWSSecretClient;
pub use aws_client_builder::AWSSecretClientBuilder;
pub use client::SecretClient;
pub use fake_client::FakeSecretClient;
