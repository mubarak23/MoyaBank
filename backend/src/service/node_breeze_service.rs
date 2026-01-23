use crate::{
    errors::LightningError,
    utils::{
        ChannelState, CustomInvoice, InvoiceHtlc, InvoiceStatus, NodeId,
        NodeInfo, PaymentType,
    },
};
use bitcoin::PublicKey;
use breez_sdk_spark::{
    BreezSdk, ConnectRequest, Network as BreezeNetwork, ReceivePaymentMethod,
    ReceivePaymentRequest, SdkError, Seed, SendPaymentRequest, default_config,
};
use tempfile::NamedTempFile;
use tonic_lnd::{
    Client,
    lnrpc::{
       ChanInfoRequest, GetInfoRequest, Invoice, ListInvoiceRequest, ListPaymentsRequest, Payment,
        SendRequest, invoice::InvoiceState, payment::PaymentStatus,
    },
};

use lightning::ln::PaymentHash;
use lightning_invoice::{Bolt11Invoice, Bolt11InvoiceDescription};
use serde::{Deserialize, Serialize};
use tokio::{
    fs::File,
    io::{AsyncReadExt, Error},
    sync::Mutex,
    time::sleep,
};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

#[derive(Debug, Deserialize)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LndConnection {
    #[serde(with = "utils::serde_node_id")]
    pub id: NodeId,
    #[serde(with = "utils::serde_address")]
    pub address: String,
    #[serde(deserialize_with = "utils::deserialize_path")]
    pub macaroon: String,
    #[serde(deserialize_with = "utils::deserialize_path")]
    pub certificate: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ConnectionRequest {
    Lnd(LndConnection),
}

pub struct LndNode {
    pub client: Mutex<Client>,
    pub info: NodeInfo,
}

fn parse_node_features(features: HashMap<u32>) -> NodeFeatures {
    let mut flags = vec![0; 256];
    for f in features.into_iter() {
        let byte_offset = (f / 8) as usize;
        let mask = 1 << (f % 8);
        if flags.len() <= byte_offset {
            flags.resize(byte_offset + 1, 0u8);
        }
        flags[byte_offset] |= mask
    }
    NodeFeatures::from_le_bytes(flags)
}

// Breeze integration
pub struct BreezeNode {
    sdk: BreezSdk,
}

impl BreezeNode {
    pub async fn new(
        storage_dir: &str,
        mnemonic: &str,
        api_key: &str,
    ) -> Result<Self, LightningError> {
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

        Ok(Self { sdk_init })
    }
}

/// A unified interface for Lightning Network node operations across implementations.
#[async_trait]
pub trait LightningClient: Send {
    // get node network
    async fn get_network(&self) -> Result<Network, LightningError>;

    async fn get_node_info(&self) -> &NodeInfo;

    /// Lists all channels, returning only their capacities in millisatoshis.
    async fn list_channels(&self) -> Result<Vec<ChannelSummary>, LightningError>;

    /// Lists all invoices.
    async fn list_invoices(&self) -> Result<Vec<CustomInvoice>, LightningError>;
    /// Create invoice
    async fn create_invoices(
        &self,
        amount: u64,
        description: &str,
    ) -> Result<CustomInvoice, LightningError>;
    // Get Invoice Details
    async fn get_invoice_details(
        &self,
        payment_hash: &PaymentHash,
    ) -> Result<CustomInvoice, LightningError>;
    /// Gets the onchain wallet balance in satoshis.
    async fn get_wallet_balance(&self) -> Result<u64, LightningError>;

    async fn pay_invoice(&self, payment_request: &str) -> Result<Payment, LightningError>;

    // async fn get_channel_info(
    //     &self,
    //     channel_id: &ShortChannelID,
    // ) -> Result<ChannelDetails, LightningError>;
}
