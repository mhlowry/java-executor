use axum::{handler::post, Router, extract::{ContentLengthLimit, Json}};
use serde_derive::Deserialize;
use std::process::Command;
use std::process::Output;
use std::fs::write;

const MAX_BODY_LENGTH: u64 = 1024 * 16; // 16 KB

#[derive(Deserialize)]
struct CodePayload {
    code: String,
}

async fn execute_custom_java(
    Json(payload): Json<CodePayload>,
    _limit: ContentLengthLimit<(), MAX_BODY_LENGTH>
) -> String {
    let path = "custom_code.java";
    write(path, &payload.code).expect("Failed to write to file");

    let compile_output: Output = Command::new("javac")
        .arg(path)
        .output()
        .expect("Failed to execute command");

    if !compile_output.status.success() {
        return String::from_utf8_lossy(&compile_output.stderr).to_string();
    }

    let run_output: Output = Command::new("java")
        .arg("-cp")
        .arg(".")
        .arg("custom_code")
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&run_output.stdout).to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/execute", post(execute_custom_java))
        .boxed();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
