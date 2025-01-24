# Chat-TUI: A Local Chatbot with Functional Caching and gRPC Integration

## Overview
**Chat-TUI** is a terminal-based chatbot designed for Linux systems. The goal of this project is to provide a local, privacy-friendly chatbot with:
- Functional **caching** and **information recall** capabilities.
- gRPC integration to facilitate communication between a Python-based Ollama app and the chatbot engine.
- A lightweight **terminal user interface (TUI)** for seamless interaction.

This project is open-source and welcomes contributions from the community. If you are passionate about privacy-focused, TUI-based applications or want to dive into gRPC communication, this is the project for you!

---

## Features (Planned and In Progress)
1. **TUI-Based Interface**: A simple, fast, and responsive terminal UI for chatbot interactions.
2. **Caching System**: Efficient storage and retrieval of previous conversations.
3. **Recall Functionality**: Ability to recall specific pieces of information from previous interactions.
4. **gRPC Integration**: Use gRPC to enable communication between components written in Python and Rust.
5. **Linux Focus**: Optimized for Linux environments.
6. **Privacy-Oriented**: Fully local chatbot with no external API calls.

---

## Getting Started

### Prerequisites
Before contributing or running the project, ensure you have the following:

#### System Requirements
- Linux-based operating system.
- Terminal emulator (e.g., Alacritty, Kitty, or default terminal).

#### Tools
- **Rust** (for the TUI and other components): Install using [rustup](https://rustup.rs/).
- **Python 3.8+** (for the Ollama integration): Install via your package manager.
- **gRPC and Protocol Buffers**:
  - Install gRPC for Python: `pip install grpcio grpcio-tools`
  - Install gRPC tools for Rust: Follow [tonic gRPC](https://docs.rs/tonic/latest/tonic/)
- **Git**: Version control tool.

#### Clone the Repository
```bash
git clone https://github.com/wicaksonoleksono/chat-tui.git
cd chat-tui
```

---

## How to Contribute
We welcome contributions of all kinds, from beginners to advanced developers. Below is a step-by-step guide for contributing to the project.

### Step 1: Fork and Clone the Repository
1. Fork the repository by clicking the "Fork" button on the GitHub page.
2. Clone your forked repository:
   ```bash
   git clone https://github.com/<your-username>/chat-tui.git
   cd chat-tui
   ```

### Step 2: Set Up the Development Environment

#### Install Dependencies
1. Install Rust dependencies:
   ```bash
   cargo build
   ```
2. Install Python dependencies:
   ```bash
   pip install -r requirements.txt
   ```
3. Compile the Protocol Buffers (if modifying `.proto` files):
   ```bash
   protoc --proto_path=proto/ --python_out=python/ --grpc_python_out=python/ proto/*.proto
   ```

#### Run the Project
To test the chatbot locally:
```bash
cargo run
```
For the Python gRPC server:
```bash
python ollama_server.py
```

---

### Step 3: Choose an Issue to Work On
- Check the [Issues](https://github.com/wicaksonoleksono/chat-tui/issues) tab for a list of tasks.
- Look for issues labeled **"good first issue"** or **"help wanted"**.
- Comment on the issue to let others know you are working on it.

### Step 4: Create a Branch
To keep your work organized:
```bash
git checkout -b feature/<feature-name>
```
Example:
```bash
git checkout -b feature/add-caching
```

### Step 5: Make Your Changes
- Follow the existing coding style and guidelines.
- Add comments to your code for better understanding.
- Test your changes locally to ensure they work as expected.

### Step 6: Commit and Push Your Changes
1. Stage your changes:
   ```bash
   git add .
   ```
2. Commit your changes with a clear message:
   ```bash
   git commit -m "Add caching functionality for chatbot"
   ```
3. Push your branch to GitHub:
   ```bash
   git push origin feature/<feature-name>
   ```

### Step 7: Open a Pull Request
1. Navigate to the original repository on GitHub.
2. Click the **"Compare & pull request"** button.
3. Provide a clear title and description for your pull request.
4. Submit the pull request for review.

### Step 8: Address Feedback
Maintainers may review your pull request and provide feedback. Respond promptly and make necessary changes.

---

## Project Structure
```
chat-tui/
├── src/                 # Rust code for the TUI
├── python/              # Python code for Ollama integration
├── proto/               # Protocol Buffer definitions
├── examples/            # Example conversations and usage
├── docs/                # Documentation
├── tests/               # Test cases
├── Cargo.toml           # Rust package configuration
├── requirements.txt     # Python dependencies
└── README.md            # Project documentation
```

---

## Guidelines for Contributions

### Coding Standards
- Follow Rust's [Rustfmt](https://github.com/rust-lang/rustfmt) for formatting.
- Use Python's [Black](https://black.readthedocs.io/en/stable/) for consistent code formatting.

### Commit Message Format
Use clear and concise commit messages. Follow this format:
```
<type>: <subject>

<body>
```
**Types:**
- `feat`: A new feature.
- `fix`: A bug fix.
- `docs`: Documentation changes.
- `test`: Adding or updating tests.
- `chore`: Maintenance tasks.

### Pull Request Checklist
Before submitting a PR:
- [ ] Code compiles without errors.
- [ ] No linting issues.
- [ ] Test cases added or updated.
- [ ] Documentation updated if applicable.

---

## License
This project is licensed under the [GNU v.3.0](LICENSE).

---


## Community
- **Questions or Suggestions?** Create a new [Discussion](https://github.com/wicaksonoleksono/chat-tui/discussions).
- **Found a Bug?** Open a new [Issue](https://github.com/wicaksonoleksono/chat-tui/issues).

We look forward to your contributions and hope you enjoy building with us!

