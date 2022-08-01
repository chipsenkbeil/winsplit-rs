use crate::lib;

/// Result of parsing a str into arguments
pub type ParseResult = Result<lib::Vec<lib::String>, ParseError>;

/// An error returned during parsing
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    ArgNotEmptyInInitialState,
    CommandNameBackslash,
    ReachedUnescapedNewline,
}

impl lib::fmt::Display for ParseError {
    fn fmt(&self, f: &mut lib::fmt::Formatter) -> lib::fmt::Result {
        match self {
            Self::ArgNotEmptyInInitialState => write!(f, "Arg not empty in initial state"),
            Self::CommandNameBackslash => write!(f, "Encountered special backslash during command name, but should be considered normal"),
            Self::ReachedUnescapedNewline => write!(f, "Reached unescaped newline"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// Port of llvm's
/// [cl::tokenizeWindowsCommandLine](https://llvm.org/doxygen/namespacellvm_1_1cl.html#a3b42fd69f84c0ceef44857e925613ee4) function
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Parser {
    /// If true, treats first argument as executable path, which has special handling;
    /// otherwise, will assume entire inner str is just arguments post-executable path
    initial_command_name: bool,
}

impl Parser {
    /// Creates a parser just for arguments
    pub fn new() -> Self {
        Self {
            initial_command_name: false,
        }
    }

    /// Creates a parser that will also parse an executable path at the beginning
    pub fn full() -> Self {
        Self {
            initial_command_name: true,
        }
    }

    /// Parses a command line string into arguments, consuming the parser
    ///
    /// If the parser expects an executable path at the beginning, then the first string in the
    /// result will represent the executable path
    pub fn parse(self, s: &str) -> ParseResult {
        enum State {
            Init,
            Quoted,
            Unquoted,
        }

        let mut chars = s.chars();
        let mut args = lib::Vec::new();
        let mut arg = lib::String::new();
        let mut state = State::Init;
        let mut command_name = self.initial_command_name;
        let mut remaining = s.chars().count();

        /// If no arguments, will retrieve next character, or break out of loop if no more
        /// characters
        ///
        /// If expression provided, will retrieve next character, or fail with given error
        /// expression
        macro_rules! next_char {
            () => {{
                match chars.next() {
                    Some(c) => {
                        if remaining > 0 {
                            remaining -= 1;
                        }
                        c
                    }
                    None => break,
                }
            }};

            ($err:expr) => {{
                match chars.next() {
                    Some(c) => {
                        if remaining > 0 {
                            remaining -= 1;
                        }
                        c
                    }
                    None => return Err($err),
                }
            }};
        }

        macro_rules! has_more_chars {
            () => {
                remaining > 0
            };
        }

        macro_rules! no_more_chars {
            () => {
                remaining == 0
            };
        }

        macro_rules! store_arg {
            () => {{
                args.push(arg);
                arg = lib::String::new();
            }};
        }

        #[inline]
        fn is_whitespace(c: char) -> bool {
            c == ' ' || c == '\t' || c == '\r' || c == '\n'
        }

        #[inline]
        fn is_whitespace_or_null(c: char) -> bool {
            is_whitespace(c) || c == '\0'
        }

        // Windows treats whitespace, double quotes, and backslashes specially, except
        // when parsing the first token of a full command line, in which case
        // backslashes are not special.
        #[inline]
        fn is_special(c: char) -> bool {
            is_whitespace_or_null(c) || c == '\\' || c == '"'
        }

        #[inline]
        fn is_special_in_command_name(c: char) -> bool {
            is_whitespace_or_null(c) || c == '"'
        }

        /// Backslashes are interpreted in a rather complicated way in the Windows-style
        /// command line, because backslashes are used both to separate path and to
        /// escape double quote. This method consumes runs of backslashes as well as the
        /// following double quote if it's escaped.
        ///
        ///  * If an even number of backslashes is followed by a double quote, one
        ///    backslash is output for every pair of backslashes, and the last double
        ///    quote remains unconsumed. The double quote will later be interpreted as
        ///    the start or end of a quoted string in the main loop outside of this
        ///    function.
        ///
        ///  * If an odd number of backslashes is followed by a double quote, one
        ///    backslash is output for every pair of backslashes, and a double quote is
        ///    output for the last pair of backslash-double quote. The double quote is
        ///    consumed in this case.
        ///
        ///  * Otherwise, backslashes are interpreted literally.
        macro_rules! parse_backslash {
     () => {{
         // Total number of backslashes
         let mut cnt = 0;

         while c == '\\' {
             cnt += 1;
             c = next_char!();
         }

         let followed_by_double_quote = has_more_chars!()

   bool FollowedByDoubleQuote = (I != E && Src[I] == '"');
   if (FollowedByDoubleQuote) {
     Token.append(BackslashCount / 2, '\\');
     if (BackslashCount % 2 == 0)
       return I - 1;
     Token.push_back('"');
     return I;
   }
   Token.append(BackslashCount, '\\');
   return I - 1;
     }};
 }

        loop {
            // Get next character, exiting if we have run out of characters
            let mut c = next_char!();

            match state {
                State::Init => {
                    if !arg.is_empty() {
                        return Err(ParseError::ArgNotEmptyInInitialState);
                    }

                    // Consume whitespace before argument
                    while is_whitespace_or_null(c) {
                        if c == '\n' {
                            return Err(ParseError::ReachedUnescapedNewline);
                        }
                        c = next_char!();
                    }

                    if no_more_chars!() {
                        break;
                    }

                    // Build up normal characters
                    if command_name {
                        while !is_special_in_command_name(c) {
                            arg.push(c);
                            c = next_char!();
                        }
                    } else {
                        while !is_special(c) {
                            arg.push(c);
                            c = next_char!();
                        }
                    }

                    if no_more_chars!() || is_whitespace_or_null(c) {
                        store_arg!();
                    } else if c == '"' {
                        state = State::Quoted;
                    } else if c == '\\' {
                        if command_name {
                            return Err(ParseError::CommandNameBackslash);
                        }
                        state = State::Unquoted;
                    } else {
                        unreachable!("unexpected special character");
                    }
                }

                State::Quoted => {}

                State::Unquoted => {}
            }
        }

        // If we have one more active argument and not initializing, add it to our list
        if !matches!(state, State::Init) && !arg.is_empty() {
            args.push(arg);
        }

        Ok(args)
    }
}

struct ParserState<'a> {
    chars: core::str::Chars<'a>,
    current_char: Option<char>,
    remaining: usize,
}

impl<'a> ParserState<'a> {
    pub fn next_char(&mut self) -> Option<char> {
        self.current_char = self.chars.next();
        if self.remaining > 0 {
            self.remaining -= 1;
        }
        self.current_char
    }

    pub fn is_done(&self) -> bool {
        self.current_char.is_none()
    }

    pub fn has_more_chars(&self) -> bool {
        self.remaining > 0
    }

    pub fn remaining(&self) -> usize {
        self.remaining
    }
}

impl<'a> PartialEq<char> for ParserState<'a> {
    fn eq(&self, other: &char) -> bool {
        self.current_char == Some(*other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_support_single_word() {
        let args = Parser::new().parse("word").unwrap();
        assert_eq!(args, &["word"]);
    }

    #[test]
    fn should_support_program_path_at_beginning_if_set_to_full_parsing() {
        let args = Parser::full().parse(r"C:\path\to\program.exe").unwrap();
        assert_eq!(args, &[r"C:\path\to\program.exe"]);
    }

    #[test]
    fn should_support_quoted_args() {
        let args = Parser::new().parse(r#""quoted arg""#).unwrap();
        assert_eq!(args, &[r"quoted arg"]);
    }

    #[test]
    fn should_trim_whitespace_at_front() {
        let args = Parser::new().parse(" \targ").unwrap();
        assert_eq!(args, &[r"arg"]);
    }

    #[test]
    fn should_support_multiple_args() {
        let args = Parser::new().parse("one two three").unwrap();
        assert_eq!(args, &["one", "two", "three"]);
    }

    #[test]
    fn should_support_multiple_args_with_quotes() {
        let args = Parser::new().parse(r#"one "two and uh" three"#).unwrap();
        assert_eq!(args, &["one", "two and uh", "three"]);
    }

    #[test]
    fn should_support_quoted_path_at_start_if_set_to_full_parsing() {
        let args = Parser::full()
            .parse(r#""C:\path\to the\program.exe" arg -arg2 --arg3"#)
            .unwrap();
        assert_eq!(
            args,
            &[r"C:\path\to the\program.exe", "arg", "-arg2", "--arg3"]
        );
    }

    #[test]
    fn should_support_escaping_quotes() {
        let args = Parser::new()
            .parse(r#"one \"two\" "three four" five"#)
            .unwrap();
        assert_eq!(args, &["one", r#"\"two\""#, "three four", "five"]);
    }

    #[test]
    fn should_support_escaping_the_escape_character() {
        let args = Parser::new().parse(r"\\\\").unwrap();
        assert_eq!(args, &[r"\\"]);
    }

    #[test]
    fn should_support_escaping_the_escape_character_and_quote() {
        let args = Parser::new().parse(r#"\\\\\" some quote "#).unwrap();
        assert_eq!(args, &[r#"\\"#, "some quote"]);
    }

    #[test]
    fn should_support_empty_quotes_as_an_argument() {
        let args = Parser::new().parse(r#"one "" three"#).unwrap();
        assert_eq!(args, &["one", "", "three"]);
    }

    #[test]
    fn should_support_quotes_within_quotes() {
        let args = Parser::new().parse(r#"one "" three"#).unwrap();
        assert_eq!(args, &[r#"\\"#, "some quote"]);
    }
}
