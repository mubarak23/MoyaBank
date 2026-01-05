use breez_sdk_spark::{
    BreezSdk, ConnectRequest, Network as BreezeNetwork, ReceivePaymentMethod,
    ReceivePaymentRequest, SdkError, Seed, SendPaymentRequest, default_config,
};
use anyhow::Result;

#[derive(Debug)]
pub enum LightningError {
    ConnectionError(String),
}

pub async fn init_breeze(
         storage_dir: &str,
        mnemonic: &str,
        api_key: &str,
) -> Result<BreezSdk, Err> {
      let seed = Seed::Mnemonic {
            mnemonic: mnemonic.to_string(),
            passphrase: None,
        };
      let mut network = default_config(BreezeNetwork::Testnet);
      if let Some(key) = api_key.clone() {
            config.api_key = Some(key);
        };
    let req = ConnectRequest {
            network,
            seed,
            storage_dir: storage_dir.to_string(),
        };
      let sdk_init = BreezSdk::connect(req)
            .await
            .map_err(|e: SdkError| LightningError::ConnectionError(e.to_string()))?;
        Ok(sdk_init)
}
