use dioxus::prelude::*;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct ListRow {
    pub name: String,
    pub fields: std::collections::HashMap<String, Value>,
}

#[component]
pub fn ListView(
    doctype: String,
    columns: Vec<String>,
    rows: Signal<Vec<ListRow>>,
    on_row_click: EventHandler<String>,
    on_refresh: EventHandler<()>,
) -> Element {
    let mut selected_rows = use_signal(|| std::collections::HashSet::<String>::new());
    let limit = use_signal(|| 10u64);
    let mut offset = use_signal(|| 0u64);

    rsx! {
        div {
            style: "background: #191b28; border: 1px solid #282a36; border-radius: 12px; padding: 24px; display: flex; flex-direction: column; gap: 20px; width: 100%; max-width: 900px; box-shadow: 0 8px 32px rgba(0,0,0,0.4);",
            
            // Header
            div { style: "display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #282a36; padding-bottom: 16px;",
                div {
                    h3 { style: "font-size: 20px; font-weight: 700; color: #f8f8f2; margin: 0;", "{doctype} List" }
                    p { style: "font-size: 12px; color: #6272a4; margin: 4px 0 0 0;", "Manage and view {doctype} documents." }
                }
                div { style: "display: flex; gap: 10px;",
                    button {
                        style: "background: #44475a; color: #f8f8f2; border: 1px solid #6272a4; padding: 8px 16px; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer; transition: background 0.2s;",
                        onclick: move |_| on_refresh.call(()),
                        "Refresh"
                    }
                    if !selected_rows().is_empty() {
                        button {
                            style: "background: #ff5555; color: #f8f8f2; border: none; padding: 8px 16px; border-radius: 6px; font-size: 13px; font-weight: 600; cursor: pointer; transition: opacity 0.2s;",
                            onclick: move |_| {
                                log::info!("Bulk delete requested for: {:?}", selected_rows());
                                selected_rows.write().clear();
                            },
                            "Delete ({selected_rows().len()})"
                        }
                    }
                }
            }

            // Table Wrapper
            div { style: "overflow-x: auto; border: 1px solid #282a36; border-radius: 8px; background: #1e1f29;",
                table { style: "width: 100%; border-collapse: collapse; text-align: left;",
                    thead {
                        tr { style: "border-bottom: 2px solid #282a36; background: #282a36;",
                            th { style: "padding: 12px 16px; width: 40px; text-align: center;",
                                input {
                                    r#type: "checkbox",
                                    checked: selected_rows().len() == rows().len() && !rows().is_empty(),
                                    onchange: move |evt| {
                                        let mut selected = selected_rows.write();
                                        if evt.value() == "true" {
                                            for r in rows().iter() {
                                                selected.insert(r.name.clone());
                                            }
                                        } else {
                                            selected.clear();
                                        }
                                    }
                                }
                            }
                            for col in columns.iter() {
                                th { style: "padding: 12px 16px; font-size: 12px; color: #8f93a2; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;",
                                    "{col}"
                                }
                            }
                        }
                    }
                    tbody {
                        if rows().is_empty() {
                            tr {
                                td {
                                    colspan: columns.len() + 1,
                                    style: "padding: 32px; text-align: center; color: #6272a4; font-size: 14px;",
                                    "No documents found."
                                }
                            }
                        } else {
                            for row in rows().iter() {
                                {
                                    let row_name_for_change = row.name.clone();
                                    let row_name_for_click = row.name.clone();
                                    let is_checked = selected_rows().contains(&row.name);
                                    rsx! {
                                        tr {
                                            style: "border-bottom: 1px solid #282a36; cursor: pointer; transition: background 0.1s; hover:background: #282a36;",
                                            td { style: "padding: 12px 16px; text-align: center;",
                                                input {
                                                    r#type: "checkbox",
                                                    checked: is_checked,
                                                    onchange: move |evt| {
                                                        let mut selected = selected_rows.write();
                                                        if evt.value() == "true" {
                                                            selected.insert(row_name_for_change.clone());
                                                        } else {
                                                            selected.remove(&row_name_for_change);
                                                        }
                                                    }
                                                }
                                            }
                                            for col in columns.iter() {
                                                {
                                                    let val = row.fields.get(col).cloned().unwrap_or(Value::Null);
                                                    let val_str = match val {
                                                        Value::String(s) => s,
                                                        Value::Number(n) => n.to_string(),
                                                        Value::Bool(b) => b.to_string(),
                                                        Value::Null => "".to_string(),
                                                        _ => val.to_string(),
                                                    };
                                                    let row_name_inner = row_name_for_click.clone();
                                                    rsx! {
                                                        td {
                                                            style: "padding: 12px 16px; font-size: 13px; color: #f8f8f2;",
                                                            onclick: move |_| on_row_click.call(row_name_inner.clone()),
                                                            "{val_str}"
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

            // Pagination Controls
            div { style: "display: flex; justify-content: space-between; align-items: center; padding-top: 10px;",
                span { style: "font-size: 12px; color: #6272a4;",
                    "Showing {offset()} - {offset() + limit()} documents"
                }
                div { style: "display: flex; gap: 8px;",
                    button {
                        style: "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; padding: 6px 12px; border-radius: 4px; font-size: 12px; cursor: pointer;",
                        disabled: offset() == 0,
                        onclick: move |_| {
                            let current_offset = offset();
                            if current_offset >= limit() {
                                offset.set(current_offset - limit());
                            }
                        },
                        "Previous"
                    }
                    button {
                        style: "background: #282a36; color: #f8f8f2; border: 1px solid #44475a; padding: 6px 12px; border-radius: 4px; font-size: 12px; cursor: pointer;",
                        onclick: move |_| {
                            offset.set(offset() + limit());
                        },
                        "Next"
                    }
                }
            }
        }
    }
}
