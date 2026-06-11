# frappe-desk-ui

Frontend desktop UI and components for the Rust ERPNext rewrite.

## Modules

The workspace is structured as a collection of frontend application crates:

- **`desk-app`**: Application shell, root setup, client-side routing, and global state management (current tenant/site, current user, open tab workspace signals).
- **`desk-components`**: Reusable components and layout shells:
  - **`layout/`**: Encompasses `navbar.rs` (branding, profile dropdowns, tenant/site dropdown switcher) and `sidebar.rs` (workspace selection items).
  - **`form_engine/`**: Comprises `interpreter.rs` (dynamically builds forms from schemas), `fields.rs` (autocomplete `LinkField` widgets, `CurrencyField` inputs), and `list_view.rs` (tabular data views with row management).
  - **`widgets/`**: Dashboard visual modules and dynamic widget builder.
  - **`sse.rs`**: Background server-sent events stream consumer that mutates frontend state reactively when updates arrive from `/api/v2/stream`.

## Getting Started

Building the WASM bundle for the Dioxus app:

```bash
# Requires Dioxus CLI
dioxus build --release
```
