use breez_sdk_spark::BreezSdk;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub breeze: Arc<BreezSdk>,
}
