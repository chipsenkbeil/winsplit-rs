# winsplit

[![Crates.io][crates_img]][crates_lnk]
[![Docs][docs_img]][docs_lnk]
[![CI][ci_img]][ci_lnk]

[ci_img]: https://github.com/chipsenkbeil/winsplit-rs/actions/workflows/ci.yml/badge.svg
[ci_lnk]: https://github.com/chipsenkbeil/winsplit-rs/actions/workflows/ci.yml

[crates_img]: https://img.shields.io/crates/v/winsplit.svg
[crates_lnk]: https://crates.io/crates/winsplit

[docs_img]: https://docs.rs/winsplit/badge.svg
[docs_lnk]: https://docs.rs/winsplit

Like [shell-words](https://crates.io/crates/shell-words), but for Windows that
somewhat mirrors
[CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw),
following VC++ 2008 parsing rules.

Written purely in Rust, so runs on any operating system! Windows is _not_ a
requirement!

Minimum tested Rust version is `1.56.1`, but this may compile and work on
earlier versions!

## Installation

```toml
[Dependencies]
winsplit = "0.2"
```

If you want to use this without `std` library, this library can be compiled for
use with `alloc` by disabling the `std` feature:

```toml
[Dependencies]
winsplit = { version = "0.2", default-features = false }
```

## Splitting

When using [CreateProcessA][CreateProcessA] and similar functions, you can use
the `vc_2008::split` function to parse and divide a command line string into
individual arguments.

```rust
let args = winsplit::vc_2008::split(
    r#"C:\ProgramFiles\Example\example.exe --key "some value" arg1 arg2"#
);
assert_eq!(
    args, 
    &[
        r"C:\ProgramFiles\Example\example.exe",
        "--key",
        "some value",
        "arg1",
        "arg2"
    ]
);
```

If you are using `cmd.exe`, you can use the `cmd_exe::split` function to parse
a command line string according to its specific rules:

```rust
let quoted = winsplit::cmd_exe::split(r#""C:\Program Files\App" /C "dir /s""#);
assert_eq!(
    args, 
    &[
        "C:\\Program Files\\App", 
        "/C", 
        "dir /s",
    ]
);
```

If you are using `powershell`, you can use the `powershell::split` function to
parse a command line string according to its specific rules:

```rust
let args = winsplit::powershell::split(r#""C:\Program Files\App" -Arg "dir /s""#);
assert_eq!(
    args, 
    &[
        "C:\\Program Files\\App", 
        "-Arg", 
        "dir /s",
    ]
);
```

[CreateProcessA]: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessa?redirectedfrom=MSDN

### Parsing Rules for VC 2008

This library follows the 2008 parsing rules for VC++ 9.9 (msvcr90.dll) that was
released with Visual Studio 2008. See [C/C++ parameter parsing
rules](https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN) for
more details.

You can also check out the mirror of the rules and examples at the [wiki
documentation
page](https://github.com/chipsenkbeil/winsplit-rs/wiki/Argument-Parsing-Process-w--Examples)
for this repository.

### Special Thanks

Goes to [David Deley](https://daviddeley.com/index.php) for documenting the
complexities of the Windows parameter parsing logic and providing numerous
examples found at
[https://daviddeley.com/autohotkey/parameters/parameters.htm](https://daviddeley.com/autohotkey/parameters/parameters.htm).

Additional thanks to [Simon Sheppard](https://ss64.com/) for documenting the
complexities of cmd.exe argument escaping and providing examples found at
[https://ss64.com/nt/syntax-esc.html](https://ss64.com/nt/syntax-esc.html).

## Quoting

This library also provides the ability to take a string and properly escape and
quote it for `cmd.exe` and `powershell`:

```rust
let quoted = winsplit::cmd_exe::quote(r#"hello \"world\", "good day""#);
assert_eq!(quoted, r#""hello \\""#);
```

```rust
let quoted = winsplit::powershell::quote(r#"hello 'world', "good day""#);
assert_eq!(quoted, r#"'hello ''world'', "good day"'"#);
```

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
