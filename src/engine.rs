use crate::client::{PredictionClient, PredictionError};
use crate::template::PredictionTemplate;
use futures_util::FutureExt;
use std::future::Future;

pub struct PredictionEngine<C: PredictionClient, T: PredictionTemplate> {
    client: C,
    template: T,
}

impl<C: PredictionClient, T: PredictionTemplate> PredictionEngine<C, T> {
    pub fn new(client: C, template: T) -> Self {
        Self { client, template }
    }

    pub fn predict<'a>(
        &'a self,
        input: &'a str,
    ) -> impl Future<Output = Result<String, PredictionError>> + '_ {
        let rendered_input = self.template.render(input);

        self.client
            .run_prediction(&rendered_input.clone())
            .map(move |res| {
                res.map(|r| self.template.extract(&r, &rendered_input))
                    .flatten()
            })
    }
}
