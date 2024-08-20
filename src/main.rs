use axum::{
    extract::Form,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct ImageGenerationRequest {
    html_text: String,
}

#[tokio::main]
async fn main() {
    // Load the OpenAI API key from the environment
    // let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    // let openai_api_key = "sk-1234567890";

    // Build our application with two routes
    let app = Router::new()
        .route("/", get(index_page))
        .route("/generate-image", post(generate_image));

    // Run the server on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index_page() -> Html<&'static str> {
    Html(include_str!("templates/index.html"))
}

async fn generate_image(Form(request): Form<ImageGenerationRequest>) {
    // TODO: Implement the logic to generate an AI image from the HTML text
    println!("HTML text: {}", request.html_text);
    // Redirect the user back to the index page
    Redirect::to("/");
}