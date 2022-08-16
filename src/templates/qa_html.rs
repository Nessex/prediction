use crate::client::PredictionError;
use crate::template::PredictionTemplate;

pub struct QAHtmlTemplate {
    preface: Option<String>,
    examples: Vec<(String, String)>,
}

impl QAHtmlTemplate {
    pub fn new() -> Self {
        Self {
            preface: None,
            examples: vec![],
        }
    }

    pub fn with_preface(mut self, preface: &str) -> Self {
        self.preface = Some(preface.to_string());

        self
    }

    pub fn with_example(mut self, input: &str, output: &str) -> Self {
        self.examples.push((input.to_string(), output.to_string()));

        self
    }
}

impl PredictionTemplate for QAHtmlTemplate {
    fn render(&self, input: &str) -> String {
        let mut out = String::new();

        if let Some(preface) = &self.preface {
            out += &format!("<p>{preface}</p>\n");
        }

        out += "<table><thead><tr><th>Input</th><th>Output</th></tr></thead><tbody>\n";

        for (ei, eo) in self.examples.iter() {
            out += &format!("<tr><td>{ei}</td><td>{eo}</td></tr>\n");
        }

        out += &format!("<tr><td>{input}</td><td>");

        out
    }

    fn extract(&self, raw_result: &str, input: &str) -> Result<String, PredictionError> {
        let output = raw_result
            .strip_prefix(input)
            .ok_or(PredictionError::InputMismatch)?;

        Ok(if let Some(output_end) = output.find("</td>") {
            let (output, _extra) = output.split_at(output_end);
            output
        } else {
            output
        }
        .to_string())
    }
}
