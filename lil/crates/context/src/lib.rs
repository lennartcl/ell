use anyhow::Result;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

const CONTEXT_SIZE: u64 = 3072; // 3kB

pub async fn get_context(log_path: &Path) -> Result<String> {
    let mut file = File::open(log_path).await?;
    let metadata = file.metadata().await?;

    let file_size = metadata.len();
    let seek_pos = if file_size > CONTEXT_SIZE {
        file_size - CONTEXT_SIZE
    } else {
        0
    };

    file.seek(SeekFrom::Start(seek_pos)).await?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    let content_with_ansi = String::from_utf8_lossy(&buffer);
    let stripped_bytes = strip_ansi_escapes::strip(content_with_ansi.as_bytes());
    let result_string = String::from_utf8_lossy(&stripped_bytes).to_string();
    Ok(result_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_get_context() {
        let temp_dir = std::env::temp_dir();
        let log_path = temp_dir.join("test_context.log");

        let mut file = File::create(&log_path).await.unwrap();
        file.write_all(b"some initial text that will be ignored because it is too old").await.unwrap();
        for i in 0..100 {
            file.write_all(format!("\x1b[31mline {}\x1b[0m\n", i).as_bytes()).await.unwrap();
        }
        file.sync_all().await.unwrap();

        let context = get_context(&log_path).await.unwrap();

        // Check that ANSI codes are stripped
        assert!(!context.contains("\x1b[31m"));

        // Check that we got the end of the file
        assert!(context.contains("line 99"));

        // Check that we didn't get the start of the file
        assert!(!context.contains("initial text"));

        std::fs::remove_file(&log_path).unwrap();
    }
}
