pub mod server;
pub mod view;

use std::io::{Result};
use std::thread;
use iced::{Settings, Sandbox};
use view::MainWindow;

fn main() -> Result<()> {
    let server = thread::spawn(move || {
        server::start_server()
    });

    let result_view = MainWindow::run(Settings::default());

    let result_server = server.join();

    Ok(())
}