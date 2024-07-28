use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use gtk::*;
use sourceview::*;
use std::path::PathBuf;

pub fn set_title(header_bar: &HeaderBar, path: &PathBuf) {
    if let Some(file_name) = path.file_name() {
        let file_name: &str = &file_name.to_string_lossy();
        header_bar.set_title(Some(file_name));

        if let Some(parent) = path.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            header_bar.set_subtitle(Some(subtitle));
        }
    }
}

pub fn buffer_to_string(buffer: &Buffer) -> String {
    let (start, end) = buffer.get_bounds();
    buffer.get_text(&start, &end, false).unwrap().to_string()
}

pub fn open_file(filename: &PathBuf) -> String {
    let file = File::open(&filename).expect("Couldn't open file");

    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    let _ = reader.read_to_string(&mut contents);

    contents
}

pub fn save_file(filename: &PathBuf, text_buffer: &Buffer) {
    let contents = buffer_to_string(text_buffer);
    let mut file = File::create(filename).expect("Couldn't save file");
    file.write_all(contents.as_bytes()).expect("File save failed");
}

pub fn configure_sourceview(buff: &Buffer) {
    LanguageManager::new()
        .get_language("markdown")
        .map(|markdown| buff.set_language(Some(&markdown)));

    let manager = StyleSchemeManager::new();
    manager
        .get_scheme("classic")
        .map(|theme| buff.set_style_scheme(Some(&theme)));
}

// http://gtk-rs.org/tuto/closures
/// A macro for creating closures that capture variables by cloning them.
///
/// This macro provides a convenient way to create closures that capture variables by cloning them.
/// It supports two syntaxes:
///
/// - `$($n:ident),+ => move || $body:expr`: Creates a closure that captures the variables `$n` by cloning them and has no parameters.
/// - `$($n:ident),+ => move |$($p:tt),+| $body:expr`: Creates a closure that captures the variables `$n` by cloning them and has parameters specified by `$($p:tt),+`.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate my_macro;
/// # fn main() {
/// let x = 42;
/// let closure = clone!(x => move || {
///     println!("x: {}", x);
/// });
/// closure();
/// # }
/// ```
///
/// In this example, the `clone!` macro is used to create a closure that captures the variable `x` by cloning it.
/// The closure takes no parameters and simply prints the value of `x`.
///
/// ```
/// # #[macro_use] extern crate my_macro;
/// # fn main() {
/// let x = 42;
/// let y = "hello".to_string();
/// let closure = clone!(x, y => move |a: i32, b: String| {
///     println!("x: {}, y: {}, a: {}, b: {}", x, y, a, b);
/// });
/// closure(10, "world".to_string());
/// # }
/// ```
///
/// In this example, the `clone!` macro is used to create a closure that captures the variables `x` and `y` by cloning them.
/// The closure takes two parameters (`a` of type `i32` and `b` of type `String`) and prints the values of `x`, `y`, `a`, and `b`.
///
// utils.rs
#[macro_export]
macro_rules! clone {
    // Match `@strong` token and clone the variable
    (@strong $($n:ident),+ => move || $body:expr) => {
        {
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
