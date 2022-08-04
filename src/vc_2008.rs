//! For this parser, we are following the updated 2008 rules, which are somewhat simpler than the
//! pre-2008 parsing rules. Given that it is 2022, any modern software that would need to take
//! advantage of Windows parameter parsing should be expected to abide by the modern rules given
//! that Windows Vista was released in 2006.
//!
//! ### Parsing Examples
//! Command-Line | argv\[1\] | Comment
//! -- | -- | --
//! CallMeIshmael | CallMeIshmael | a plain parameter can contain any characters except {space} {tab}  \\  "
//! "Call Me Ishmael" | Call Me Ishmael | spaces enclosed in a double quoted part
//! Cal"l Me I"shmael | Call Me Ishmael | a double quoted part can be anywhere within a parameter
//! CallMe\\"Ishmael | CallMe"Ishmael | \\" → "
//! "CallMe\\"Ishmael" | CallMe"Ishmael | \\" → "  (whether or not in a double quoted part)
//! "Call Me Ishmael\\\\" | Call Me Ishmael\\ | \\\\" → \\ + " (which may begin or end a double quoted part)
//! "CallMe\\\\\\"Ishmael"  | CallMe\\"Ishmael | \\\\\\" → \\"     (\\\\ → \\)  (\\" → ")
//! a\\\\\\b | a\\\\\\b | backslashes not followed immediately by a double quotation mark are interpreted literally
//! "a\\\\\\b" | a\\\\\\b | whether or not the backslashes are in a double quoted part
//!
//! #### Command Tasks
//!
//! Command-Line | argv\[1\] | Comment
//! -- | -- | --
//! "\\"Call Me Ishmael\\"" | "Call Me Ishmael" | the parameter includes double quotes
//! "C:\\TEST A\\\\" | C:\\TEST A\\ | the parameter includes a trailing slash
//! "\\"C:\\TEST A\\\\\\"" | "C:\\TEST A\\" | the parameter includes double quotes and a trailing slash
//!
//! #### Examples Explained
//!
//! Command-Line Input | argv\[1\] | argv\[2\] | argv\[3\] | Comment
//! -- | -- | -- | -- | --
//! "a b c"  d  e | a b c | d | e | spaces enclosed in double quotes
//! "ab\\"c"  "\\\\"  d | ab"c | \\ | d | \\" → "\\\\" → \\ + begin or end a double quoted part
//! a\\\\\\b d"e f"g h | a\\\\\\b | de fg | h | backslashes not followed immediately by a double quotation mark are interpreted literally parameters are separated by spaces or tabs a double quoted part can be anywhere within a parameter the space enclosed in double quotation marks is not a delimiter
//! a\\\\\\"b c d | a\\"b | c | d | 2n+1 backslashes before " → n backslashes + a literal "
//! a\\\\\\\\"b c" d e | a\\\\b c | d | e | 2n backslashes followed by a " produce n backslashes + start/end double quoted part. parameters are separated by spaces or tabs a double quoted part can be anywhere within a parameter the space enclosed in double quotation marks is not a delimiter
//!
//! #### Double Double Quote Examples
//!
//! Command-Line Input | argv\[1\] | argv\[2\] | argv\[3\] | argv\[4\] | argv\[5\] | Comment
//! -- | -- | -- | -- | -- | -- | --
//! "a b c"" | a b c" |   |   |   |   | " Begin double quoted part."" while in a double quoted part → accept 2nd " literally, double quoted part continues
//! """CallMeIshmael"""  b  c | "CallMeIshmael" | b | c |   |   | " Begin double quoted part."" while in a double quoted part → accept 2nd " literally, double quoted part continues" not followed by another " (i.e. not "") while in a double quoted part → ends the double quoted partParameters are delimited by spaces or tabs.
//! """Call Me Ishmael""" | "Call Me Ishmael"|   |   |   |   | " Begin double quoted part."" while in a double quoted part → accept 2nd " literally, double quoted part continues" not followed by another " (i.e. not "") while in a double quoted part → ends the double quoted part
//! """"Call Me Ishmael"" b c | "Call | Me | Ishmael | b | c | " Begin double quoted part."" while in a double quoted part → accept 2nd " literally, double quoted part continues" not followed by another " (i.e. not "") in a double quoted part → ends the double quoted partParameters are delimited by spaces or tabs.(note "" outside of double quoted block begins and then immediately ends a double quoted part.)
//!
//!
//!
//! #### Triple Double Quotes
//!
//! ```text
//!                                      ..."""Call Me Ishmael"""...
//!                                         ↑↑↑               ↑↑↑↑
//! quote #1: Begin double quoted part──────┘├┘               ├┘├┘
//! quotes #2 & 3: Skip 1st " take 2nd " ────┘                │ │
//!                                                           │ │
//! quotes 4 & 5: Skip 1st " take 2nd " ──────────────────────┘ │
//! quote #6: End double quoted part────────────────────────────┘
//! ```
//!
//! ```text
//!  >ShowParams.exe """Call Me Ishmael"""
//!  param 1 = "Call Me Ishmael"
//! ```
//!
//! an alternative method is
//!
//! ```text
//!                    ┌───────────────┐
//!  >ShowParams.exe \""Call Me Ishmael"\"
//!  param 1 = "Call Me Ishmael"
//! ```
//!
//! or
//!
//! ```text
//!                  ┌───────────────────┐
//!  >ShowParams.exe "\"Call Me Ishmael\""
//!  param 1 = "Call Me Ishmael"
//! ```
//!
//! #### Quadruple Double Quotes
//!
//! ```text
//!                                      ...""""Call me Ishmael""""...
//!                                         ↑↑↑↑↑              ↑↑↑↑↑
//! quote #1: Begin double quoted part──────┘├┘├┘              │├┘││
//! quotes #2 & 3: Skip 1st " take 2nd " ────┘ │               ││ ││
//! quote #4: End double quoted part───────────┘               ││ ││
//!                                                            ││ ││
//! quote #5: Begin double quoted part─────────────────────────┘│ ││
//! quotes #6 & 7: Skip 1st " take 2nd " ───────────────────────┘ ││
//! quote #8: End double quoted part──────────────────────────────┘│
//!           Assuming this isn't another " ───────────────────────┘
//! ```
//!
//! ```text
//! >ShowParams.exe """"Call Me Ishmael""""
//!  param 1 = "Call
//!  param 2 = Me
//!  param 3 = Ishmael"
//! ```
//!
//! an alternative method is
//!
//! ```text
//!  >ShowParams.exe \"Call Me Ishmael\"
//!  param 1 = "Call
//!  param 2 = Me
//!  param 3 = Ishmael"
//! ```
//!
//! ### The Rules
//!
//! This is a mirror from David Deley's website to ensure that we have a historical copy in case the website disappears.
//!
//! ![parsingrules](https://user-images.githubusercontent.com/2481802/182859707-008040c5-39eb-4e2a-949a-89911fa5a973.png)
use crate::lib;

