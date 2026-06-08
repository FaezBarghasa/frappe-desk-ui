use crate::form_engine::fields::{CurrencyField, LinkField};
use dioxus::prelude::*;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq)]
pub struct LocalDocField {
    pub fieldname: String,
    pub fieldtype: String, // "Data", "Int", "Link", "Currency"
    pub label: String,
    pub options: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalDocTypeSchema {
    pub name: String,
    pub fields: Vec<LocalDocField>,
}

#[component]
pub fn FormInterpreter(schema: LocalDocTypeSchema) -> Element {
    let mut form_data = use_signal(|| std::collections::HashMap::<String, String>::new());

    rsx! {
        div {
            style: "background: #191b28; border: 1px solid #282a36; border-radius: 12px; padding: 24px; display: flex; flex-direction: column; gap: 20px; max-width: 600px;",
            h3 { style: "font-size: 18px; font-weight: 700; color: #f8f8f2; margin: 0 0 10px 0;",
                "{schema.name} Form"
            }
            div { style: "display: grid; grid-template-columns: 1fr; gap: 16px;",
                for field in schema.fields.iter() {
                    match field.fieldtype.as_str() {
                        "Link" => {
                            let fieldname = field.fieldname.clone();
                            let value = use_signal(|| "".to_string());
                            rsx! {
                                LinkField {
                                    label: field.label.clone(),
                                    options_doctype: field.options.clone().unwrap_or_default(),
                                    value: value,
                                    on_change: move |val: String| {
                                        form_data.write().insert(fieldname.clone(), val);
                                    }
                                }
                            }
                        }
                        "Currency" => {
                            let fieldname = field.fieldname.clone();
                            let value = use_signal(|| Decimal::ZERO);
                            rsx! {
                                CurrencyField {
                                    label: field.label.clone(),
                                    value: value,
                                    on_change: move |val: Decimal| {
                                        form_data.write().insert(fieldname.clone(), val.to_string());
                                    }
                                }
                            }
                        }
                        _ => {
                            let fieldname = field.fieldname.clone();
                            let mut value = use_signal(|| "".to_string());
                            rsx! {
                                div { style: "display: flex; flex-direction: column; gap: 6px; width: 100%;",
                                    label { style: "font-size: 12px; color: #8f93a2; font-weight: 500;", "{field.label}" }
                                    input {
                                        style: "background: #1e1f29; border: 1px solid #44475a; color: #f8f8f2; padding: 10px 12px; border-radius: 6px; font-size: 14px; outline: none;",
                                        value: "{value}",
                                        oninput: move |evt| {
                                            let val = evt.value();
                                            value.set(val.clone());
                                            form_data.write().insert(fieldname.clone(), val);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            button {
                style: "background: linear-gradient(135deg, #ff79c6, #bd93f9); color: #282a36; border: none; padding: 12px 24px; border-radius: 8px; font-weight: 700; font-size: 14px; cursor: pointer; margin-top: 10px; transition: opacity 0.2s;",
                onclick: move |_| {
                    log::info!("Form submitted with data: {:?}", form_data());
                },
                "Submit Document"
            }
        }
    }
}
