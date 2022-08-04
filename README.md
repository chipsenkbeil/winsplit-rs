# winsplit

Like [shell-words](https://crates.io/crates/shell-words), but for Windows that
somewhat mirrors
[CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw).

Specifically, this library follows the 2008 parsing rules for VC++ 9.9
(msvcr90.dll) that was released with Visual Studio 2008. See [C/C++ parameter
parsing rules](https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN)
for more details.

<figure>
<img src="https://user-images.githubusercontent.com/2481802/182859707-008040c5-39eb-4e2a-949a-89911fa5a973.png" alt="Trulli" style="width:100%">
<figcaption align = "center"><b>Parsing Rules for VC++ 2008</b></figcaption>
</figure>

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
