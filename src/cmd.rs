use crate::{
    core::{Cow, String, Vec},
    ParseError,
};

// Single quotes are not used at all by the cmd.exe command processor except to enclose the command
// to run within a FOR /F statement:
//
//     for /f %%A in ('someCommand') do ...
//
// Or to specify a string to process by FOR /F if the USEBACKQ option is used:
//
//     for /f "usebackq" %%A in ('some string') do ...
//
// Here are some important notes about using double quotes " with cmd.exe:
//
// 1. Double quotes are a simple state machine. The first " turns quoting on, the next turns it
//    off.
// 2. All special characters are treated as literal values when quoted, except for the % and
//    <newLine> characters always, and the ! character if delayed expansion is enabled, and the ^
//    character if delayed expansion is enabled and ! also appears somewhere within the line.
// 3. If quoting is off, then you can escape a " as ^" to prevent it from turning quoting on. But
//    once quoting is on, you cannot escape the closing ". The very next " will always turn quoting
//    off.
const SINGLE_QUOTE: char = '\'';
const DOUBLE_QUOTE: char = '"';

// Delimiters for arguments - any of these will work!
const DELIMITER_COMMA: char = ',';
const DELIMITER_SEMICOLON: char = ':';
const DELIMITER_EQUALS: char = '=';
const DELIMITER_SPACE: char = ' ';
const DELIMITER_TAB: char = '\t';

// Escape for use with itself and the command characters
const ESCAPE: char = '^';
const COMMAND_COLON: char = ':';
const COMMAND_AMPERSAND: char = '&';
const COMMAND_BACKSLASH: char = '\\';
const COMMAND_LESSTHAN: char = '<';
const COMMAND_GREATERTHAN: char = '>';
const COMMAND_CARET: char = '^';
const COMMAND_PIPE: char = '|';

#[derive(Default)]
struct Split<'a> {
    ///
    inner: &'a str,

    /// Position within inner str
    idx: usize,
}

impl<'a> Split<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { inner: s, idx: 0 }
    }
}

impl<'a> core::iter::Iterator for Split<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub fn split(s: &str) -> impl core::iter::Iterator<Item = &str> {
    Split::new(s)
}

#[inline]
fn is_delimiter_char(c: char) -> bool {
    c == DELIMITER_COMMA
        || c == DELIMITER_SEMICOLON
        || c == DELIMITER_EQUALS
        || c == DELIMITER_SPACE
        || c == DELIMITER_TAB
}

#[inline]
fn is_escape_char(c: char) -> bool {
    c == ESCAPE
}

#[inline]
fn is_command_char(c: char) -> bool {
    c == COMMAND_COLON
        || c == COMMAND_AMPERSAND
        || c == COMMAND_BACKSLASH
        || c == COMMAND_LESSTHAN
        || c == COMMAND_GREATERTHAN
        || c == COMMAND_CARET
        || c == COMMAND_PIPE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_entire_string_if_single_word() {
        todo!();
    }

    #[test]
    fn should_split_by_delimiters() {
        todo!();
    }

    #[test]
    fn should_support_quoted_words_with_delimiters_inside() {
        todo!();
    }

    #[test]
    fn should_support_escaping_command_characters() {
        todo!("should support ^| or ^");
    }

    #[test]
    fn should_support_escaping_crlf_endings() {
        todo!(r"should support ^\r\n at end of line");
    }

    #[test]
    fn should_support_escaping_the_percent_character() {
        todo!("should support %%");
    }

    #[test]
    fn should_support_escaping_the_escape_character() {
        todo!("should support ^^");
    }

    #[test]
    fn should_support_double_quotes_to_group_words() {
        todo!(r#"should support "some words" in quotes"#);
    }

    #[test]
    fn should_support_adjacent_double_quotes_to_escape_double_quotes() {
        todo!(r#"Should support "" as an escape"#);
    }

    #[test]
    fn should_support_escaping_exclamation_marks() {
        todo!("Needs to be ^^!");
    }
}
