use axum::{extract::State, Json};
use breez_sdk_spark::{ReceivePaymentRequest, ReceivePaymentMethod};
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