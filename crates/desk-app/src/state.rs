use dioxus::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct DeskState {
    pub active_site: Signal<String>,
    pub current_user: Signal<String>,
    pub open_tabs: Signal<Vec<String>>,
}
