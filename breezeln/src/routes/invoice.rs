use axum::{extract::State, Json};
use breez_sdk_spark::{ReceivePaymentRequest, ReceivePaymentMethod, ListPaymentsRequest, Payment};
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


pub async fn create_invoice(
   State(state): State<AppState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Json<CreateInvoiceResponse> {
  let response = state.breeze.receive_payment(
      ReceivePaymentRequest {
        payment_method: ReceivePaymentMethod::Bolt11Invoice {
          description: req.description.unwrap(),
          amount_sats: Some(req.amount_sats)
        }
      }
  ).await;
  
  Json(CreateInvoiceResponse {
        paymentRequest: response.unwrap().payment_request,
    })
}

pub async fn list_payments(
  State(state): State<AppState>
) -> Json<PaymentListsResponse> {
  let response = state.breeze.list_payments(ListPaymentsRequest::default()).await;

  Json(PaymentListsResponse {
    paymentListsResponse: response.unwrap().payments 
  })
}