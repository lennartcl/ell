# lil: A Rust-based shell-to-LLM helper

`lil` is a rewrite of the bash-based `ell` tool in Rust. It provides a set of utilities to interact with Large Language Models (LLMs) from your shell.

This project is a port of the original [ell by simonmysun](https://github.com/simonmysun/ell).

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- The `script` command (usually available via `util-linux` on Linux or comes with BSD/macOS)

### Building

1. Clone the repository:
   ```bash
   git clone https://github.com/lennartcl/ell.git
   cd ell/lil
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. The binary will be available at `target/release/lil`.

## Installation

(Instructions to be added once the project is ready for distribution.)

## Usage

`lil` provides several modes of operation, implemented as subcommands.

### Pipe Mode

Pipe any text to `lil pipe` to send it to an LLM as a prompt.

```bash
echo "What is the capital of France?" | lil pipe
```

### Record Mode

Start a new recorded shell session using `lil record`. The session will be saved to a log file.

```bash
lil record
```

This will launch a new shell. When you exit the shell, the recording will stop.

**Note for non-TTY environments:** In environments without a TTY (like CI/CD pipelines or some sandboxes), `lil record` will create a simulated log file for testing purposes, as the underlying `script` command requires a real terminal to function.

### Interactive Mode

Use `lil interactive` to start an interactive session with the LLM. In this mode, `lil` will use the content of the session log file as context for your prompts.

```bash
lil interactive
```

You can then type your prompts at the `>` prompt.

### Doctor

Run `lil doctor` to check for common environment issues and dependencies, like the `script` command.

```bash
lil doctor
```

## Configuration

`lil` can be configured by creating a `lil.toml` file at `~/.config/lil/lil.toml`.

You can customize the prompt template using the `liquid` template language.

Example `lil.toml`:
```toml
prompt = """
You are a shell assistant. Based on the following session history:
{{context}}

Please respond to the user's request: '{{user_input}}'
"""
```
