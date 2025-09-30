mod cache;
mod desktop;
mod gui;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--service".to_string()) {
        service::run_service().await;
    } else {
        gui::run_gui();
    }
    Ok(())
}
