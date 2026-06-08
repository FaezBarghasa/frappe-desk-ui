use desk_components::layout::navbar::Navbar;
use desk_components::layout::sidebar::Sidebar;
use dioxus::prelude::*;

mod router;

use desk_components::state::DeskState;
use router::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 1. Initialize global UI state
    let state = DeskState {
        active_site: use_signal(|| "site1".to_string()),
        current_user: use_signal(|| "Administrator".to_string()),
        open_tabs: use_signal(|| vec!["Accounting".to_string(), "Inventory".to_string()]),
    };
    provide_context(state);

    // 2. Render Premium Desk UI Layout
    rsx! {
        div { class: "min-h-screen flex flex-col bg-[#0f111a] text-[#a6accd] font-sans",
            Navbar {}
            div { class: "flex-1 flex",
                Sidebar {}
                main { class: "flex-1 p-6 bg-[#121520] border-t border-l border-[#1a1e30]",
                    Router::<Route> {}
                }
            }
        }
    }
}
