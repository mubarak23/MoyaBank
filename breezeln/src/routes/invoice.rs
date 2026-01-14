use axum::{extract::State, Json};
use breez_sdk_spark::{ReceivePaymentRequest, ReceivePaymentMethod, ListPaymentsRequest,
   Payment, PrepareSendPaymentRequest, SendPaymentOptions, SendPaymentRequest};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
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
pub struct PayInvoiceRequest {
    pub invoice: String,
}

#[derive(Serialize)]
pub struct PayInvoiceResponse {
    pub payment_hash: String,
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

pub async fn pay_invoice(
   State(state): State<AppState>,
    Json(req): Json<PayInvoiceRequest>,
) -> Json<PayInvoiceResponse> {
  // parse input
  // let parse_input = state.breeze.parse(req.invoice).await? {
  //   InputType::Bolt11Invoice(details) => {
  //       println!(
  //           "Input is BOLT11 invoice for {} msats",
  //           details
  //               .amount_msat
  //               .map_or("unknown".to_string(), |a| a.to_string())
  //       );
  //       details
  //               .amount_msat
  //               .map_or("unknown".to_string(), |a| a.to_string())
  //   },
  //    _ => {}
  // }

  let prepare_response = state.breeze.prepare_send_payment(PrepareSendPaymentRequest {
      payment_request: req.invoice,
      token_identifier: None,
  }).await?;

  let send_option = Some(SendPaymentOptions::Bolt11Invoice {
      prefer_spark: false,
      completion_timeout_secs: Some(15),
  })
  let optional_idempotency_key = Some(Uuid::new_v4().to_string());

  let pay_invoice = state.breeze.send_payment(SendPaymentRequest{
    prepare_response,
    options: send_option,
    idempotency_key: 
  }).await?

   Json(PayInvoiceResponse {
    payment_hash: pay_invoice.unwrap().payment 
  })

}