# Contributing to frappe-desk-ui

1. **Reactive State**: Avoid mutating values outside Dioxus signals. Use `.write()` or `.set()` for updating state dynamically.
2. **WASM Constraints**: Keep bundles slim. Minimize reliance on heavy packages that require local system libraries.
3. **No Placeholders**: Ensure widgets, form fields, and interpret pages are fully production-ready without stubs.
4. **Form Inputs**: Ensure all numeric inputs validate boundaries and utilize precise decimal parsing.
