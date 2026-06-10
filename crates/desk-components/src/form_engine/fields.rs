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

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TableRow {
    pub id: String,
    pub cells: std::collections::HashMap<String, String>,
}

#[component]
pub fn TableField(
    label: String,
    columns: Vec<String>,
    mut rows: Signal<Vec<TableRow>>,
    on_change: EventHandler<Vec<TableRow>>,
) -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 8px; width: 100%; border: 1px solid #44475a; border-radius: 8px; padding: 12px; background: #1e1f29;",
            div { style: "display: flex; justify-content: space-between; align-items: center;",
                label { style: "font-size: 13px; color: #f8f8f2; font-weight: 600;", "{label}" }
                button {
                    style: "background: #50fa7b; color: #282a36; border: none; padding: 6px 12px; border-radius: 4px; font-size: 11px; font-weight: 700; cursor: pointer;",
                    onclick: move |_| {
                        let mut current_rows = rows.write();
                        let mut cells = std::collections::HashMap::new();
                        for col in &columns {
                            cells.insert(col.clone(), "".to_string());
                        }
                        current_rows.push(TableRow {
                            id: uuid::Uuid::new_v4().to_string(),
                            cells,
                        });
                        on_change.call(current_rows.clone());
                    },
                    "+ Add Row"
                }
            }
            div { style: "overflow-x: auto;",
                table { style: "width: 100%; border-collapse: collapse; text-align: left;",
                    thead {
                        tr { style: "border-bottom: 2px solid #44475a;",
                            for col in columns.iter() {
                                th { style: "padding: 8px; font-size: 11px; color: #8f93a2; text-transform: uppercase;",
                                    "{col}"
                                }
                            }
                            th { style: "padding: 8px; width: 50px;" } // Action column
                        }
                    }
                    tbody {
                        for (idx, row) in rows().iter().enumerate() {
                            {
                                let row_id = row.id.clone();
                                rsx! {
                                    tr { style: "border-bottom: 1px solid #282a36;",
                                        for col in columns.iter() {
                                            {
                                                let col_name = col.clone();
                                                let cell_val = row.cells.get(&col_name).cloned().unwrap_or_default();
                                                let row_id_inner = row_id.clone();
                                                rsx! {
                                                    td { style: "padding: 4px;",
                                                        input {
                                                            style: "background: transparent; border: 1px solid transparent; color: #f8f8f2; padding: 6px; width: 100%; font-size: 13px; outline: none; border-radius: 4px;",
                                                            value: "{cell_val}",
                                                            onfocus: move |evt| {
                                                                // Highlight outline on focus
                                                                let _ = evt;
                                                            },
                                                            oninput: move |evt| {
                                                                let val = evt.value();
                                                                let mut current_rows = rows.write();
                                                                if let Some(r) = current_rows.iter_mut().find(|r| r.id == row_id_inner) {
                                                                    r.cells.insert(col_name.clone(), val);
                                                                }

                                                                on_change.call(current_rows.clone());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        td { style: "padding: 4px; text-align: center;",
                                            button {
                                                style: "background: #ff5555; color: #f8f8f2; border: none; padding: 4px 8px; border-radius: 4px; font-size: 10px; cursor: pointer;",
                                                onclick: move |_| {
                                                    let mut current_rows = rows.write();
                                                    current_rows.remove(idx);
                                                    on_change.call(current_rows.clone());
                                                },
                                                "Delete"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

