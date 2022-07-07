# GUI Guidelines
In order to make easy to understand, elegant GUIs one must limit what they can
do.  Sprengui has these limitations on purpose.  Some of these decisions may be
unusual, but the end-user experience is improved because of them.

## Tabs Pane
Tabs should always be vertical and groupable.  This allows for easy
organization and scrollable tabs without reducing visibility of tab identifiers.
Tabs should always be located on the left and auto-collapse when the user is not
interacting with it.

## Scroll View
Depending on the state of the application you may want a scrollable document.
This should take up the majority of the screen (besides the tab pane and the
header menu).  This should always be the pane in the lower right of the screen
so that the mouse can hit the edge to grab the scroll bar.  If the scroll pane
primarily scrolls horizontally timewise that should be the default action of the
scroll wheel.  There should only be one scroll view, and the only other thing
that scrolls is the tabs pane.  Nested scroll views are bad for obvious reasons,
so disallowed.  Natural scrolling should be preferred for touchpads and
touchscreens, but not mouse wheels.

## Preview Pane
Preview panes appear above the scroll view (or may use an additional monitor).
Useful for things like video editing, picture editing, IDEs and 3D modeling.

## Hamburger Menu
The header menu reserves the middle space for dragging the window and displaying
the file name.  The header menu may have either a tabs button or a back button
(they may display the same "<" icon, though).  Followed by the back icon, is a
search icon (which may expand to fill up the entire header menu except for the
back and exit buttons when clicked).  On the right there is a menu icon which
shows a vertical list of additional pages.

## Mode Switcher
Mode switchers are floating action button(s) that are located on the right side
of the screen on top of the scroll bar.  A mouse clicked on the very right edge
of the screen will press a button.  This will change into selecting the scroll
bar if dragged.  It can be assumed that you can have at least 8 mode buttons
without overflowing the screen, so to never overflow, the number maxes out at 8.

## Action Tray
The Action Tray is located at the bottom of the screen.  It resembles the mode
switcher, and works the same way with the scroll bar.  The action tray should
give you buttons for what operations you can do in whatever mode you are in.  It
can be assumed that you can have at least 8 actions buttons without overflowing
the screen, so to never overflow, the number mabxes out at 8.

## Context Menu
The context menu has quick actions that are available in all modes, but may
change depending on where they are openned.  Stuff like undo, redo, select all,
cut, copy, paste belongs in a context menu.  Additionally may have an "object
properties" or similar option.  Context menus are openned with either right
click, menu key (on some keyboards) or two-finger tap.

## Pop-Ups
Pop-ups aren't visually pop-ups in the traditional sense.  They are equivalent
to pages that you can return from.  They can't have tabs because you have to be
able to go back.  You can have an action tray, mode switcher and hamburger menu,
though.

## Document
The document is located inside of the scroll view.  It may have rasters, text,
buttons, etc.
