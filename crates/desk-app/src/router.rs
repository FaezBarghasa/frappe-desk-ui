use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/workspace/:name")]
    Workspace { name: String },
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-2xl font-bold mb-4", "Welcome to Caffeine-Rust ERP Desk" }
            p { "Select a workspace from the sidebar to get started." }
        }
    }
}

#[component]
pub fn Workspace(name: String) -> Element {
    rsx! {
        div { class: "p-6",
            h1 { class: "text-2xl font-bold mb-4", "Workspace: {name}" }
            p { "This is the workspace view for {name}." }
        }
    }
}
