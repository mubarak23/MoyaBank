use tonic_lnd::{
  Client,
  lnrpc::{
    Invoice, GetInfoRequest, ListInvoiceRequest, ListPaymentsRequest,
    invoice::InvoiceState,
    payment::PaymentStatus, 
    NodeInfo,
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
  pub cert: String
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

  


}