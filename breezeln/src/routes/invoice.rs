use axum::{extract::State, Json};
use breez_sdk_spark::{ReceivePaymentRequest, ReceivePaymentMethod, ListPaymentsRequest};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateInvoiceRequest {
  pub amount_sats: u64,
  pub description: Option<String>
}

#[derive(Serialize)]
pub struct CreateInvoiceResponse {
    pub paymentRequest: String,
}

#[derive(Serialize)]
pub struct PaymentListsResponse {
    pub paymentListsResponse: Vec<Payment>,
}

#[derive(Serialize)]
pub enum Payment {
    Sent { data: PaymentData },
    Received { data: PaymentData },
    ClosedChannel { data: PaymentData },
}

#[derive(Serialize)]
pub struct PaymentData {
    pub id: String,
    pub payment_type: PaymentType,
    pub amount_msat: u64,
    pub fee_msat: u64,
    pub status: PaymentStatus,
    pub description: Option<String>,
    pub details: PaymentDetails,
    pub created_at: i64, 
}

#[derive(Serialize)]
pub enum PaymentType {
    Sent,
    Received,
    ClosedChannel,
}

#[derive(Serialize)]
pub enum PaymentStatus {
    Pending,
    Complete,
    Failed,
}

#[derive(Serialize)]
pub enum PaymentDetails {
    Ln {
        invoice: Option<String>,
        payment_hash: Option<String>,
        destination_pubkey: Option<String>,
        lnurl_success_action: Option<LnUrlSuccessAction>,
    },
    Onchain {
        txid: Option<String>,
        address: Option<String>,
    },
    ClosedChannel {
        closing_txid: Option<String>,
    },
}


pub async fn create_invoice(
   State(state): State<AppState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Json<CreateInvoiceResponse> {
  let response = state.breeze.receive_payment(
      ReceivePaymentRequest {
        payment_method: ReceivePaymentMethod::Bolt11Invoice {
          description: req.description,
          amount_sats: req.amount_sats
        }
      }
  ).await?;
  
  Json(CreateInvoiceResponse {
        paymentRequest: response.payment_request,
    })
}

pub async fn list_payments(
  State(state): State<AppState>
) -> Json<PaymentListsResponse> {
  let response state.breeze.list_payments(ListPaymentsRequest::default()).await();

  Json(PaymentListsResponse {
    paymentListsResponse: response.payments 
  })
}