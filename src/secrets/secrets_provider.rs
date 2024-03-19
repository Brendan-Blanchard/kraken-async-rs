//! Trait and implementations for retrieving API keys and secrets needed for private calls
use dotenvy::dotenv;
use secrecy::Secret;
use std::env;

/// A struct containing the API key and secret (using [secrecy::Secret])
#[derive(Clone)]
pub struct Secrets {
    pub key: Secret<String>,
    pub secret: Secret<String>,
}

/// Trait that exposes a method for retrieving secrets.
///
/// Clients are generic over [SecretsProvider] so the client can specify how to retrieve the API
/// key and secret at runtime.
pub trait SecretsProvider: Send + Sync {
    fn get_secrets(&mut self) -> Secrets;
}

/// A common implementation that retrieves the key and secret from the given environment variable names.
///
/// This retrieves secrets once from the environment and caches them. If your use case requires
/// retrieving them each time, a custom implementation may be your best choice.
pub struct EnvSecretsProvider<'a> {
    key_name: &'a str,
    secret_name: &'a str,
    secrets: Option<Secrets>,
}

impl<'a> EnvSecretsProvider<'a> {
    /// Creates an instance that will retrieve secrets by environment variables, looking for `key_name`
    /// and `secret_name`.
    pub fn new(key_name: &'a str, secret_name: &'a str) -> EnvSecretsProvider<'a> {
        EnvSecretsProvider {
            key_name,
            secret_name,
            secrets: None,
        }
    }
}

impl<'a> SecretsProvider for EnvSecretsProvider<'a> {
    fn get_secrets(&mut self) -> Secrets {
        if self.secrets.is_none() {
            self.set_secrets();
        }

        self.secrets.clone().unwrap()
    }
}

impl<'a> EnvSecretsProvider<'a> {
    fn set_secrets(&mut self) {
        dotenv().ok();
        let key = Secret::new(match env::var(self.key_name) {
            Ok(secret) => secret,
            _ => panic!("Error retrieving Kraken key from env"),
        });

        let secret = Secret::new(match env::var(self.secret_name) {
            Ok(secret) => secret,
            _ => panic!("Error retrieving Kraken secret from env"),
        });

        self.secrets = Some(Secrets { key, secret });
    }
}

/// A [SecretsProvider] that stores the key and secret directly. This is useful if you don't wish
/// to provide a custom implementation, and will directly instantiate a [StaticSecretsProvider] with
/// your key and secret.
///
/// *This is not recommended for use outside of testing!* It is relatively unsafe to store the key
/// and secret as plain text outside of secrecy, and would be downright unsafe to store the key
/// and secret in source-code by directly creating a [StaticSecretsProvider] with `'static` strings.
pub struct StaticSecretsProvider<'a> {
    key: &'a str,
    secret: &'a str,
}

impl<'a> StaticSecretsProvider<'a> {
    pub fn new(key: &'a str, secret: &'a str) -> StaticSecretsProvider<'a> {
        StaticSecretsProvider { key, secret }
    }
}

impl<'a> SecretsProvider for StaticSecretsProvider<'a> {
    fn get_secrets(&mut self) -> Secrets {
        Secrets {
            key: Secret::new(self.key.to_string()),
            secret: Secret::new(self.secret.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::secrets::secrets_provider::{EnvSecretsProvider, SecretsProvider};
    use secrecy::ExposeSecret;

    #[test]
    fn test_env_secrets_provider() {
        let key_name = "TEST_KEY";
        let secret_name = "TEST_SECRET";
        let key = "api-key";
        let secret = "api-secret";

        std::env::set_var(key_name, key);
        std::env::set_var(secret_name, secret);

        let mut secrets_provider = EnvSecretsProvider::new(key_name, secret_name);

        let secrets = secrets_provider.get_secrets();

        assert_eq!(key, secrets.key.expose_secret());
        assert_eq!(secret, secrets.secret.expose_secret());
    }
}
