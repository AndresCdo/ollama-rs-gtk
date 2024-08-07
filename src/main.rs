// http://gtk-rs.org

use gio::prelude::*;

mod api;
mod menu;
mod preview;
mod ui;

use crate::ui::build_ui;
#[macro_use]
mod utils;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[tokio::main]
async fn main() {
    let application = gtk::Application::new(
        Some("com.github.ollama-rs-gtk"),
        gio::ApplicationFlags::empty(),
    )
    .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);

        let quit = gio::SimpleAction::new("quit", None);
        quit.connect_activate(clone!(@strong app => move |_, _| {
            app.quit();
        }));
        app.add_action(&quit);
    });

    application.connect_activate(|_| {});

    application.run(&std::env::args().collect::<Vec<_>>());
}
