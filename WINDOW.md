# GUI Window Design
Sprengui aims to not feel completely foreign on any platform, but also use the
same widgets across platforms so that the app is recognizable across platforms.

Sprengui's designed as a modern streamlined GUI.  This means it's very
opinionated.  Here's some of the opinions:

 - Nested menus are bad (no file, view etc. menus), the only modern OS that is
   still pushing them is MacOS.  Windows has gone to a "ribbon" style interface
   which is more intuitive and compatible with touchscreens, but I prefer to
   call it a toolbox, as it's more descriptive, and the tabs are compartments.
 - MacOS and GTK have the best-looking widgets, so rounding, margins, and icons
   will be modelled after their's.
 - The ESCAPE key should be virtually equivalent to the BACK button on Android.
 - A high-contrast light color theme that avoids bright colors shall be used by
   default.
 - If you need previous and next icons, they are action buttons, do not use the
   back button.
 - The forward icon may open up a sidebar.  When a sidebar is open it may 

================================================================================

## WASM / MacOS / iOS / Android
This is the default layout for sprengui programs.  The OS is responsible for
displaying the window title, icon and closing "X" icon (or home button).

```
┌────────────┬───┬─────────────────┬──────────────────┬───┐
│ Filename + │ < │ 🔍 Search...    │   Action Buttons │ … │
├────────────│───┴─────────────────┴──────────────────┴───┤
│ Vertical   │ Interactive Canvas or Form                 │
│ Tabs       │                                            │
├──────────────┬─────────────┬────────────────────────┬───┤
│ App Status   │ Compartment │ Additional Compartment │ ↙ │
├──────────────┘             ╰────────────────────────┴───┤
│ Modal Quick Action Buttons                              │
└─────────────────────────────────────────────────────────┘
```

================================================================================

## GNOME / Windows
GNOME and Windows are slightly different, because they use tall window headers
intended to be populated with action buttons and search.

```
┌────────────┬───┬────┬──────────┬────────────────┬───┬───┐
│ Filename + │ < │ 🔍 │ WinTitle │ Action Buttons │ … │ X │
├────────────│───┴────┴──────────┴────────────────┴───┴───┤
│ Vertical   │ Interactive Canvas or Form                 │
│ Tabs       │                                            │
├──────────────┬─────────────┬────────────────────────┬───┤
│ App Status   │ Compartment │ Additional Compartment │ ↙ │
├──────────────┘             ╰────────────────────────┴───┤
│ Modal Quick Action Buttons                              │
└─────────────────────────────────────────────────────────┘
```

================================================================================

## Dive
DiveOS's desktop environment moves action buttons to the top of the screen so
that they are only visible when the window is in focus, and the title is moved
down to the window header.

```
┌───┬─────────────┬───────────┬────┬──────────────────┬───┐
│ ⌘ │ 01-01-00:00 │ OS Status │ 🔍 │   Action Buttons │ … │
├───┴─────────────┴───────────┴────┴──────────────────┴───┤
│ Filename + │ <                             Window Title │
├────────────│────────────────────────────────────────────┤
│ Vertical   │ Interactive Canvas or Form                 │
│ Tabs       │                                            │
├──────────────┬─────────────┬────────────────────────┬───┤
│ App Status   │ Compartment │ Additional Compartment │ ↙ │
├──────────────┘             ╰────────────────────────┴───┤
│ Modal Quick Action Buttons                              │
└─────────────────────────────────────────────────────────┘
```

================================================================================
