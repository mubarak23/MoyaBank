use breez_sdk_spark::connect;
use breez_sdk_spark::{
    default_config, BreezSdk, ConnectRequest, Network as BreezeNetwork, Network,
    ReceivePaymentMethod, ReceivePaymentRequest, SdkError, Seed, SendPaymentRequest,
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
) -> Result<BreezSdk, LightningError> {
    let seed = Seed::Mnemonic {
        mnemonic: mnemonic.to_string(),
        passphrase: None,
    };
    let mut config = default_config(Network::Mainnet);
    config.api_key = Some(api_key.to_string());

    let req = ConnectRequest {
        config,
        seed,
        storage_dir: storage_dir.to_string(),
    };
    let sdk_init = connect(req)
        .await
        .map_err(|e: SdkError| LightningError::ConnectionError(e.to_string()))?;
    Ok(sdk_init)
}
