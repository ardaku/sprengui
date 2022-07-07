# Design
Standards for Sprengui App Design.  Words and lines may refer to things other
than words and lines (music: word is measure, line is part; 3D Modelling / SVG:
word is face/polygon, line is object).  These are based on current practices of
most software, with the removal double/tripple-clicking, context menus and
keyboard chords of more than two keys due to being considered bad practice by
Sprengui research.

## Input

### Mouse Buttons
 - Left: Click GUI Buttons, Move Cursor, Grab/Select (Drag), Unselect (Selected)
 - Middle (Opt): Moves Viewport (Drag), Slow Scroll (Drag), Clone (Selected)
 - Right (Ctrl): Add Word To Selection, Add Line (Selected), Visual Lines (Drag)
 - Side (Fn/Esc): High DPI, Next View Mode, Previous Page, Undo
 - Super: Move Window (Drag)
 - Shift+Left: Select From Cursor, Select From Cursor (Drag)
 - Shift+Middle: Quick Scroll Toggle, Quick Scroll (Drag)
 - Shift+Right: Remove Word From Selection, Line (Unselected), Lines (Drag)
 - Shift+Side: Reset DPI, Previous View Mode, Next Page, Redo
 - Shift+Super: Move Window - Cling To Screen Edges (Drag)

### Mouse Wheel
Mouse wheels do not use natural scrolling.

 - Normal: Scroll Default (usually vertical)
 - Shift: Scroll Opposite (usually horizontal)
 - Ctrl (or Shift+Opt): Zoom
 - Opt (or Shift+Ctrl): Rotate
 - Super: Change Workspace
 - Fn: OS Zoom

### Touchscreen / Trackpad
Touchscreens and trackpads use natural scrolling.  Diagonal controls should be
swappable for left-handed people.

 - One: Click GUI Buttons (Tap), Move Cursor (Tap), Move Viewport (Swipe on
   Touchscreen)
 - Two: Select (Tap), Zoom (Pinch /) / Rotate (Pinch \\), Move Viewport (Swipe
   on Trackpad)
 - Three: Select App (Tap), Switch Apps (Swipe Horizontal), Switch Workspace
   (Swipe Vertical), OS Zoom (Pinch)
 - Four: Volume (Swipe Vertical), Brightness (Swipe Horizontal)

========================

### Keyboard
Sprengui apps should be compatible with the condensed 64-key ardaku keyboard for
maximum compatibility.  They should not require the numpad, and treat number
keys on the top the same as numpad keys, as well as return the same as enter.

**Normal Layout:**

![Picture](404.svg)

**Shift Layout:**

![Picture](404.svg)

**Ctrl Shortcuts:**

![Picture](404.svg)

**Opt (Shift+Ctrl) Shortcuts:**

![Picture](404.svg)

**Super Shortcuts:**

![Picture](404.svg)

**Shift+Super Shortcuts:**

![Picture](404.svg)

**Fn/Esc Shortcuts:**

![Picture](404.svg)

**Shift+Fn Shortcuts:**

![Picture](404.svg)
