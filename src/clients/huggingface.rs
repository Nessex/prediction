use crate::client::{ClientError, PredictionClient, PredictionError};
use futures_util::TryFutureExt;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use std::future::Future;

pub struct HuggingFaceClient {
    http_client: Client,
    model: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    generated_text: String,
}

impl HuggingFaceClient {
    pub fn new(api_token: &str, model: &str) -> Result<Self, ClientError> {
        let hv = HeaderValue::from_str(&format!("Bearer {}", api_token))
            .map_err(|e| ClientError::InitializationError(Box::new(e)))?;

        let mut headers = HeaderMap::new();
        headers.append("Authorization", hv);

        let http_client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| ClientError::InitializationError(Box::new(e)))?;

        Ok(Self {
            http_client,
            model: model.to_string(),
        })
    }
}

impl PredictionClient for HuggingFaceClient {
    type PredictionFuture = impl Future<Output = Result<String, PredictionError>>;

    fn run_prediction(&self, input: &str) -> Self::PredictionFuture {
        self.http_client
            .post(format!(
                "https://api-inference.huggingface.co/models/{}",
                self.model
            ))
            .body(input.to_string())
            .send()
            .and_then(|r| r.json::<Vec<ApiResponse>>())
            .map_ok(|r| r[0].generated_text.clone())
            .map_err(|e| PredictionError::PredictionClientFailure(Box::new(e)))
    }
}
