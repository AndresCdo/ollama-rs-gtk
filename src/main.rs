use gtk::{Application, ApplicationWindow, Button, Entry, Orientation, ScrolledWindow, TextView};
use reqwest::Client;
use serde_json::{json, Value};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;
use tokio::runtime::Runtime;
use glib::MainContext;
use std::sync::{Arc, Mutex};

use gtk::prelude::*;

const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const REQUEST_TIMEOUT: u64 = 10000; // Timeout in seconds

async fn send_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()?;

    let response = client.post(OLLAMA_API_URL)
        .json(&json!({
            "model": "llama3",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await?;

    let json: Value = response.json().await?;
    Ok(json["response"].as_str().unwrap_or("Error: No response").to_string())
}

fn main() {
    let app = Application::new(Some("com.example.ollama_llama3_assistant"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Ollama Llama 3 Assistant");
        window.set_default_size(400, 300);
        
        let hbox = gtk::Box::new(Orientation::Horizontal, 5);
        window.add(&hbox);

        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        hbox.pack_start(&vbox, true, true, 5);

        let input_entry = Rc::new(RefCell::new(Entry::new()));
        vbox.pack_start(&*input_entry.borrow(), false, false, 0);

        let send_button = Button::with_label("Send");
        vbox.pack_start(&send_button, false, false, 0);

        // Shared state to control button sensitivity across threads
        let button_state = Arc::new(Mutex::new(send_button.clone()));

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

        let main_context = MainContext::default();
        let runtime = Arc::new(Runtime::new().unwrap());

        send_button.connect_clicked(move |_| {
            let prompt = input_entry_clone.borrow().text().to_string();
            let prompt_clone = prompt.clone();

            let  button = button_state.lock().unwrap();
            button.set_sensitive(false);

            let button_state_clone = Arc::clone(&button_state);

            if !prompt.is_empty() {
                let text_buffer_clone = Rc::clone(&text_buffer_clone);
                let input_entry_clone = Rc::clone(&input_entry_clone);
                let runtime = runtime.clone();

                main_context.spawn_local(async move {
                    let button = button_state_clone.lock().unwrap();
                    match runtime.spawn(async move {
                        send_prompt(&prompt).await.unwrap()
                    }).await {
                        Ok(response) => {
                            let text_buffer = text_buffer_clone.borrow_mut();
                            let mut end_iter = text_buffer.end_iter();
                            text_buffer.insert_markup(&mut end_iter, &format!("You: {}\n\n", prompt_clone));
                            text_buffer.insert_markup(&mut end_iter, &format!("Ollama: {}\n\n", response));
                        },
                        Err(e) => {
                            let text_buffer = text_buffer_clone.borrow_mut();
                            let mut end_iter = text_buffer.end_iter();
                            text_buffer.insert_markup(&mut end_iter, &format!("Error: {}\n\n", e));
                        }
                    }

                    button.set_sensitive(true);
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
