use crate::lib;

/// Escapes special characters in a string, so that it will retain its literal
/// meaning when used as a part of command in Windows command line.
pub fn quote(s: &str) -> lib::String {
    // Wrap in double quotes and escape double quotes and backslashes.
    let mut quoted = lib::String::from("\"");
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // Backslashes need to be escaped, but only if they precede a double quote.
                // If we find one or more backslashes and the next character is a double quote,
                // we double the number of backslashes and escape the double quote.
                let mut num_backslashes = 1;
                while chars.peek() == Some(&'\\') {
                    num_backslashes += 1;
                    chars.next();
                }
                if chars.peek() == Some(&'"') {
                    // Double the number of backslashes and escape the double quote.
                    quoted.extend(std::iter::repeat('\\').take(num_backslashes * 2));
                    quoted.push_str("\\\"");
                    chars.next();
                } else {
                    // Just include the backslashes as they are.
                    quoted.extend(std::iter::repeat('\\').take(num_backslashes));
                }
            }
            '"' => {
                // Double quotes need to be escaped.
                quoted.push_str("\\\"");
            }
            _ => {
                quoted.push(c);
            }
        }
    }

    quoted.push('"');
    quoted
}

/// Splits according to .
pub fn split(s: &str) -> lib::Vec<lib::String> {
    let mut args = lib::Vec::new();
    let mut arg = lib::String::new();
    let mut chars = s.chars().peekable();
    let mut in_quotes = false;

    while let Some(c) = chars.next() {
        match c {
            ' ' if !in_quotes => {
                if !arg.is_empty() {
                    args.push(arg);
                    arg = String::new();
                }
            }
            '"' => in_quotes = !in_quotes,
            '\\' => {
                if in_quotes && chars.peek() == Some(&'"') {
                    // Escape double quote within quotes
                    chars.next();
                    arg.push('"');
                } else {
                    arg.push('\\');
                }
            }
            _ => arg.push(c),
        }
    }

    if !arg.is_empty() {
        args.push(arg);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_quote_across_multiple_situations() {
        // Basic tests
        assert_eq!(quote(""), "\"\"");
        assert_eq!(quote("hello"), "\"hello\"");
        assert_eq!(quote("hello world"), "\"hello world\"");

        // Special characters
        assert_eq!(quote("\\\""), "\"\\\\\\\"\"");
        assert_eq!(quote("\""), "\"\\\"\"");
        assert_eq!(quote("C:\\Program Files\\"), "\"C:\\\\Program Files\\\\\"");

        // Escaping sequence
        assert_eq!(quote("\\\\\\\""), "\"\\\\\\\\\\\\\\\"\"");
    }

    #[test]
    fn should_split_across_multiple_situations() {
        // Basic tests
        assert_eq!(split("cmd /C dir"), vec!["cmd", "/C", "dir"]);
        assert_eq!(
            split("\"C:\\Program Files\\App\""),
            vec!["C:\\Program Files\\App"]
        );

        // Escaping double quotes
        assert_eq!(
            split("\"This is a \\\"quote\\\".\""),
            vec!["This is a \"quote\"."]
        );

        // Escaping backslash
        assert_eq!(split("C:\\\\dir\\\\file"), vec!["C:\\dir\\file"]);

        // Mixed quotes and spaces
        assert_eq!(
            split("\"C:\\Program Files\\App\" /C \"dir /s\""),
            vec!["C:\\Program Files\\App", "/C", "dir /s"]
        );
    }
}
