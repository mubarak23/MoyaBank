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

pub(crate) struct SdkEventListener {}

#[async_trait::async_trait]
impl EventListener for SdkEventListener {
    async fn on_event(&self, e: SdkEvent) {
        match e {
            SdkEvent::Synced => {
                // Data has been synchronized with the network. When this event is received,
                // it is recommended to refresh the payment list and wallet balance.
            }
            SdkEvent::UnclaimedDeposits { unclaimed_deposits } => {
                // SDK was unable to claim some deposits automatically
            }
            SdkEvent::ClaimedDeposits { claimed_deposits } => {
                // Deposits were successfully claimed
            }
            SdkEvent::PaymentSucceeded { payment } => {
                // A payment completed successfully
            }
            SdkEvent::PaymentPending { payment } => {
                // A payment is pending (waiting for confirmation)
            }
            SdkEvent::PaymentFailed { payment } => {
                // A payment failed
            }
            SdkEvent::Optimization { optimization_event } => {
                // An optimization event occurred
            }
        }
    }
}

pub(crate) async fn add_event_listener(
    sdk: &BreezSdk,
    listener: Box<SdkEventListener>,
) -> Result<String> {
    let listener_id = sdk.add_event_listener(listener).await;
    Ok(listener_id)
}