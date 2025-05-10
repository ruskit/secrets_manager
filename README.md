# Secrets Manager

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A Rust library for securely retrieving and managing secrets from AWS Secrets Manager.

## Overview

This crate provides a trait-based abstraction for secret clients with implementations for AWS Secrets Manager and a fake client for testing/development purposes. It's designed to be easy to use, type-safe, and to follow best practices for Rust libraries.

## Features

- **Trait-based API design**: Flexible and testable with dependency injection
- **AWS Secrets Manager integration**: Secure retrieval of secrets from AWS
- **Mock implementations**: Facilitates testing without requiring actual AWS credentials
- **Comprehensive error handling**: Clear and specific error types
- **Tracing integration**: Detailed logging through the `tracing` crate

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
secrets-manager = "0.0.1"
```

## Usage

### Basic usage with AWS Secrets Manager

```rust
use secrets_manager::{AWSSecretClientBuilder, SecretClient};

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an AWS Secrets Manager client
    let client = AWSSecretClientBuilder::new("my-secret-name".to_string())
        .build()
        .await?;
    
    // Retrieve a secret by key
    let db_password = client.get_by_key("database-password")?;
    
    // Use the secret...
    
    Ok(())
}
```

### Using the Fake Client for testing

```rust
use secrets_manager::{SecretClient, FakeSecretClient};

fn test_component_with_secrets() {
    // Create a fake client that doesn't need AWS
    let client = FakeSecretClient::new();
    
    // Use it in your component that needs secrets
    let component = MyComponent::new(client);
    
    // Test your component...
}
```

### Dependency Injection

```rust
use secrets_manager::SecretClient;

struct Database {
    client: Box<dyn SecretClient>,
    // other fields...
}

impl Database {
    fn new(client: Box<dyn SecretClient>) -> Self {
        Database {
            client,
            // initialize other fields...
        }
    }
    
    fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let password = self.client.get_by_key("db-password")?;
        // Use password to connect...
        Ok(())
    }
}
```

## Feature Flags

- `mocks` - Enables mock implementations for testing (requires `mockall`)

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

## License

This project is licensed under the [MIT License](LICENSE).