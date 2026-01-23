use crate::errors::LightningError;
use bitcoin::Txid;
use bitcoin::secp256k1::PublicKey;
use expanduser::expanduser;
use lightning::ln::features::NodeFeatures;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod jwt;

// #[derive(Serialize, Debug, Clone)]
// pub enum NodeId {
//   PublicKey(PublicKey),
//   Alias(String)
// }

// impl NodeId {
//   pub fn validate(&self, node_id: &PublicKey, alias: &mut String) -> Result<(), LightningError> {
//       match self {
//         NodeId::PublicKey(pk) => {
//            if pk != node_id {
//                     return Err(LightningError::ValidationError(format!(
//                         "The provided node id does not match the one returned by the backend ({pk} != {node_id})"
//                     )));
//                 }
//         }
//         NodeId::Alias(r#as) => {
//           if r#as != alias {
//                     return Err(LightningError::ValidationError(format!(
//                         "The provided alias does not match the one returned by the backend ({as} != {alias})"
//                     )));
//                 }
//         }
//       }
//       Ok(())
//   }
// }

// impl std::fmt::Display for NodeId {
//      fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 NodeId::PublicKey(pk) => pk.to_string(),
//                 NodeId::Alias(a) => a.to_owned(),
//             }
//         )
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct NodeInfo {
//     pub pubkey: PublicKey,
//     pub alias: String,

//     #[serde(with = "node_features_serde")]
//     pub features: NodeFeatures
// }

// impl Display for NodeInfo {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     let pk = self.pubkey.to_string();
//     let pk_summary = format!("{}...{}", &pk[..6], &pk[pk.len() - 6..]);
//     if self.alias.is_empty() {
//        write!(f, "{pk_summary}")
//     } else {
//        write!(f, "{}({})", self.alias, pk_summary)
//     }
//   }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomInvoice {
    pub memo: String,
    pub payment_hash: String,
    pub payment_preimage: String,
    pub value: u64,
    pub value_msat: u64,
    pub creation_date: Option<i64>,
    pub settle_date: Option<i64>,
    pub payment_request: String,
    pub expiry: Option<u64>,
    pub state: InvoiceStatus,
    pub is_keysend: Option<bool>,
    pub is_amp: Option<bool>,
    pub payment_addr: Option<String>,
    pub htlcs: Option<Vec<InvoiceHtlc>>,
    //  pub features: Option<HashMap<u32, Feature>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PaymentType {
    Outgoing,
    Incoming,
    Forwarded,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum InvoiceStatus {
    #[default]
    Settled,
    Open,
    Expired,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceHtlc {
    pub chan_id: Option<u64>,
    pub htlc_index: Option<u64>,
    pub amt_msat: Option<u64>,
    pub accept_time: Option<i64>,
    pub resolve_time: Option<i64>,
    pub expiry_height: Option<u32>,
    pub mpp_total_amt_msat: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct ChannelSummary {
    //  pub chan_id: ShortChannelID,
    pub alias: Option<String>,
    pub channel_state: ChannelState,
    pub private: bool,
    pub remote_balance: u64,
    pub local_balance: u64,
    pub capacity: u64,
    pub last_update: Option<u64>,
    pub uptime: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum ChannelState {
    Opening,
    #[default]
    Active,
    Disabled,
    Closing,
    Closed,
    Failed,
}

pub mod serde_address {
    use super::*;

    pub fn serialize<S>(address: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(address)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("https://") || s.starts_with("http://") {
            Ok(s)
        } else {
            Ok(format!("https://{s}"))
        }
    }
}

pub fn deserialize_path<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(expanduser(s)
        .map_err(serde::de::Error::custom)?
        .display()
        .to_string())
}
