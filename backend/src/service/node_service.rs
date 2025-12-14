use tonic_lnd::{
  Client,
  lnrpc::{
    Invoice, GetInfoRequest, ListInvoiceRequest, ListPaymentsRequest,
    invoice::InvoiceState,
    payment::PaymentStatus, 
    NodeInfo,
    Payment,
  }
}
use crate::{
  error::LightningError
  utils::{
    self, NodeInfo, NodeId, CustomInvoice, PaymentType, InvoiceStatus, InvoiceHtlc, 
    ChanInfoRequest, ChannelState,
  }
}
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};
use serde::{Deserialize, Serialize};
use lightning_invoice::{Bolt11InvoiceDescription, Bolt11Invoice};
use lightning::ln::PaymentHash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LndConnection {
  #[serde(with = "utils::serde_node_id")]
  pub id: NodeId,
  #[serde(with = "utils::serde_address")]
  pub address: String,
  #[serde(deserialize_with = "utils::deserialize_path")]
  pub macaroon: String,
  #[serde(deserialize_with = "utils::deserialize_path")]
  pub certificate: String
}


#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ConnectionRequest {
  Lnd(LndConnection)
}


pub struct LndNode {
  pub client: Mutex<Client>,
  pub info: NodeInfo
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

impl LndNode {

  pub async fn new(connection: LndConnection) -> Result<Self, LightningError> {
      let mut client = 
          Self::connect_lnd_with_hex(connection.address, connection.certificate, connection.macaroon)
          .await.map_err(|e| LightningError::connection_error(err.to_string()))?;
      let node_info = client.lightning()
          .get_info(GetInfoRequest {})
          .await
          .map_err(|err| LightningError::GetInfoError(err.to_string()))?
          .into_inner();

      let mut alias = node_info.alias;
      let pubkey = PublicKey::from_str(&node_info.identity_pubkey)
        .map_err(|err| LightningError::GetInfoError(err.to_string()))?
      connection.id.validate(&pubkey, &mut alias)?;

      Ok(Self {
        client: Mutex::new(client),
        info: NodeInfo {
          pubkey,
          alias,
          features:  parse_node_features(node_info.features.keys().cloned().collect());
        }
      })
  }

  fn hex_to_temp_file(hex_string: String) -> Result<(NamedTempFile, PathBuf), std::io::Error> {
     let bytes = hex::decode(hex_string).map_err(|e|, {
        std::io::Error::new(
          std::io::ErrorKind::InvalidData,
          format!("Invalid hex string: {}", e)
        )
     })?;

     let mut temp_file = NamedTempFile::new()?;
     temp_file.write_all(&bytes)?;
     temp_file.flush()?;

     let path = temp_file.path().to_path_buf();

     Ok((temp_file, path))
  }


  async fn connect_lnd_with_hex(
    address: String,
    cert_hex: String,
    macaroon_hex: String
  ) -> Result<tonic_lnd::Client, Box<dyn std::error::Error>> {
      let (cert_file, cert_path) = self.hex_to_temp_file(&cert_hex)?;
      let (macaroon_file, macaroon_path) = Self::hex_to_temp_file(&macaroon_hex)?;

      let client = tonic_lnd::connect(address, cert_path, macaroon_path).await?;

      drop(cert_file);
      drop(macaroon_file);

      Ok(client)
  }

  async fn get_lnd_client_sub(&self) -> tonic_lnd::LightningClient {
    let mut client = self.client.lock().await;
    client.lightning.clone()
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
  async fn create_invoices(&self) -> Result<CustomInvoice, LightningError>;  
  // Get Invoice Details
  async fn get_invoice_details(
        &self,
        payment_hash: &PaymentHash,
    ) -> Result<CustomInvoice, LightningError>;
    /// Gets the onchain wallet balance in satoshis.
    async fn get_wallet_balance(&self) -> Result<u64, LightningError>;
  
  async fn pay_invoice(
    &self,
    payment_request: &str,
  ) -> Result<Payment, LightningError>   

    // async fn get_channel_info(
    //     &self,
    //     channel_id: &ShortChannelID,
    // ) -> Result<ChannelDetails, LightningError>;
}

#[async_trait]
impl LightningClient for LndNode {
   async fn get_node_info(&self) -> &NodeInfo {
      self.info
   }

   async fn get_network(&self) -> Result<Network, LightningError> {
         let mut client = self.client.lock().await;
         let info = client.lightning()
              .get_info()
              .await
              .map_err(|err| LightningError::GetInfoError(err.to_string()))?
              .into_inner();
          if info.chain.is_empty() {
            return err(LightningError::ValidationError(format!(
              "{}  is not connected to any chain",
              self.get_info()
            )));
          } else if info.chain.len() > 1 {
             return err(LightningError::ValidationError(format!(
              "{} is connected to more than one chain: {:?}",
              self.get_info(),
              info.chains.iter().map(|c| c.chain.to_string())
            )));
          }
          Ok(Network::from_str(match info.chain[0].network.as_str(){
            "mainnet" => "bitcoin",
            x => x
          }).map_err(|err| LightningError::ValidationError(err.to_string()))?)
   }

async fn list_channels(&self) -> Result<Vec<ChannelSummary>, LightningError> {
    let mut lightning_lnd = self.get_lnd_client_sub();

    let channel_list = lightning_lnd
        .list_channels(ListChannelsRequest::default())
        .await
        .map_err(|err| LightningError::ChannelError(err.to_string()))?
        .into_inner();

    let mut last_updates: HashMap<u64, u64> = HashMap::new();

    for channel in &channel_list.channels {
        // fetch only public channels
        if !channel.private {
            match lightning_lnd
                .get_chan_info(ChanInfoRequest {
                    chan_id: channel.chan_id,
                })
                .await
            {
                Ok(response) => {
                    let chan_info = response.into_inner();
                    let mut last_max_update = 0u64;

                    if let Some(node1_policy) = &chan_info.node1_policy {
                        last_max_update =
                            last_max_update.max(node1_policy.last_update as u64);
                    }

                    if let Some(node2_policy) = &chan_info.node2_policy {
                        last_max_update =
                            last_max_update.max(node2_policy.last_update as u64);
                    }

                    if last_max_update > 0 {
                        last_updates.insert(channel.chan_id, last_max_update);
                    }
                }
                Err(e) => {
                    // Channel may not be announced yet
                    tracing::debug!(
                        "Failed to get channel info for {}: {}",
                        channel.chan_id,
                        e
                    );
                }
            }
        }
    }

    let channels: Vec<ChannelSummary> = channel_list
        .channels
        .into_iter()
        .map(|channel| {
            let channel_state = if channel.active {
                ChannelState::Active
            } else {
                ChannelState::Disabled
            };

            let last_update = last_updates.get(&channel.chan_id).copied();

            ChannelSummary {
                chan_id: ShortChannelID(channel.chan_id),
                alias: None,
                channel_state,
                private: channel.private,
                remote_balance: channel.remote_balance.try_into().unwrap_or(0),
                local_balance: channel.local_balance.try_into().unwrap_or(0),
                capacity: channel.capacity.try_into().unwrap_or(0),
                last_update,
                uptime: Some(channel.uptime as u64),
            }
        })
        .collect();

    Ok(channels)
}

}