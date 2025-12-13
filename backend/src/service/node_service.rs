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
