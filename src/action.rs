// Sprengui
// Copyright © 2020-2021 Jeron Aldaron Lau.
//
// Licensed under any of:
//  - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
//  - MIT License (https://mit-license.org/)
//  - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your choosing (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).

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