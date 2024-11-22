use kraken_async_rs::secrets::secrets_provider::{
    EnvSecretsProvider, SecretsProvider, StaticSecretsProvider,
};
use std::sync::Arc;
use tokio::sync::Mutex;

const NULL_KEY: &str =
    "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
const NULL_SECRET: &str =
    "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";

pub fn get_null_secrets_provider<'a>() -> Box<Arc<Mutex<dyn SecretsProvider>>> {
    Box::new(Arc::new(Mutex::new(StaticSecretsProvider::new(
        NULL_KEY,
        NULL_SECRET,
    ))))
}
