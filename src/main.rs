mod application;
mod service;
mod utils;
mod router;

#[tokio::main]
async fn main() {
    application::app().await.expect("App Exited With Error");
}
