use crate::state::DeskState;
use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    let mut state = use_context::<DeskState>();

    let workspaces = vec![
        ("Accounting", "💳"),
        ("Inventory", "📦"),
        ("Manufacturing", "⚙️"),
        ("CRM", "🤝"),
        ("HR", "👥"),
    ];

    rsx! {
        aside {
            style: "width: 260px; background: #11121d; border-right: 1px solid #1a1b26; padding: 24px 16px; display: flex; flex-direction: column; gap: 20px;",
            div {
                style: "font-size: 11px; text-transform: uppercase; letter-spacing: 1.5px; color: #565f89; font-weight: 700; margin-left: 8px;",
                "Workspaces"
            }
            nav {
                style: "display: flex; flex-direction: column; gap: 8px;",
                for (name , icon) in workspaces {
                    button {
                        style: "display: flex; align-items: center; gap: 12px; padding: 10px 12px; border-radius: 8px; border: none; background: transparent; color: #a6accd; text-align: left; font-size: 14px; cursor: pointer; transition: all 0.2s ease;",
                        onclick: move |_| {
                            let mut tabs = state.open_tabs.write();
                            if !tabs.contains(&name.to_string()) {
                                tabs.push(name.to_string());
                            }
                            // Add routing logic here
                            navigator().push(format!("/workspace/{}", name));
                        },
                        span { style: "font-size: 16px;", "{icon}" }
                        span { "{name}" }
                    }
                }
            }
        }
    }
}
