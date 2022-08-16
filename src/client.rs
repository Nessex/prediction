use async_trait::async_trait;
use std::error::Error;
use std::future::Future;
use thiserror::Error;

#[async_trait]
pub trait PredictionClient {
    type PredictionFuture: Future<Output = Result<String, PredictionError>>;

    fn run_prediction(&self, input: &str) -> Self::PredictionFuture;
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("prediction client failed to initialize")]
    InitializationError(Box<dyn Error>),
}

#[derive(Error, Debug)]
pub enum PredictionError {
    #[error("failed to extract prediction results")]
    ExtractionError,
    #[error("prediction result was incomplete")]
    IncompleteResult,
    #[error("unknown error in prediction client")]
    PredictionClientFailure(Box<dyn Error>),
    #[error("input was expected to be present in output")]
    InputMismatch,
}
