// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

use pasts::{prelude::*, Notifier};

#[cfg(test)]
mod tests;


/// Text formatting
pub enum Format {
    /// Paragraph
    Paragraph,
    /// Code block
    Code,
    /// Quote block begin
    QuoteIn,
    /// Quote block end
    QuoteOut,
    /// H1, usually title of document
    Title,
    /// H2, section 1
    Section,
    /// H3, section 1.1
    Header,
    /// H4, section 1.1.1
    Subheader,
    /// Label for following user entry
    Label,
}

/// Text component
pub enum Text {
    /// Change to new format
    Format(Format),
    /// Strong (bold for written English)
    Strong(bool),
    /// Emphasis (italic for written English)
    Emphasis(bool),
    /// Code (always monospace)
    Code(bool),
    /// Highlight (with a color - except in high-contrast mode, then boxed text)
    Highlight(bool),
    /// Mark (underline, or bracket depending on language)
    Mark(bool),
    /// Render text
    Span(String),
}

/// A field for user entry
pub enum Field {
    Text(String),
    Int(i64),
    Number(f64),
}

/// A widget.
///
/// Customized widgets can be built using a `Canvas` (TODO: Example).
pub enum Widget {
    /// Page text
    Text(Vec<Text>),
    /// Page field
    Field(Field),
    /// Page choices (with number of columns)
    Choices(Vec<Widget>, usize),
    /// Page buttons (with number of columns)
    Buttons(Vec<Widget>, usize),
    /// Page canvas (picture, movie, rendered, etc.)
    Canvas(pix::Raster<pix::rgb::SRgba8>),
}

enum Icon {
    /// Wand icon for effects
    Wand,
    /// Pencil icon
    Pencil,
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// ID for a MenuAction
pub struct MenuAction {
    // what is happening
    action_id: usize,
    // where
    menu_id: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
/// ID for a WindowAction
pub struct WindowAction {
    // what is happening
    action_id: usize,
    // where
    window_id: usize,
}

#[derive(Clone, PartialEq, Eq)]
/// ID for a search action
pub struct SearchAction {
    // what are we searching for?
    query: String,
    // where are we looking?
    db_id: usize,
}

/// ID for button action
pub struct ButtonAction {
    // what is happening?
    action_id: usize,
    // where?
    entry_id: usize,
}

pub struct FieldAction;

pub struct SliderAction;

/// Actions that produce data
pub enum EntryAction {
    Button(ButtonAction),
    Field(FieldAction),
    Slider(SliderAction),
}

/// Actions performed with the tab key
pub enum TabAction {
    SelectTab,
    MoveTab,
    ReverseTab,
}

/// Actions
///
/// Events that are returned back to the user.
pub enum Action {
    /// Interaction with a menu
    Menu(MenuAction),
    /// Interaction with a window
    Window(WindowAction),
    /// Search action
    Search(SearchAction),
    /// Input interactions
    Entry(EntryAction),
    /// Tab functionality
    Tab(TabAction),
}

struct MenuItem {
    icon: Option<Icon>,
    name: String,
    description: String,
    // FIXME: using window crate types
    shortcut: (),
}

/// Heads-up-display menu interface
pub struct Menu {
    id: usize,
    items: Vec<MenuItem>,
}

impl Menu {
    /// Create a menu for this window
    // FIXME: Take menu items as parameters or use builder
    pub fn new(window: &mut Window) -> Self {
        let id = window.menus;
        window.menus += 1;
        let items = Vec::new();
        Self { id, items }
    }
}

/// Graphical window.
pub struct Window {
    /*window::Window, */
    widgets: Vec<Widget>,
    menus: usize,
}

impl Window {
    /// Open a window
    pub fn new(self) -> Window {
        Window {
            widgets: Vec::new(),
            menus: 0,
        }
    }

    /// Create a canvas for the window
    ///
    /// The canvas will be created the same size as the window.
    pub fn canvas(&mut self) {
        todo!("Tried creating a canvas");
    }

    /// Set the window label
    ///
    /// Usually this is set to the name of the file that is currently open.
    pub fn label(&mut self, label: &str) {
        todo!("Tried setting label to {label}");
    }

    /// Close current tab (APP+W)
    pub fn close(&mut self) {
        todo!();
    }

    /// Switch tab (OPT+W), also collapse/expand tab list
    pub fn switch(&mut self) {
        todo!();
    }

    /// Open a new tab using the OS file selector (APP+E)
    pub fn open(&mut self) {
        todo!();
    }

    /// Select tab (OPT+E)
    pub fn select(&mut self) {
        todo!();
    }

    /// Group tab (APP+R)
    pub fn group(&mut self) {
        todo!();
    }

    /// Ungroup tab (OPT+R)
    pub fn ungroup(&mut self) {
        todo!();
    }

    /// Open a new blank tab (APP+T)
    pub fn tab(&mut self) {
        todo!();
    }

    /// Re-open a tab (OPT+T)
    pub fn reopen(&mut self) {
        todo!();
    }

    /// Set the HUD menu for this window.
    pub fn menu(&mut self, _menu: &Menu) {
        todo!()
    }

    /// Searching
    pub fn search(&mut self) {
        todo!()
    }

    /// Headerbar navigation
    pub fn nav(&mut self) {
        todo!()
    }
}

impl Notifier for Window {
    type Event = Action;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Exec<'_>,
    ) -> Poll<Self::Event> {
        Poll::Pending
    }
}

/// A GUI page
pub struct Page {
    widgets: Vec<Widget>,
}

impl Page {
    pub fn new(_window: &mut Window) -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    /// Set the widgets
    pub fn set<const N: usize>(&mut self, widgets: [Widget; N]) {
        self.widgets.clear();
        self.widgets.extend(widgets);
    }

    /// Get the number of widgets in this window
    pub fn len(&self) -> usize {
        self.widgets.len()
    }

    /// Add a widget to the window
    pub fn push(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
}