/// Parses a command line string into arguments using the VC++ 2008 rules
pub fn parse(s: &str) -> lib::Vec<lib::String> {
    let mut args = lib::Vec::new();
    let mut arg = lib::String::new();
    let mut backslash_cnt = 0;
    let mut in_quote = false;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        // Check the next character to see if it is a quote
        let is_quote_next = chars.peek() == Some(&'"');

        // True if we have an odd number of backslashes
        let even_backslash_cnt = backslash_cnt % 2 == 0;

        match c {
            // Backslash should just increase the count without immediately adding the char
            '\\' => {
                backslash_cnt += 1;
                continue;
            }

            // Quote with even number of backslashes and already within a quote and next
            // character is also a quote
            '"' if even_backslash_cnt && in_quote && is_quote_next => {
                // Move to second quote (essentially skip it since both are ")
                let _ = chars.next();

                // Set backslash cnt to N/2 so we add N/2
                backslash_cnt /= 2;
            }

            // Quote with even number of backslashes and already within a quote
            '"' if even_backslash_cnt && in_quote => {
                // Flag that we are no longer in a quote
                in_quote = false;

                // Set backslash cnt to N/2 so we add N/2
                backslash_cnt /= 2;
            }

            // Quote with even number of backslashes, but not within a quote
            '"' if even_backslash_cnt => {
                // Flag that we are now in a quote
                in_quote = true;

                // Set backslash cnt to N/2 so we add N/2
                backslash_cnt /= 2;
            }

            // Quote with even number of backslashes or anything else
            _ => {}
        }

        // Add backslashes to arg and reset
        if backslash_cnt > 0 {
            add_n_backslashes(&mut arg, backslash_cnt);
            backslash_cnt = 0;
        }

        // If we are in a quote, then we should consume everything,
        // otherwise once we hit whitespace we want to finish the arg
        if !in_quote && is_whitespace_or_null(c) {
            args.push(arg);
            arg = lib::String::new();
        } else {
            arg.push(c);
        }
    }

    if !arg.is_empty() {
        args.push(arg);
    }

    args
}

