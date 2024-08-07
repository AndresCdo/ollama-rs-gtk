use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use gio::prelude::Cast;
use gtk::prelude::TextBufferExt;
use gtk::HeaderBar;
use gtk::HeaderBarExt;
use gtk::TextBuffer;
use sourceview::prelude::BufferExt;
use sourceview::LanguageManager;
use sourceview::LanguageManagerExt;

pub fn buffer_to_string(buffer: &TextBuffer) -> String {
    let start_iter = buffer.get_start_iter();
    let end_iter = buffer.get_end_iter();
    buffer
        .get_text(&start_iter, &end_iter, false)
        .expect("Failed to get text from buffer")
        .to_string()
}

pub fn save_file(filename: &PathBuf, text_buffer: &TextBuffer) {
    let contents = buffer_to_string(text_buffer);
    let mut file = File::create(filename).expect("Couldn't create file");
    file.write_all(contents.as_bytes())
        .expect("Couldn't write to file");
}

pub fn set_title(header_bar: &HeaderBar, path: &Path) {
    if let Some(file_name) = path.file_name() {
        let file_name: &str = &file_name.to_string_lossy();
        header_bar.set_title(Some(file_name));

        if let Some(parent) = path.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            header_bar.set_subtitle(Some(subtitle));
        }
    }
}

pub fn open_file(filename: &Path) -> String {
    let file = File::open(filename).expect("Couldn't open file");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);

    contents
}

pub fn configure_sourceview(buffer: &gtk::TextBuffer) {
    let language_manager = LanguageManager::get_default().expect("Failed to get language manager");
    let language = language_manager
        .get_language("markdown")
        .expect("Failed to get language");

    let buffer: &sourceview::Buffer = buffer
        .downcast_ref()
        .expect("Failed to downcast TextBuffer to Buffer");
    buffer.set_language(Some(&language));
}

#[macro_export]
macro_rules! clone {
    // Match `@strong` token and clone the variable
    (@strong $($n:ident),+ => move || $body:expr) => {
        {
            let ($($n),+) = ($($n.clone()),+);
            $(let $n = $n.clone();)+
            move || $body
        }
    };
    (@strong $($n:ident),+ => move |$($p:pat),*| $body:expr) => {
        {
            $(let $n = $n.clone();)+
            move |$($p),*| $body
        }
    };
    (@strong $($n:ident),+ => async move { $($body:tt)* }) => {
        {
            $(let $n = $n.clone();)+
            async move { $($body)* }
        }
    };
    // Fallback for other cases
    ($($body:tt)*) => {
        $($body)*
    };
}

#[cfg(test)]
mod tests {
    use glib::{MainContext, MainLoop};
    use gtk::prelude::*;
    use lazy_static::lazy_static;
    use std::path::PathBuf;
    use std::sync::Mutex;

    use super::*;

    lazy_static! {
        static ref GTK_INIT: Mutex<()> = {
            gtk::init().unwrap();
            Mutex::new(())
        };
    }

    #[tokio::test]
    async fn test_buffer_to_string() {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK");
            return;
        }

        let _guard = GTK_INIT.lock().unwrap();
        let main_context = MainContext::default();
        let main_loop = MainLoop::new(Some(&main_context), false);

        main_context.spawn_local(clone!(@strong main_loop => async move {
            let buffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
            buffer.set_text("Hello, World!");
            assert_eq!(buffer_to_string(&buffer), "Hello, World!");
            main_loop.quit();
        }));

        main_loop.run();
    }

    #[test]
    fn test_open_file() {
        let filename = PathBuf::from("src/utils.rs");
        let contents = open_file(&filename);
        assert!(!contents.is_empty());
    }
}
