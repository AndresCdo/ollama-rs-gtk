use crate::clone;
use crate::menu::build_system_menu;
use crate::preview::Preview;
use gtk::{AccelGroup, Builder, Button, Entry, TextBuffer, Window, WindowType};
use webkit2gtk::WebViewExt;

use crate::api::send_prompt;
use crate::utils::{buffer_to_string, configure_sourceview, open_file, save_file, set_title};

// http://gtk-rs.org
use sourceview::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("ui.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Builder couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let header_bar: gtk::HeaderBar = builder.get_object("header_bar").unwrap();
    header_bar.set_title(Some(crate::NAME));

    let open_button: gtk::ToolButton = builder.get_object("open_button").unwrap();
    let save_button: gtk::ToolButton = builder.get_object("save_button").unwrap();
    let input_entry: Entry = builder
        .get_object("input_entry")
        .expect("Couldn't get input_entry");
    let send_button: Button = builder
        .get_object("send_button")
        .expect("Couldn't get send_button");

    // Assuming you have a sourceview::Buffer instance
    let source_buffer: TextBuffer = builder
        .get_object("text_buffer")
        .expect("Couldn't get source_buffer");

    // Convert sourceview::Buffer to TextBuffer
    let text_buffer: TextBuffer = source_buffer
        .downcast::<TextBuffer>()
        .expect("Failed to downcast to TextBuffer");

    configure_sourceview(&text_buffer);

    let web_context = webkit2gtk::WebContext::get_default().unwrap();
    let web_view = webkit2gtk::WebView::with_context(&web_context);

    let markdown_view: gtk::ScrolledWindow = builder.get_object("scrolled_window_right").unwrap();
    markdown_view.add(&web_view);

    let file_open: gtk::FileChooserDialog = builder.get_object("file_open").unwrap();
    file_open.add_buttons(&[
        ("Open", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    let file_save: gtk::FileChooserDialog = builder.get_object("file_save").unwrap();
    file_save.add_buttons(&[
        ("Save", gtk::ResponseType::Ok),
        ("Cancel", gtk::ResponseType::Cancel),
    ]);

    let about_dialog: gtk::AboutDialog = builder.get_object("about_dialog").unwrap();
    about_dialog.set_program_name(crate::NAME);
    about_dialog.set_version(Some(crate::VERSION));
    about_dialog.set_authors(&[crate::AUTHORS]);
    about_dialog.set_comments(Some(crate::DESCRIPTION));

    let preview = Preview::new();
    text_buffer.connect_changed(clone!(@strong web_view, preview => move |buffer| {
        let markdown = buffer_to_string(buffer);
        web_view.load_html(&preview.render(&markdown), None);
    }));

    web_view.connect_load_failed(|_, _, _, _| true);

    send_button.connect_clicked({
        let send_button = send_button.clone();
        let text_buffer = text_buffer.clone();
        let input_entry_clone = input_entry.clone();
        move |_| {
            let accel_group = AccelGroup::new();
            let window = Window::new(WindowType::Toplevel);
            window.add_accel_group(&accel_group);
            send_button.set_sensitive(false);
            input_entry_clone.set_sensitive(false);

            let prompt = input_entry_clone.get_text().to_string();

            // Create a MainContext for async execution
            let main_context = glib::MainContext::default();
            main_context.spawn_local({
                let text_buffer = text_buffer.clone();
                let send_button = send_button.clone();
                let input_entry_clone = input_entry_clone.clone();
                async move {
                    let result = send_prompt(&prompt).await;
                    match result {
                        Ok(response) => {
                            // Ensure the GTK operations run on the main thread
                            glib::MainContext::default().invoke_local(move || {
                                let mut end_iter = text_buffer.get_end_iter();
                                text_buffer.insert(&mut end_iter, &format!("You: {}\n\n", prompt));
                                text_buffer
                                    .insert(&mut end_iter, &format!("Ollama: {}\n\n", response));
                                // Re-enable the UI elements
                                send_button.set_sensitive(true);
                                input_entry_clone.set_sensitive(true);
                            });
                        }
                        Err(e) => {
                            eprintln!("Error sending prompt: {}", e);
                            // Re-enable the UI elements in case of error
                            glib::MainContext::default().invoke_local(clone!(move || {
                                send_button.set_sensitive(true);
                                input_entry_clone.set_sensitive(true);
                            }));
                        }
                    }
                }
            });
        }
    });

    input_entry.connect_activate(clone!(@strong send_button => move |_| {
        send_button.clicked();
    }));

    open_button.connect_clicked(
        clone!(@strong file_open, header_bar, text_buffer => move |_| {
            file_open.show();
            if file_open.run() == gtk::ResponseType::Ok {
                if let Some(filename) = file_open.get_filename() {
                    set_title(&header_bar, &filename);
                    let contents = open_file(&filename);
                    text_buffer.set_text(&contents);
                }
            }
            file_open.hide();
        }),
    );

    save_button.connect_clicked(clone!(@strong file_save, text_buffer => move |_| {
        file_save.show();
        if file_save.run() == gtk::ResponseType::Ok {
            if let Some(filename) = file_save.get_filename() {
                save_file(&filename, &text_buffer);
            }
        }
        file_save.hide();
    }));

    about_dialog.connect_delete_event(move |dialog, _| {
        dialog.hide();
        Inhibit(true)
    });

    window.connect_delete_event(move |win, _| {
        win.close();
        Inhibit(false)
    });

    build_system_menu(application, &window, &about_dialog);

    window.show_all();
}
