// https://github.com/Stebalien/horrorshow-rs

// https://github.com/kivikakk/comrak
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};

use ammonia;
use handlebars::Handlebars;
use serde_json::json;

// https://github.com/sindresorhus/github-markdown-css
const GITHUB_CSS: &str = include_str!("github-markdown-dark.css");
const HIGHLIGHT_CSS: &str = include_str!("highlight.css");

#[derive(Clone, Debug)]
/// The `Preview` struct represents a markdown previewer.
pub struct Preview {
    comrak_options: ComrakOptions,
}

impl Default for Preview {
    fn default() -> Self {
        let comrak_render_options = ComrakRenderOptions {
            hardbreaks: true,
            ..ComrakRenderOptions::default()
        };

        let comrak_options = ComrakOptions {
            render: comrak_render_options,
            ..ComrakOptions::default()
        };

        Preview { comrak_options }
    }
}

impl Preview {
    /// Creates a new `Preview` instance.
    pub fn new() -> Preview {
        let comrak_render_options = ComrakRenderOptions {
            hardbreaks: true,
            ..ComrakRenderOptions::default()
        };
        let comrak_extension_options = ComrakExtensionOptions {
            table: true,
            strikethrough: true,
            ..ComrakExtensionOptions::default()
        };
        let comrak_options = ComrakOptions {
            render: comrak_render_options,
            extension: comrak_extension_options,
            ..ComrakOptions::default()
        };

        Preview { comrak_options }
    }

    /// Renders the given markdown as HTML.
    ///
    /// # Arguments
    ///
    /// * `markdown` - The markdown content to render.
    ///
    /// # Returns
    ///
    /// The rendered HTML as a string.
    pub fn render(&self, markdown: &str) -> String {
        // Sanitize the input markdown
        let sanitized_markdown = ammonia::clean(markdown);

        // Use a HTML templating engine that automatically escapes input
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("page", r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js" integrity="sha384-ZeLYJ2PNSQjvogWP559CDAf02Qb8FE5OyQicqtz/+UhZutbrwyr87Be7NPH/RgyC" crossorigin="anonymous"></script>
                    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js" integrity="sha384-OBJOKgNdLyh0+KeF4HV9qlOEPvj6VyfuPSI/Yz+Tr2mOqwbRDqGsMtYlKz3tZkA" crossorigin="anonymous"></script>
                    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/bash.min.js" integrity="sha384-eQ7mmCQvBDl4XhA/Lxe6YfLK09TdqV3GBk9L3af17KsbqtWIrBjbvG/hzGSuJsO" crossorigin="anonymous"></script>
                    <script>hljs.initHighlightingOnLoad();</script>
                    <style>
                        {{github_css}}
                        {{highlight_css}}
                        body { padding: 20px; }
                    </style>
                </head>
                <body>
                    <article class="markdown-body">
                        {{{content}}}
                    </article>
                </body>
            </html>
        "#).expect("Failed to register template");

        let html_content = markdown_to_html(&sanitized_markdown, &self.comrak_options);

        let data = json!({
            "github_css": GITHUB_CSS,
            "highlight_css": HIGHLIGHT_CSS,
            "content": html_content
        });

        handlebars
            .render("page", &data)
            .unwrap_or_else(|_| "Rendering failed".to_string())
    }
}
