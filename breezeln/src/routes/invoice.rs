use crate::state::AppState;
use axum::{extract::State, Json};
use breez_sdk_spark::{
    GetInfoRequest, InputType, ListPaymentsRequest, Payment, PrepareSendPaymentRequest,
    ReceivePaymentMethod, ReceivePaymentRequest, SendPaymentOptions, SendPaymentRequest,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateInvoiceRequest {
    pub amount_sats: u64,
    pub description: Option<String>,
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
pub struct BalanceResponse {
    pub balance: u64,
}

#[derive(Serialize)]
pub struct PayInvoiceResponse {
    pub payment: Payment,
}

#[derive(Serialize)]
pub struct PaymentListsResponse {
    pub paymentListsResponse: Vec<Payment>,
}

pub async fn create_invoice(
    State(state): State<AppState>,
    Json(req): Json<CreateInvoiceRequest>,
) -> Json<CreateInvoiceResponse> {
    let response = state
        .breeze
        .receive_payment(ReceivePaymentRequest {
            payment_method: ReceivePaymentMethod::Bolt11Invoice {
                description: req.description.unwrap(),
                amount_sats: Some(req.amount_sats),
            },
        })
        .await;

    Json(CreateInvoiceResponse {
        paymentRequest: response.unwrap().payment_request,
    })
}

pub async fn list_payments(State(state): State<AppState>) -> Json<PaymentListsResponse> {
    let response = state
        .breeze
        .list_payments(ListPaymentsRequest::default())
        .await;

    Json(PaymentListsResponse {
        paymentListsResponse: response.unwrap().payments,
    })
}

// pub fn pay_invoice_spawn(
//     State(state): State<AppState>,
//     Json(req): Json<PayInvoiceRequest>,
// ) -> impl std::future::Future<Output = Json<PayInvoiceResponse>> + Send {
//     let breeze = state.breeze.clone();
//     let invoice = req.invoice.clone();

//     async move {
//         let pay_invoice = tokio::task::spawn_blocking(move || {
//             // Run all Breez SDK calls in this blocking thread
//             let parsed_input =
//                 futures::executor::block_on(breeze.parse(invoice.as_str())).unwrap();

//             let amount_msat = match parsed_input {
//                 InputType::Bolt11Invoice(details) => details.amount_msat.map(|a| a as u128),
//                 _ => None,
//             };

//             let prepare_response = futures::executor::block_on(
//                 breeze.prepare_send_payment(PrepareSendPaymentRequest {
//                     payment_request: invoice,
//                     amount: amount_msat,
//                     token_identifier: None,
//                 }),
//             )
//             .unwrap();

//             let pay_invoice = futures::executor::block_on(breeze.send_payment(SendPaymentRequest {
//                 prepare_response,
//                 options: Some(SendPaymentOptions::Bolt11Invoice {
//                     prefer_spark: false,
//                     completion_timeout_secs: Some(15),
//                 }),
//                 idempotency_key: Some(Uuid::new_v4().to_string()),
//             }))
//             .unwrap();

//             pay_invoice
//         })
//         .await
//         .unwrap();

//         Json(PayInvoiceResponse {
//             payment: pay_invoice.payment,
//         })
//     }
// }

pub async fn pay_invoice(
    State(state): State<AppState>,
    Json(req): Json<PayInvoiceRequest>,
) -> Json<PayInvoiceResponse> {
    let parsed_input = state.breeze.parse(req.invoice.as_str()).await;

    let amount_msat = match parsed_input.unwrap() {
        InputType::Bolt11Invoice(details) => {
            if let Some(a) = details.amount_msat {
                println!("Input is BOLT11 invoice for {} msats", a);
            }
            details.amount_msat.map(|a| a as u128)
        }
        _ => None,
    };

    let prepare_response = state
        .breeze
        .prepare_send_payment(PrepareSendPaymentRequest {
            payment_request: req.invoice,
            amount: amount_msat,
            token_identifier: None,
        })
        .await
        .unwrap();

    let pay_invoice = state
        .breeze
        .send_payment(SendPaymentRequest {
            prepare_response,
            options: Some(SendPaymentOptions::Bolt11Invoice {
                prefer_spark: false,
                completion_timeout_secs: Some(15),
            }),
            idempotency_key: Some(Uuid::new_v4().to_string()),
        })
        .await
        .unwrap();

    Json(PayInvoiceResponse {
        payment: pay_invoice.payment,
    })
}

pub async fn balance(State(state): State<AppState>) -> Json<BalanceResponse> {
    let response = state
        .breeze
        .get_info(GetInfoRequest {
            ensure_synced: Some(false),
        })
        .await;

    Json(BalanceResponse {
        balance: response.unwrap().balance_sats,
    })
}
