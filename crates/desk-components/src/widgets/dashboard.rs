use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct DashboardProps {
    pub title: String,
}

#[component]
pub fn RenderLineChart(data: Vec<f64>, step_x: f64, scale_y: f64) -> Element {
    let points = data
        .iter()
        .enumerate()
        .map(|(i, val)| format!("{},{}", (i as f64) * step_x, val * scale_y))
        .collect::<Vec<_>>()
        .join(" ");
    rsx! {
        svg {
            width: "100%",
            height: "100%",
            view_box: "0 0 100 100",
            polyline {
                fill: "none",
                stroke: "#ff79c6",
                stroke_width: "2",
                points: "{points}",
            }
        }
    }
}

#[component]
pub fn InsightsDashboard(props: DashboardProps) -> Element {
    // Leverage Dioxus Resource signals (`use_resource`) to fetch backend aggregation results asynchronously
    let metrics_resource = use_resource(move || async move {
        gloo_timers::future::TimeoutFuture::new(500).await;
        vec![10.0, 25.0, 15.0, 40.0, 30.0, 60.0, 50.0]
    });

    rsx! {
        div {
            style: "background: #191b28; border: 1px solid #282a36; border-radius: 12px; padding: 24px; display: flex; flex-direction: column; gap: 20px;",
            h3 { style: "font-size: 18px; font-weight: 700; color: #f8f8f2; margin: 0 0 10px 0;",
                "{props.title}"
            }
            div {
                style: "height: 200px; width: 100%; border: 1px solid #44475a; border-radius: 8px; display: flex; align-items: flex-end; padding: 10px;",
                if let Some(data) = metrics_resource.read().clone() {
                    RenderLineChart { data, step_x: 15.0, scale_y: 1.0 }
                } else {
                    span { style: "color: #6272a4;", "Loading insights..." }
                }
            }
        }
    }
}
