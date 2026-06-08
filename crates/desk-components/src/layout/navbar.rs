use crate::state::DeskState;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut state = use_context::<DeskState>();

    let sites = vec!["site1.local", "site2.local", "site3.local"];

    rsx! {
        header {
            style: "height: 64px; background: rgba(15, 17, 26, 0.8); backdrop-filter: blur(12px); border-bottom: 1px solid #1a1b26; display: flex; align-items: center; justify-content: space-between; padding: 0 24px; z-index: 100;",
            // Brand Logo
            div {
                style: "display: flex; align-items: center; gap: 10px;",
                span {
                    style: "font-size: 24px; font-weight: 800; background: linear-gradient(135deg, #ff79c6, #bd93f9); -webkit-background-clip: text; -webkit-text-fill-color: transparent;",
                    "Caffeine"
                }
                span {
                    style: "font-size: 11px; background: #282a36; padding: 2px 6px; border-radius: 4px; color: #6272a4;",
                    "RUST"
                }
            }

            // Tenant Selector & User Info
            div {
                style: "display: flex; align-items: center; gap: 20px;",
                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "font-size: 12px; color: #6272a4;", "Active Site:" }
                    select {
                        style: "background: #1e1f29; border: 1px solid #44475a; color: #f8f8f2; border-radius: 6px; padding: 4px 8px; font-size: 13px; cursor: pointer; outline: none;",
                        value: "{state.active_site}",
                        onchange: move |evt| {
                            state.active_site.set(evt.value());
                        },
                        for site in sites {
                            option { value: "{site}", "{site}" }
                        }
                    }
                }

                // User profile info
                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    div {
                        style: "text-align: right;",
                        div { style: "font-size: 13px; font-weight: 600; color: #f8f8f2;", "{state.current_user}" }
                        div { style: "font-size: 11px; color: #6272a4;", "Administrator" }
                    }
                    div {
                        style: "width: 36px; height: 36px; border-radius: 50%; background: linear-gradient(135deg, #8be9fd, #bd93f9); display: flex; align-items: center; justify-content: center; font-weight: 700; color: #282a36; font-size: 14px;",
                        "A"
                    }
                }
            }
        }
    }
}
