# Architecture: frappe-desk-ui

This workspace contains the single-page application and UI component library built using the Dioxus web toolkit. It compiles to `wasm32-unknown-unknown` for web browser execution.

## Architectural Features

1. **Reactivity & Signals**:
   - Built on Dioxus signal primitives (`use_signal`).
   - Global reactive state (`DeskState`) wraps user authentication, active tenant/site identifiers, and active workspace tab configurations.

2. **Form Engine & Interpreter**:
   - The form interpreter dynamically compiles layout inputs from DocType schema metadata definitions.
   - Form fields include asynchronous search autocomplete `LinkField` and high-precision `CurrencyField` inputs formatted with `rust_decimal::Decimal`.
   - **`ListView` Component**: Provides interactive lists for DocTypes with row management and actions.

3. **Real-time Synchronization (SSE)**:
   - Houses a persistent Server-Sent Events listener (`sse.rs`) mapping server events from `/api/v2/stream` directly back to reactive Dioxus signal mutations.
   - Leverages explicitly forgotten event closures to prevent browser garbage collection of active SSE streams.
