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
