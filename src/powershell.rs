use crate::lib;

/// Escapes special characters in a string, so that it will retain its literal
/// meaning when used as a part of command in PowerShell.
pub fn quote(s: &str) -> lib::String {
    // Wrap in single quotes and escape single quotes.
    let mut quoted = lib::String::from("'");
    for c in s.chars() {
        match c {
            '\'' => {
                // Single quotes are escaped by doubling them.
                quoted.push_str("''");
            }
            _ => {
                quoted.push(c);
            }
        }
    }
    quoted.push('\'');
    quoted
}

/// Splits according to [Microsoft quoting rules][rules].
///
/// [rules]: https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_quoting_rules?view=powershell-7.3.
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
            '`' if in_quotes => {
                // Only consider as escape character when it's in quotes
                if let Some(next) = chars.next() {
                    arg.push(next);
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
        assert_eq!(quote(""), "''");
        assert_eq!(quote("hello"), "'hello'");
        assert_eq!(quote("hello world"), "'hello world'");

        // Special characters
        assert_eq!(quote("'"), "''''");
        assert_eq!(quote("`"), "'`'");

        // Multiple special characters
        assert_eq!(quote("''"), "''''''''");
    }

    #[test]
    fn should_split_across_multiple_situations() {
        // Basic tests
        assert_eq!(
            split("Get-Process -Name \"My Process\""),
            vec!["Get-Process", "-Name", "My Process"]
        );

        // Escaping with backtick
        assert_eq!(
            split("\"This is a `\"quote`\".\""),
            vec!["This is a \"quote\"."]
        );

        // Mixed quotes and spaces
        assert_eq!(
            split("\"C:\\Program Files\\App\" -Arg \"dir /s\""),
            vec!["C:\\Program Files\\App", "-Arg", "dir /s"]
        );
    }
}
