use dioxus::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

#[component]
pub fn LinkField(
    label: String,
    options_doctype: String,
    value: Signal<String>,
    on_change: EventHandler<String>,
) -> Element {
    let mut search_query = use_signal(|| "".to_string());
    let mut show_dropdown = use_signal(|| false);

    // Mock asynchronous search queries for targets
    let search_results = use_resource(move || {
        let query = search_query();
        let doctype = options_doctype.clone();
        async move {
            gloo_timers::future::TimeoutFuture::new(150).await;
            if query.is_empty() {
                vec![
                    format!("{} - Default A", doctype),
                    format!("{} - Default B", doctype),
                ]
            } else {
                vec![
                    format!("{} - Matches: {}", doctype, query),
                    format!("{} - Option 2: {}", doctype, query),
                ]
            }
        }
    });

    let list_opt = search_results.read().clone();

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 6px; position: relative; width: 100%;",
            label { style: "font-size: 12px; color: #8f93a2; font-weight: 500;", "{label}" }
            input {
                style: "background: #1e1f29; border: 1px solid #44475a; color: #f8f8f2; padding: 10px 12px; border-radius: 6px; font-size: 14px; outline: none; transition: border 0.2s;",
                value: "{value}",
                oninput: move |evt| {
                    let val = evt.value();
                    value.set(val.clone());
                    search_query.set(val);
                    show_dropdown.set(true);
                },
                onfocus: move |_| show_dropdown.set(true),
            }
            if show_dropdown() {
                div {
                    style: "position: absolute; top: 100%; left: 0; right: 0; background: #282a36; border: 1px solid #44475a; border-radius: 6px; margin-top: 4px; box-shadow: 0 4px 12px rgba(0,0,0,0.5); z-index: 50; max-height: 200px; overflow-y: auto;",
                    if let Some(list) = list_opt {
                        for item in list {
                            {
                                let item_clone = item.clone();
                                rsx! {
                                    div {
                                        style: "padding: 10px 12px; font-size: 13px; color: #f8f8f2; cursor: pointer; border-bottom: 1px solid #1f2029;",
                                        onclick: move |_| {
                                            value.set(item_clone.clone());
                                            on_change.call(item_clone.clone());
                                            show_dropdown.set(false);
                                        },
                                        "{item}"
                                    }
                                }
                            }
                        }
                    } else {
                        div { style: "padding: 10px 12px; font-size: 13px; color: #6272a4;", "Searching..." }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CurrencyField(
    label: String,
    value: Signal<Decimal>,
    on_change: EventHandler<Decimal>,
) -> Element {
    let mut input_text = use_signal(|| value().to_string());
    let mut error_msg = use_signal(|| None::<String>);

    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 6px; width: 100%;",
            label { style: "font-size: 12px; color: #8f93a2; font-weight: 500;", "{label}" }
            div {
                style: "display: flex; position: relative; align-items: center;",
                span {
                    style: "position: absolute; left: 12px; font-size: 14px; color: #6272a4;",
                    "$"
                }
                input {
                    style: "background: #1e1f29; border: 1px solid #44475a; color: #f8f8f2; padding: 10px 12px 10px 24px; border-radius: 6px; font-size: 14px; outline: none; width: 100%;",
                    value: "{input_text}",
                    oninput: move |evt| {
                        let text = evt.value();
                        input_text.set(text.clone());
                        match Decimal::from_str(&text) {
                            Ok(dec) => {
                                error_msg.set(None);
                                value.set(dec);
                                on_change.call(dec);
                            }
                            Err(_) => {
                                error_msg.set(Some("Invalid currency format".to_string()));
                            }
                        }
                    }
                }
            }
            if let Some(err) = error_msg() {
                span { style: "font-size: 11px; color: #ff5555; margin-top: 2px;", "{err}" }
            }
        }
    }
}
