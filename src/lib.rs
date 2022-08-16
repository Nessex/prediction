#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]
#![feature(result_flattening)]

pub mod client;
mod clients;
mod engine;
mod template;
mod templates;

#[cfg(test)]
mod tests {
    use crate::clients::huggingface::HuggingFaceClient;
    use crate::engine::PredictionEngine;
    use crate::templates::qa_html::QAHtmlTemplate;

    fn huggingface_client() -> Option<HuggingFaceClient> {
        if let Ok(api_key) = std::env::var("TEST_HUGGINGFACE_API_KEY") {
            Some(
                HuggingFaceClient::new(&api_key, "bigscience/bloom")
                    .expect("Unable to initialize HuggingFaceClient"),
            )
        } else {
            eprintln!("Warning: TEST_HUGGINGFACE_API_KEY is not set. Corresponding tests have been skipped.");

            None
        }
    }

    #[tokio::test]
    async fn it_works() {
        if let Some(client) = huggingface_client() {
            let template = QAHtmlTemplate::new()
                .with_preface("Here are the outputs from each enquiry:")
                .with_example("What is your pet's name?", "Flippers")
                .with_example("What is the name of your hometown?", "McMurdo Station");

            let hf_qa = PredictionEngine::new(client, template);
            let res = hf_qa
                .predict("What type of pet do you own?")
                .await
                .expect("prediction is expected to succeed");

            assert_eq!("Penguin", &res);
        }
    }
}
