use wasm_bindgen::prelude::*;
use web_sys::{EventSource, MessageEvent};
use dioxus::prelude::*;

/// Establishes a connection to the Server-Sent Events endpoint and updates a Dioxus signal on data frames.
///
/// Algorithmic Complexity: $O(1)$ callback handling.
pub fn connect_event_stream(
    url: &str,
    mut target_signal: Signal<String>,
) -> std::result::Result<EventSource, JsValue> {
    let es = EventSource::new(url)?;

    let onmessage_callback = Closure::wrap(Box::new(move |evt: MessageEvent| {
        if let Some(data_str) = evt.data().as_string() {
            // Update Dioxus signal reactively on message frame arrival
            target_signal.set(data_str);
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    es.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    
    // Leak the closure memory intentionally so the browser continues invoking it asynchronously
    onmessage_callback.forget();

    Ok(es)
}
