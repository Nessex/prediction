use crate::client::PredictionError;

pub trait PredictionTemplate {
    fn render(&self, input: &str) -> String;
    fn extract(&self, raw_result: &str, input: &str) -> Result<String, PredictionError>;
}
