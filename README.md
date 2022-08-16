# Prediction

*Not ready for use*
*The code is bad, the library interface is bad, it's all going to change*

This is a simple rust library for doing structured prediction with ML models.

It's intended to allow you to quickly include structured, AI-driven prediction into an application, to codify integrations with models like Bloom, GPT-J etc.

Example usage:
```rust
// This client uses the huggingface.co inference API to generate predictions, and is configured to use the
// 176 Billion parameter Bloom model.
// https://huggingface.co/bigscience/bloom
let client = HuggingFaceClient::new("hf_apikey", "bigscience/bloom").unwrap();

// QAHtmlTemplate is a template that's worked well for me. It provides a description, followed by a HTML table.
// The table can be pre-seeded with some examples to improve output, and then a table cell is left blank for the
// model to predict the answer to a question.
// This structure also allows you to easily identify the end of a particular answer. That can remove some noise where
// the model runs off and starts inventing new questions, new characters etc.
//
// Here's an example:
let template = QAHtmlTemplate::new()
    // A preface can provide additional context to the model
    .with_preface("About me:")
    // Examples set the tone for future answers, and ensure the model has a grasp on the structure of the response
    .with_example("What is your pet's name?", "Flippers")
    .with_example("What is the name of your hometown?", "McMurdo Station");

let hf_qa = PredictionEngine::new(client, template);

let res = hf_qa
    // An answer to this question will be extracted from the table cell that is expected to follow.
    .predict("What type of pet do you own?")
    .await
    .unwrap();

assert_eq!("Penguin", &res);

// There may be additional steps you need to take here to clean up the output before use
// e.g. removing HTML tags, filtering out links, retrying if the output is clearly not good...
// this will depend upon your requirements.
```

## Clients

A client takes an input and sends it off to predict what the next output should be. Clients can be wrappers around APIs, in-memory models etc. 

## Templates

Templates define the structure within which a prediction should be made. Predictions are better when they are presented in a structured / prompted form.

This allows defining common structures or prompts that achieve better results from a given model, for more specific types of questions.

## Engine

An engine is an interface around a client and a template. An engine can be passed around within your application, and used to make async requests to predict input.
