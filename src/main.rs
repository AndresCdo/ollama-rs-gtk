use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, TextView, ScrolledWindow, Orientation};
use reqwest::blocking::ClientBuilder;
use serde_json::{json, Value};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const REQUEST_TIMEOUT: u64 = 10000; // Timeout in seconds

async fn send_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()?;

    let response = client.post(OLLAMA_API_URL)
        .json(&json!({
            "model": "llama3",
            "prompt": prompt,
            "stream": false
        }))
        .send()?;

    let json: Value = response.json()?;
    Ok(json["response"].as_str().unwrap_or("Error: No response").to_string())
}

fn main() {
    let app = Application::new(Some("com.example.ollama_llama3_assistant"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Ollama Llama 3 Assistant");
        window.set_default_size(400, 300);

        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        window.add(&vbox);

        let input_entry = Rc::new(RefCell::new(Entry::new()));
        vbox.pack_start(&*input_entry.borrow(), false, false, 0);

        let send_button = Button::with_label("Send");
        vbox.pack_start(&send_button, false, false, 0);

        let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        vbox.pack_start(&scrolled_window, true, true, 0);

        let text_view = TextView::new();
        text_view.set_editable(false);
        text_view.set_wrap_mode(gtk::WrapMode::Word);
        scrolled_window.add(&text_view);

        let text_buffer = Rc::new(RefCell::new(text_view.buffer().unwrap()));

        let send_button_weak = send_button.downgrade();
        let input_entry_clone = Rc::clone(&input_entry);
        let text_buffer_clone = Rc::clone(&text_buffer);

        send_button.connect_clicked(move |_| {
            let prompt = input_entry_clone.borrow().text().to_string();
            if !prompt.is_empty() {
                futures::executor::block_on(async {
                    match send_prompt(&prompt).await {
                        Ok(response) => {
                            let text_buffer = text_buffer_clone.borrow_mut();
                            let mut end_iter = text_buffer.end_iter();
                            text_buffer.insert_markup(&mut end_iter, &format!("You: {}\n", prompt));
                            text_buffer.insert_markup(&mut end_iter, &format!("Ollama: {}\n", response));
                        },
                        Err(e) => {
                            let text_buffer = text_buffer_clone.borrow_mut();
                            let mut end_iter = text_buffer.end_iter();
                            text_buffer.insert_markup(&mut end_iter, &format!("Error: {}\n", e));
                        }
                    }
                });                
                input_entry_clone.borrow_mut().set_text("");
            }
        });

        input_entry.borrow().connect_activate(move |_| {
            if let Some(send_button) = send_button_weak.upgrade() {
                send_button.emit_clicked();
            }
        });

        window.show_all();
    });

    app.run();
}