#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
///!
///! Follows [CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw).
///!
///! CommandLineToArgvW            [SHELL32.@]
///!
///! We must interpret the quotes in the command line to rebuild the argv
///! array correctly:
///! - arguments are separated by spaces or tabs
///! - quotes serve as optional argument delimiters
///!   '"a b"'   -> 'a b'
///! - escaped quotes must be converted back to '"'
///!   '\"'      -> '"'
///! - consecutive backslashes preceding a quote see their number halved with
///!   the remainder escaping the quote:
///!   2n   backslashes + quote -> n backslashes + quote as an argument delimiter
///!   2n+1 backslashes + quote -> n backslashes + literal quote
///! - backslashes that are not followed by a quote are copied literally:
///!   'a\b'     -> 'a\b'
///!   'a\\b'    -> 'a\\b'
///! - in quoted strings, consecutive quotes see their number divided by three
///!   with the remainder modulo 3 deciding whether to close the string or not.
///!   Note that the opening quote must be counted in the consecutive quotes,
///!   that's the (1+) below:
///!   (1+) 3n   quotes -> n quotes
///!   (1+) 3n+1 quotes -> n quotes plus closes the quoted string
///!   (1+) 3n+2 quotes -> n+1 quotes plus closes the quoted string
///! - in unquoted strings, the first quote opens the quoted string and the
///!   remaining consecutive quotes follow the above rule.
///!
///!
///! Licensed under the Apache License, Version 2.0 <LICENSE-APACHE>
///! or the MIT license <LICENSE-MIT>, at your option.

mod lib {
    #[cfg(feature = "std")]
    extern crate core;

    pub use core::fmt;
    pub use core::mem;

    #[cfg(not(feature = "std"))]
    #[macro_use]
    extern crate alloc;

    #[cfg(not(feature = "std"))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(not(feature = "std"))]
    pub use alloc::vec::Vec;
    #[cfg(feature = "std")]
    pub use std::vec::Vec;

    #[cfg(not(feature = "std"))]
    pub use alloc::borrow::Cow;
    #[cfg(feature = "std")]
    pub use std::borrow::Cow;
}

mod parser;
use parser::Parser;
pub use parser::{ParseError, ParseResult};

/// Tokenizes a string of Windows command line arguments, which may contain quotes and escaped quotes.
///
/// See [MSDN docs](http://msdn.microsoft.com/en-us/library/windows/desktop/17w5ykft(v=vs.85).aspx)
/// for [CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw)
/// for information on the quoting rules.
///
/// For handling a full Windows command line including the executable name at the start, see [`split_full`].
pub fn split(s: &str) -> ParseResult {
    Parser::new().parse(s)
}

/// Tokenizes a Windows full command line, including command name at the start.
///
/// This uses the same syntax rules as [`split`] for all but the first token. But
/// the first token is expected to be parsed as the executable file name in the way
/// [CreateProcess](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa)
/// would do it, rather than the way the C library startup code would do it: CreateProcess does not
/// consider that \ is ever an escape character (because " is not a valid filename char, hence
/// there's never a need to escape it to be used literally).
///
/// Parameters are the same as for [`split`]. In particular, if you set MarkEOLs =
/// true, then the first word of every line will be parsed using the special rules for command
/// names, making this function suitable for parsing a file full of commands to execute.
pub fn split_full(s: &str) -> ParseResult {
    Parser::full().parse(s)
}
