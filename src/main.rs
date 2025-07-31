use std::ops::Deref;

use anyhow::{Context, Result, anyhow};
use poem::middleware::Tracing;
use poem::{Body, EndpointExt};
use poem::{IntoResponse, Response, Route, Server, get, handler, listener::TcpListener, web::Query};
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::process::Command;
// use tracing::{debug, error, info, instrument, warn};
use serde::{Deserialize, Serialize};
use serde_json;
use tracing_subscriber;

#[derive(Debug, Deserialize, Serialize)]
struct Params {
    size: u8,
    title: String,
    website: String,
    date: String,
}

#[handler]
async fn generate(res: poem::Result<Query<Params>>) -> Result<impl IntoResponse> {
    let params = res?;

    // info!("Starting OpenGraph image generation");
    let temp_dir = tempfile::tempdir().context("Failed to create tempdir")?;

    // Create assets dir
    let asset_dir = temp_dir.path().join("assets");
    fs::create_dir(&asset_dir).await?;

    // Copy the background image to assets
    let background_image = include_bytes!("../template/assets/background.png");
    fs::write(asset_dir.join("background.png"), background_image).await?;

    // Copy the typst file to temp dir
    let typst_template = include_str!("../template/og.typ");
    let typst_file_path = temp_dir.path().join("og.typ");
    fs::write(&typst_file_path, typst_template).await?;

    // Create temp file for the output
    let output_file = NamedTempFile::new().context("Failed to create temp output file")?;

    let json_data = serde_json::to_string(&params.deref()).context("Could not convert params to string")?;

    // Prep the command
    let mut command = Command::new("typst");
    command.arg("compile").arg("--format").arg("png");

    let input = format!("data={json_data}");
    command.arg("--input").arg(input);

    command.arg(&typst_file_path).arg(output_file.path());

    // Clear environment variables to avoid leaking sensitive data
    command.env_clear();

    // Preserve environment variables needed for font discovery
    if let Ok(path) = std::env::var("PATH") {
        command.env("PATH", path);
    }
    if let Ok(home) = std::env::var("HOME") {
        command.env("HOME", home);
    }

    let output = command.output().await?;
    if !output.status.success() {
        return Err(anyhow!("Typst failed to compile the image"));
    }

    let image_bytes = fs::read(output_file.path()).await?;

    Ok(Response::builder()
        .content_type("image/png")
        .body(Body::from_bytes(image_bytes.into())))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();
    let app = Route::new().at("/generate", get(generate).with(Tracing));
    Server::new(TcpListener::bind("0.0.0.0:3000")).run(app).await
}
