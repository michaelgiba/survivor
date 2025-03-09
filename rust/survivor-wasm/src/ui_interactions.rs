use std::cell::RefCell;

thread_local! {
    // Store the currently selected button text to display in main region
    static ACTIVE_SELECTION: RefCell<Option<String>> = RefCell::new(None);
}

/// Sets the active selection to display in the main region
pub fn set_active_selection(text: &str) {
    ACTIVE_SELECTION.with(|selection| {
        *selection.borrow_mut() = Some(text.to_string());
    });
}

/// Gets the currently active selection (if any)
pub fn get_active_selection() -> Option<String> {
    ACTIVE_SELECTION.with(|selection| selection.borrow().clone())
}
