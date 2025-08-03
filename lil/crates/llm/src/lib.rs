use anyhow::Result;

pub async fn call(input: &str) -> Result<String> {
    // Simulate an LLM call
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    Ok(format!("LLM response for: '{}'", input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_llm() {
        let input = "hello";
        let response = call(input).await.unwrap();
        assert!(response.contains(input));
    }
}
