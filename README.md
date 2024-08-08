# ollama-rs-gtk

A sleek Rust application leveraging GTK for a user-friendly graphical interface, seamlessly integrating with the Ollama API to generate AI-powered responses.

<p align="center">
  <img src="https://github.com/user-attachments/assets/b54daf9b-a98b-49c1-acd1-ee09fd69784e" alt="ollama-rs-gtk" width="300">
</p>

## Features

- **Intuitive GUI**: Built with GTK for a smooth user experience.
- **AI Integration**: Harnesses the power of Ollama's language model for intelligent responses.
- **Cross-Platform**: Runs on various operating systems supporting Rust and GTK.

## Prerequisites

Ensure you have the following installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [GTK3](https://www.gtk.org/docs/installations/) (version 3.24 or higher)
- [Ollama](https://ollama.ai/download) (make sure the Ollama service is running)

## Installation

1. Clone the repository:
   ```shell
   git clone https://github.com/AndresCdo/ollama-rs-gtk.git
   ```

2. Navigate to the project directory:
   ```shell
   cd ollama-rs-gtk
   ```

3. Build the application:
   ```shell
   cargo build --release
   ```

## Usage

1. Start the Ollama service (if not already running).

2. Launch the application:
   ```shell
   cargo run --release
   ```

3. Enter your prompt in the text field and click "Send" to generate a response.

## Configuration

- To modify API endpoints or model parameters, edit the `config.toml` file in the project root.

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-branch-name`.
3. Make your changes and commit them: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-branch-name`.
5. Submit a pull request.

## Troubleshooting

- If you encounter GTK-related errors, ensure your GTK installation is correct and up-to-date.
- For API issues, check your internet connection and verify that the Ollama service is running.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [GTK-rs](https://gtk-rs.org/) for Rust bindings to GTK
- [Ollama](https://ollama.ai/) for providing the AI model API
- All contributors who have helped shape this project

## Contact

For support or queries, please open an issue on the GitHub repository or contact the maintainer at [andres.felipe.caicedo.ultengo@outlook.com].