# Diagrams: frappe-desk-ui

The diagram below shows the reactivity loop of the Dioxus client-side application, displaying how layout, router, user events, and real-time SSE stream events mutate global signal state:

```mermaid
graph TD
    subgraph desk-app [Desk Shell & Routing]
        Init[App Launch] --> Context[DeskState Signal Context]
        Context --> Layout[App Shell Layout]
        Layout --> Sidebar[Sidebar Navigation]
        Layout --> Navbar[Navbar User & Tenant Select]
        Layout --> Router[Dioxus Client-Side Router]
    end

    subgraph desk-components [Dynamic Renderer & SSE]
        Router -->|Render View| Form[Form Interpreter Component]
        Router -->|Render List| ListView[ListView Component]
        Form -->|Dynamic Rows| Fields[Link / Date / Currency Fields]
        SSE[SSE Stream Listener] -->|Event Trigger| SignalMutate[Signal Context Value Mutation]
        SignalMutate -->|Auto-Rerender| Form
        SignalMutate -->|Auto-Rerender| ListView
    end
```
