// http://gtk-rs.org

use gio::prelude::*;
use gtk::prelude::*;

use crate::clone;

pub fn build_system_menu(
    application: &gtk::Application,
    window: &gtk::ApplicationWindow,
    about_dialog: &gtk::AboutDialog,
) {
    let menu = gio::Menu::new();

    menu.append(Some("About"), Some("app.about"));
    menu.append(Some("Quit"), Some("app.quit"));

    application.set_app_menu(Some(&menu));

    let quit = gio::SimpleAction::new("quit", None);
    let about = gio::SimpleAction::new("about", None);
    quit.connect_activate(clone!(@strong window => move |_, _| {
        window.close();
    }));
    about.connect_activate(clone!(@strong about_dialog => move |_, _| {
        about_dialog.show();
    }));

    application.add_action(&about);
    application.add_action(&quit);
}
