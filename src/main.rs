use gtk::{prelude::*, Application, ApplicationWindow, Builder, Entry, Button, TextView};
use reqwest::Client;
use serde_json::{json, Value};
use std::{sync::{Arc, Mutex}, time::Duration};
use tokio::runtime::Runtime;
use glib::{clone, MainContext};
use anyhow::{Context, Result};

const OLLAMA_API_URL: &str = "http://localhost:11434/api/generate";
const REQUEST_TIMEOUT: u64 = 10000;

async fn send_prompt(prompt: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .build()
        .context("Failed to build HTTP client")?;

    let response = client
        .post(OLLAMA_API_URL)
        .json(&json!({
            "model": "llama3.1",
            "prompt": prompt,
            "stream": false
        }))
        .send()
        .await
        .context("Failed to send request")?;

    let json: Value = response.json().await.context("Failed to parse JSON response")?;
    Ok(json["response"].as_str().unwrap_or("Error: No response").to_string())
}

fn build_ui(app: &Application) -> (ApplicationWindow, Arc<Mutex<Entry>>, Arc<Mutex<Button>>, Arc<Mutex<TextView>>) {
    let builder = Builder::new();
    builder.add_from_string(include_str!("ui.glade")).expect("Failed to load UI from string");

    let window: ApplicationWindow = builder.object("main_window").expect("Couldn't get main_window");
    let input_entry: Entry = builder.object("input_entry").expect("Couldn't get input_entry");
    let send_button: Button = builder.object("send_button").expect("Couldn't get send_button");
    let text_view: TextView = builder.object("text_view").expect("Couldn't get text_view");
    
    window.set_application(Some(app));

    (
        window,
        Arc::new(Mutex::new(input_entry)),
        Arc::new(Mutex::new(send_button)),
        Arc::new(Mutex::new(text_view)),
    )
}

fn main() -> Result<()> {
    let app = Application::new(Some("com.example.OllamaAssistant"), Default::default());

    app.connect_activate(|app| {
        let (window, input_entry, send_button, text_view) = build_ui(app);

        let text_buffer = text_view.lock().unwrap().buffer().unwrap();
        let text_buffer = Arc::new(Mutex::new(text_buffer));
        let runtime = Runtime::new().expect("Failed to create Tokio runtime");
        let runtime = Arc::new(runtime);

        let send_request = clone!(@strong input_entry, @strong text_buffer, @strong runtime, @strong send_button => move || {
            let prompt = input_entry.lock().unwrap().text().to_string();
            if prompt.is_empty() {
                return;
            }

            send_button.lock().unwrap().set_sensitive(false);
            input_entry.lock().unwrap().set_sensitive(false);

            let text_buffer_clone = Arc::clone(&text_buffer);
            let runtime_clone = Arc::clone(&runtime);
            let send_button_clone = Arc::clone(&send_button);
            let input_entry_clone = Arc::clone(&input_entry);

            let prompt_clone = prompt.clone();
            let future = async move {
                let response = runtime_clone.spawn(async move {
                    send_prompt(&prompt_clone).await
                }).await;

                let response_text = match response {
                    Ok(Ok(response)) => response,
                    Ok(Err(err)) => format!("Error: {}", err),
                    Err(err) => format!("Error: {}", err),
                };

                let text_buffer = text_buffer_clone.lock().unwrap();
                let mut end_iter = text_buffer.end_iter();
                text_buffer.insert(&mut end_iter, &format!("You: {}\n\n", prompt));
                text_buffer.insert(&mut end_iter, &format!("Ollama: {}\n\n", response_text));

                send_button_clone.lock().unwrap().set_sensitive(true);
                input_entry_clone.lock().unwrap().set_sensitive(true);
                input_entry_clone.lock().unwrap().set_text("");
            };

            MainContext::default().spawn_local(future);
        });

        send_button.lock().unwrap().connect_clicked(clone!(@strong send_request => move |_| {
            send_request();
        }));

        input_entry.lock().unwrap().connect_activate(clone!(@strong send_request => move |_| {
            send_request();
        }));

        window.show_all();
    });

    app.run();
    Ok(())
}
