# SprenGUI

Rust Native-ish Sans-IO GUI Toolkit.  Targetting Linux, Windows, Mac OS, Web
Assembly (in browser and Daku), Android, iOS, Redox, Fuchsia, BSD, with a
fallback for other OSes / Game consoles, and possibly supporting other targets
if there's enough interest.

## What Does Native-Ish Mean?

When people say native GUI toolkit, they mean the widgets are used from the GUI
libraries packaged with the operating system.  Unfortunately, there's not always
a GUI packaged with an OS - On Linux, it's packaged with the desktop environment
(not to mention when you're not running an OS).  Additionally, someone who's
learned a program who tries to use it on another operating system may not feel
at home in your program if the widgets all look completely different - which is
an issue.  But, overall, the application should at least try to "blend-in" to
avoid standing out and making users feel like the application doesn't belong on
their system.  Also, the GUI should have full access to the accessibility and
internationalization features built-in to the system.  Like nearly everything in
Computer Science, the "best" solution is a compromise.

### The Compromise

Basically, the window header will use the desktop environment's style to render
the window header (Window Title, Close/Minimize/Maximize Icon) and use the
user's theme, and pixel scaling throughout the whole window if enabled.  The
actual widgets specific to the application will use the same code accross
platforms.  The style of the widgets shall attempt to blend in with all
operating systems (there's a somewhat general consensus on what widgets should
look like across major operating systems - rounded corners, a comfortable amount
of padding an margin, slight gradient, etc.).  There will also be options for
making your own widgets.

## Usable With Any Backend

Sprengui's graphics are communicated as vector graphics (either using the DOM
for web, or an emulation with a [`pix`](https://crates.io/crates/pix) `Raster`
for other platforms).  You also have to pass input in to Sprengui.  This is done
with the [`human`](https://crates.io/crates/human) crate's `Input` enum.

**TODO**: Link to example

## Naming

Sprengui is named after the German adjective for explosive used in the names of
many German explosives: `Spreng`, combined with `GUI`.  This is due to it's
initial inspiration from [semtext](https://github.com/DougLau/semtext), which is
a TUI Toolkit named after Semtex (which is an explosive).

## MSRV

The current MSRV is Rust 1.85.

Any future MSRV updates will follow the [Ardaku MSRV guidelines].

## License

Copyright © 2020-2025 The SprenGUI Contributors.

Licensed under any of
 - Apache License, Version 2.0, ([LICENSE\_APACHE] or
   <https://www.apache.org/licenses/LICENSE-2.0>)
 - Boost Software License, Version 1.0, ([LICENSE\_BOOST] or
   <https://www.boost.org/LICENSE_1_0.txt>)
 - MIT License, ([LICENSE\_MIT] or <https://mit-license.org/>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as described above, without any additional terms or conditions.

## Help

If you want help using or contributing to this library, feel free to send me an
email at <aldaronlau@gmail.com>.

[Ardaku MSRV guidelines]: https://github.com/ardaku/.github/blob/v1/profile/MSRV.md
[LICENSE\_APACHE]: https://github.com/ardaku/sprengui/blob/v0/LICENSE_APACHE
[LICENSE\_BOOST]: https://github.com/ardaku/sprengui/blob/v0/LICENSE_BOOST
[LICENSE\_MIT]: https://github.com/ardaku/sprengui/blob/v0/LICENSE_MIT
