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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentType {
    Sent,
    Received
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Lightning,
    Onchain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningPaymentDetails {
  pub description: Option<String>,
  pub primage: Option<String>,
  pub invoice: String,
  pub payment_hash: String,
  pub destination_pubkey: String,
  pub lnurl_pay_info: Option<serde_json::Value>,
  pub lnurl_withdraw_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub payment_type: PaymentType,
    pub status: PaymentStatus,
    pub amount: u64,     
    pub fees: u64,        
    pub timestamp: i64,   
    pub method: PaymentMethod,
    pub details: Option<LightningPaymentDetails>,
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