// https://github.com/Stebalien/horrorshow-rs
use horrorshow::{html, Raw};
use horrorshow::helper::doctype;

// https://github.com/kivikakk/comrak
use comrak::{markdown_to_html, ComrakOptions, ComrakExtensionOptions, ComrakRenderOptions};

// https://github.com/sindresorhus/github-markdown-css
const GITHUB_CSS: &str = include_str!("github-markdown-dark.css");
const HIGHLIGHT_CSS: &str = r#"
.hljs {
    display: block;
    overflow-x: auto;
    padding: 0.5em;
    color: #abb2bf;
    background: #282c34;
}

.hljs-comment, .hljs-quote {
    color: #5c6370;
    font-style: italic;
}

.hljs-keyword, .hljs-selector-tag, .hljs-subst {
    color: #c678dd;
    font-weight: bold;
}

.hljs-number, .hljs-literal, .hljs-variable, .hljs-template-variable, .hljs-tag .hljs-attr {
    color: #d19a66;
}

.hljs-string, .hljs-doctag {
    color: #98c379;
}

.hljs-title, .hljs-section, .hljs-selector-id {
    color: #e06c75;
    font-weight: bold;
}

.hljs-subst {
    font-weight: normal;
}

.hljs-type, .hljs-class .hljs-title {
    color: #e5c07b;
    font-weight: bold;
}

.hljs-tag, .hljs-name, .hljs-attribute {
    color: #61aeee;
    font-weight: normal;
}

.hljs-regexp, .hljs-link {
    color: #56b6c2;
}

.hljs-symbol, .hljs-bullet {
    color: #d19a66;
}

.hljs-built_in, .hljs-builtin-name {
    color: #e06c75;
}

.hljs-meta {
    color: #abb2bf;
    font-weight: bold;
}

.hljs-deletion {
    background: #e06c75;
    color: #282c34;
}

.hljs-addition {
    background: #98c379;
    color: #282c34;
}

.hljs-emphasis {
    font-style: italic;
}

.hljs-strong {
    font-weight: bold;
}
"#;



#[derive(Clone, Debug)]
/// The `Preview` struct represents a markdown previewer.
pub struct Preview {
    comrak_options: ComrakOptions,
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
        format!(
            "{}",
            html!(
                : doctype::HTML;
                html {
                    head {
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js") {}
                        script(src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/languages/rust.min.js") {}
                        script {
                            : Raw("hljs.initHighlightingOnLoad()")
                        }
                        style {
                            : GITHUB_CSS;
                            : HIGHLIGHT_CSS;
                            : "body { width: 90%; margin: 0 auto; } img { max-width: 90% }";
                        }
                    }
                    body {
                        article(class="markdown-body") {
                            : Raw(markdown_to_html(markdown, &self.comrak_options));
                        }
                    }
                }
            )
        )
    }
}
