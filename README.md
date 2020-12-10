# Sprengui
Rust Native-ish GUI Toolkit - Designed for use with Cala, but not limited to Cala.  Targetting Linux, Windows, Mac OS, Web Assembly (in browser), Android, iOS, Redox, Fuchsia, BSD, with a fallback for other OSes / Game consoles, and possibly supporting other targets if there's enough interest.

## What Does Native-Ish Mean?
When people say native GUI toolkit, they mean the widgets are used from the GUI libraries packaged with the operating system.
Unfortunately, there's not always a GUI packaged with an OS - On Linux, it's packaged with the desktop environment (not to mention when you're not running an OS).
Additionally, someone who's learned a program who tries to use it on another operating system may not feel at home in your program if the widgets all look completely different - which is an issue.  But, overall, the application should at least try to "blend-in" to avoid standing out and making users feel like the application doesn't belong on their system.  Also, the GUI should have full access to the accessibility features built-in to the system, as well as retreive the user's configured language.  Like everything in Computer Science, the correct solution is a compromise.

### The Compromise
Basically, the window header will use the desktop environment's style to render the window header (Window Title, Close/Minimize/Maximize Icon) and use the user's theme, and pixel scaling throughout the whole window if enabled.  The actual widgets specific to the application will use the same code accross platforms.  The style of the widgets shall attempt to blend in with all operating systems (there's a somewhat general consensus on what widgets should look like across major operating systems - rounded corners, a comfortable amount of padding an margin, slight gradient, etc.).  There will also be options for making your own widgets.

## Usable With Any Backend
Sprengui takes care of rendering your graphical interface to a [`pix`](https://crates.io/crates/pix) `Raster`.  From there, you can write it into a texture, and use whichever crate you please in order to render it to the screen.  You also have to pass input in to Sprengui.  This is done with the [`human`](https://crates.io/crates/human) crate's `Input` enum.  These crates are used natively by the [`cala`](https://crates.io/crates/cala) crate, making integration easy - but as long as you can convert to `human::Input` and from `pix::Raster`, there's no need to be tied to the `cala` crate.  I would suggest conversions be made in a crate titled something like `sprengui-winit`.

### Using with Cala
Cala allows you to use `async` to write your GUI application.  It does not, however require you to depend on a large runtime like `tokio` or `async-std` or even the `futures` crate (although you can if you want).  In fact, the runtime used by Cala is a self-contained crate titled `pasts`.

**TODO**: Link to example

## Naming
Sprengui is named after the German adjective for explosive used in the names of many German explosives: `Spreng`, combined with `GUI`.  This is due to it's similarities and inspiration from [semtext](https://github.com/DougLau/semtext), which is a TUI Toolkit named after Semtex (which is an explosive).
