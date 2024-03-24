use axum::{response::Redirect, routing::get, Router};
use chrono::Local;
use clap::Parser;
use std::error::Error;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, env = "FRONTEND_PATH", default_value = "frontend/")]
    frontend_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::temporary("/frontend/index.html") }),
        )
        .route("/time", get(|| async { format!("{}", Local::now()) }))
        .nest_service("/frontend", ServeDir::new(args.frontend_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
