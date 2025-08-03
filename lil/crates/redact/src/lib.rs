use lazy_static::lazy_static;
use regex::{Regex, RegexSet};

lazy_static! {
    static ref PATTERNS: Vec<&'static str> = vec![
        // UUID
        r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
        // JWT
        r"ey[a-zA-Z0-9_-]{10,}\.ey[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]+",
        // IP Address
        r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b",
        // Email
        r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b",
        // Phone numbers (basic)
        r"\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}",
    ];

    static ref RE_SET: RegexSet = RegexSet::new(&*PATTERNS).unwrap();
    // We need individual Regex objects for replacement
    static ref REGEXES: Vec<Regex> = PATTERNS.iter().map(|p| Regex::new(p).unwrap()).collect();
}

pub fn redact(s: &str) -> String {
    let mut redacted_s = s.to_string();
    let matches = RE_SET.matches(s);

    // Iterate over the patterns that matched
    for pattern_index in matches.iter() {
        let re = &REGEXES[pattern_index];
        redacted_s = re.replace_all(&redacted_s, "[REDACTED]").to_string();
    }

    redacted_s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redact_uuid() {
        let input = "Here is a uuid: 123e4567-e89b-12d3-a456-426614174000";
        let expected = "Here is a uuid: [REDACTED]";
        assert_eq!(redact(input), expected);
    }

    #[test]
    fn test_redact_jwt() {
        let input = "Token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let expected = "Token: [REDACTED]";
        assert_eq!(redact(input), expected);
    }

    #[test]
    fn test_redact_ip() {
        let input = "An IP address is 192.168.1.1";
        let expected = "An IP address is [REDACTED]";
        assert_eq!(redact(input), expected);
    }

    #[test]
    fn test_redact_email() {
        let input = "Contact me at test@example.com";
        let expected = "Contact me at [REDACTED]";
        assert_eq!(redact(input), expected);
    }

    #[test]
    fn test_no_redaction() {
        let input = "This is a normal sentence.";
        assert_eq!(redact(input), input);
    }

    #[test]
    fn test_multiple_redactions() {
        let input = "My email is test@example.com and my ip is 127.0.0.1";
        let expected = "My email is [REDACTED] and my ip is [REDACTED]";
        assert_eq!(redact(input), expected);
    }
}
