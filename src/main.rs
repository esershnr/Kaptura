#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod media;
mod renderer;
mod ui;

use log::info;
use renderer::app::App;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Starting Kaptura...");

    let app = App::new(None, None)?;
    app.run()?;

    Ok(())
}
