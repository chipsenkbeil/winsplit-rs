# winsplit

Like [shell-words](https://crates.io/crates/shell-words), but for Windows that
somewhat mirrors
[CommandLineToArgvW](https://docs.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw).

Specifically, this library follows the 2008 parsing rules for VC++ 9.9
(msvcr90.dll) that was released with Visual Studio 2008. See [C/C++ parameter
parsing rules](https://daviddeley.com/autohotkey/parameters/parameters.htm#WIN)
for more details.

<figure>
<img src="https://images.unsplash.com/photo-1549740425-5e9ed4d8cd34?ixlib=rb-1.2.1&ixid=MXwxMjA3fDB8MHxjb2xsZWN0aW9uLXBhZ2V8MXwzOTU0NTB8fGVufDB8fHw%3D&w=1000&q=80" alt="Trulli" style="width:100%">
<figcaption align = "center"><b>Fig.1 - 4K Mountains Wallpaper</b></figcaption>
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
