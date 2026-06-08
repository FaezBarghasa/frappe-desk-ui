use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct WidgetConfig {
    pub id: String,
    pub widget_type: String,
    pub x: f64,
    pub y: f64,
}

#[component]
pub fn BuilderCanvas() -> Element {
    let mut widgets = use_signal(|| Vec::<WidgetConfig>::new());
    let mut dragged_widget = use_signal(|| None::<String>);

    rsx! {
        div {
            style: "display: flex; gap: 20px; width: 100%; height: 600px;",
            div {
                style: "width: 200px; background: #1e1f29; padding: 16px; border-radius: 8px;",
                h3 { style: "color: #f8f8f2;", "Widgets" }
                div {
                    draggable: true,
                    style: "background: #282a36; color: #f8f8f2; padding: 12px; margin-bottom: 8px; border-radius: 4px; cursor: grab;",
                    ondragstart: move |_| dragged_widget.set(Some("LineChart".to_string())),
                    "Line Chart"
                }
            }
            div {
                style: "flex: 1; background: #191b28; border: 2px dashed #44475a; border-radius: 8px; position: relative; overflow: hidden;",
                ondragover: move |evt| {
                    evt.prevent_default();
                },
                ondrop: move |evt| {
                    evt.prevent_default();
                    if let Some(w_type) = dragged_widget() {
                        let coords = evt.client_coordinates();
                        let len = widgets().len();
                        widgets.write().push(WidgetConfig {
                            id: format!("{}-{}", w_type, len),
                            widget_type: w_type,
                            x: coords.x,
                            y: coords.y,
                        });
                        dragged_widget.set(None);
                    }
                },
                for widget in widgets() {
                    div {
                        style: "position: absolute; left: {widget.x}px; top: {widget.y}px; background: #bd93f9; color: #282a36; padding: 16px; border-radius: 8px;",
                        "{widget.widget_type}"
                    }
                }
            }
        }
    }
}
