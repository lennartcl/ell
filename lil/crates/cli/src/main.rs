use clap::Parser;
use std::io::{self, Read, Write};
use anyhow::Result;

use clap::{Parser, Subcommand};

/// A Rust rewrite of *ell
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Pipe mode: read from stdin, send to LLM, print response
    Pipe,
    /// Record mode: record a shell session
    Record {
        /// Start an interactive session inside the recording
        #[arg(short, long)]
        interactive: bool,
    },
    /// Interactive mode: start an interactive session
    Interactive,
    /// Doctor: check for environment issues
    Doctor,
}


#[tokio::main]
async fn main() -> Result<()> {
    println!("lil CLI starting...");
    let cli = Cli::parse();

    match cli.command {
        Commands::Pipe => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;

            let redacted = redact::redact(&input);
            let response = llm::call(&redacted).await?;

            println!("{}", response);
        }
        Commands::Record { interactive } => {
            let log_path = std::env::temp_dir().join("lil_session.log");
            println!("Recording session to: {}", log_path.display());
            record::spawn_shell(&log_path, interactive)?;
            println!("Session recording finished.");
        }
        Commands::Interactive => {
            let config = config::load().await?;
            let log_path = std::env::temp_dir().join("lil_session.log");

            // Create a dummy log file for testing if it doesn't exist
            if !log_path.exists() {
                tokio::fs::write(&log_path, "Dummy log file for interactive mode.\n").await?;
            }

            let mut user_input = String::new();
            loop {
                print!("> ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut user_input)?;

                let context = context::get_context(&log_path).await?;
                let prompt = config.render_prompt(&context, &user_input)?;
                let response = llm::call(&prompt).await?;

                println!("{}", response);
                user_input.clear();
            }
        }
        Commands::Doctor => {
            println!("Running lil doctor...");
            println!("Checking for `script` command...");
            let script_path = std::process::Command::new("which")
                .arg("script")
                .output();

            match script_path {
                Ok(output) if output.status.success() => {
                    let path = String::from_utf8_lossy(&output.stdout);
                    println!("✅ `script` command found at: {}", path.trim());
                }
                _ => {
                    println!("❌ `script` command not found. Record mode will not work.");
                }
            }
        }
    }

    Ok(())
}
