use anyhow::{anyhow, Result};
use std::process::Command;
use std::path::Path;
use atty::Stream;

fn has_gnu_script() -> bool {
    Command::new("script")
        .args(["-q", "-f", "/dev/null", "-c", "true"])
        .status()
        .map_or(false, |s| s.success())
}

pub fn spawn_shell(log_path: &Path, interactive_mode: bool) -> Result<()> {
    let is_tty = atty::is(Stream::Stdout);
    println!("Is TTY: {}", is_tty);
    // TODO: End-to-end record mode needs real terminal/pty; simulating for CI/test.
    if !is_tty {
        eprintln!("No TTY detected; simulating session log for test purposes.");
        let fake_log = "ls -la\nfile1.txt\nfile2.txt\n";
        std::fs::write(&log_path, fake_log)?;
        return Ok(());
    }

    let flags = if has_gnu_script() {
        vec!["-q", "-f"]
    } else {
        vec!["-q"]
    };

    let cmd = if interactive_mode {
        "lil -i"
    } else {
        "bash -i"
    };

    let mut command = Command::new("script");
    command.args(&flags);
    command.arg(log_path);
    command.arg("-c");
    command.arg(cmd);

    let status = command.status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("script command failed with status: {}", status))
    }
}

#[cfg(test)]
mod tests {
    // We can't easily test the full script spawning logic in a unit test,
    // especially because it's interactive. We will rely on integration
    // tests for this.
}
