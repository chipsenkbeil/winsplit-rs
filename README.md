# winsplit

Like [shell-words](https://crates.io/crates/shell-words), but for Windows that
somewhat mirrors
[CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw),
following VC++ 2008 parsing rules.

## Installation

```toml
[Dependencies]
winsplit = "0.1"
```

## Usage

```rust
let args = winsplit::split(
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

## Parsing Rules

This library follows the 2008 parsing rules for VC++ 9.9 (msvcr90.dll) that was
released with Visual Studio 2008. See [C/C++ parameter parsing
rules](https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN) for
more details.

You can also check out the mirror of the rules and examples at the [wiki
documentation
page](https://github.com/chipsenkbeil/winsplit-rs/wiki/Argument-Parsing-Process-w--Examples)
for this repository.

## Special Thanks

Goes to [David Deley](https://daviddeley.com/index.php) for documenting the
complexities of the Windows parameter parsing logic and providing numerous
examples found at
[https://daviddeley.com/autohotkey/parameters/parameters.htm](https://daviddeley.com/autohotkey/parameters/parameters.htm).

## License

This project is licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or
[apache-license][apache-license]) MIT license (LICENSE-MIT or
[mit-license][mit-license]) at your option.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
[mit-license]: http://opensource.org/licenses/MIT