#[inline]
fn add_n_backslashes(s: &mut lib::String, n: usize) {
    for _ in 0..n {
        s.push('\\');
    }
}

#[inline]
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\r' || c == '\n'
}

#[inline]
fn is_whitespace_or_null(c: char) -> bool {
    is_whitespace(c) || c == '\0'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_support_single_word() {
        let args = parse("word");
        assert_eq!(args, &["word"]);
    }

    #[test]
    fn should_support_program_path_at_beginning() {
        let args = parse(r"C:\path\to\program.exe");
        assert_eq!(args, &[r"C:\path\to\program.exe"]);
    }

    #[test]
    fn should_support_quoted_path_at_beginning() {
        let args = parse(r#""C:\path\to the\program.exe" arg -arg2 --arg3"#);
        assert_eq!(
            args,
            &[r"C:\path\to the\program.exe", "arg", "-arg2", "--arg3"]
        );
    }

    #[test]
    fn should_support_quoted_args() {
        let args = parse(r#""quoted arg""#);
        assert_eq!(args, &[r"quoted arg"]);
    }

    #[test]
    fn should_trim_whitespace_at_front() {
        let args = parse(" \targ");
        assert_eq!(args, &[r"arg"]);
    }

    #[test]
    fn should_support_multiple_args() {
        let args = parse("one two three");
        assert_eq!(args, &["one", "two", "three"]);
    }

    #[test]
    fn should_support_multiple_args_with_quotes() {
        let args = parse(r#"one "two and uh" three"#);
        assert_eq!(args, &["one", "two and uh", "three"]);
    }

    #[test]
    fn should_support_escaping_quotes() {
        let args = parse(r#"one \"two\" "three four" five"#);
        assert_eq!(args, &["one", r#"\"two\""#, "three four", "five"]);
    }

    #[test]
    fn should_support_escaping_the_escape_character() {
        let args = parse(r"\\\\");
        assert_eq!(args, &[r"\\"]);
    }

    #[test]
    fn should_support_escaping_the_escape_character_and_quote() {
        let args = parse(r#"\\\\\" some quote "#);
        assert_eq!(args, &[r#"\\"#, "some quote"]);
    }

    #[test]
    fn should_support_empty_quotes_as_an_argument() {
        let args = parse(r#"one "" three"#);
        assert_eq!(args, &["one", "", "three"]);
    }

    #[test]
    fn should_support_quotes_within_quotes() {
        let args = parse(r#"one "" three"#);
        assert_eq!(args, &[r#"\\"#, "some quote"]);
    }

    // Extra tests from https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN
    mod extra_from_david_deley {
        use super::*;

        #[test]
        fn examples() {
            // Single word is okay
            assert_eq!(parse("CallMeIshmael"), &["CallMeIshmael"]);

            // Quotes can be used to include whitespace in parameter
            assert_eq!(parse(r#""Call Me Ishmael""#), &["Call Me Ishmael"]);

            // Quotes can be anywhere in parameter
            assert_eq!(parse(r#"Cal"l Me I"shmael"#), &["Call Me Ishmael"]);

            // Escaped quote yields just the quote
            assert_eq!(parse(r#"CallMe\"Ishmael"#), &[r#"CallMe"Ishmael"#]);

            // Escaped quote yields just the quote even within a quote
            assert_eq!(parse(r#""CallMe\"Ishmael""#), &[r#"CallMe"Ishmael"#]);

            // Multiple backslash get converted
            //
            // \\\" -> \"
            // (\\ -> \) (\" -> ")
            assert_eq!(parse(r#""CallMe\\\"Ishmael""#), &[r#"CallMe\"Ishmael"#]);

            // Backslashes not followed immediately by a double quotation mark are interpreted
            // literally
            assert_eq!(parse(r"a\\\b"), &[r"a\\\b"]);

            // Backslashes not followed immediately by a double quotation mark are interpreted
            // literally even within quotes
            assert_eq!(parse(r#""a\\\b""#), &[r"a\\\b"]);
        }

        #[test]
        fn common_tasks() {
            // Parameter includes double quotes
            assert_eq!(parse(r#""\"Call Me Ishmael\"""#), &[r#""Call Me Ishmael""#]);

            // Parameter includes trailing slash
            assert_eq!(parse(r#""C:\TEST A\\""#), &[r"C:\TEST A\"]);

            // Parameter includes double quotes and trailing slash
            assert_eq!(parse(r#""\"C:\TEST A\\\"""#), &[r#""C:\TEST A\""#]);
        }

        #[test]
        fn explained_examples() {
            // Spaces enclosed in double quotes
            assert_eq!(parse(r#""a b c"  d  e"#), &["a b c", "d", "e"]);

            // Some escaped quotes
            assert_eq!(parse(r#""ab\"c"  "\\"  d"#), &[r#"ab"c"#, r"\", "d"]);

            // Backslashes not followed immediately by a double quotation mark are interpreted
            // literally
            assert_eq!(parse(r#"a\\\b d"e f"g h"#), &[r"a\\\b", "de fg", "h"]);

            // 2n+1 backslashes before " → n backslashes + a literal "
            assert_eq!(parse(r#"a\\\"b c d"#), &[r#"a\"b"#, "c", "d"]);

            // 2n backslashes followed by a " produce n backslashes + start/end double quoted part
            //
            // the space enclosed in double quotation marks is not a delimiter
            assert_eq!(parse(r#"a\\\\"b c" d e"#), &[r"a\\b c", "d", "e"]);
        }

        #[test]
        fn double_double_quote_examples() {
            assert_eq!(parse(r#""a b c"""#), &[r#"a b c""#]);
            assert_eq!(
                parse(r#""""CallMeIshmael"""  b  c"#),
                &[r#""Call Me Ishmael""#, "b", "c"]
            );
            assert_eq!(parse(r#""""CallMeIshmael""""#), &[r#""Call Me Ishmael""#]);
            assert_eq!(
                parse(r#"""""Call Me Ishmael"" b c"#),
                &[r#""Call"#, "Me", "Ishmael", "b", "c"]
            );
        }

        #[test]
        fn triple_double_quote_examples() {
            assert_eq!(parse(r#""""Call Me Ishmael""""#), &[r#""Call Me Ishmael""#]);

            // Same as above
            assert_eq!(parse(r#"\""Call Me Ishmael"\""#), &[r#""Call Me Ishmael""#]);

            // Same as above
            assert_eq!(parse(r#""\"Call Me Ishmael\"""#), &[r#""Call Me Ishmael""#]);
        }

        #[test]
        fn quadruple_double_quote_examples() {
            assert_eq!(
                parse(r#"""""Call Me Ishmael"""""#),
                &["\"Call", "Me", "Ishmael\""]
            );

            // Same as above
            assert_eq!(
                parse(r#"\"Call Me Ishmael\""#),
                &["\"Call", "Me", "Ishmael\""]
            );
        }
    }
}
