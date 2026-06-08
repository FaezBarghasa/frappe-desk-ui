with open("crates/desk-components/src/layout/sidebar.rs", "r") as f:
    content = f.read()

target = """                        onclick: move |_| {
                            let mut tabs = state.open_tabs.write();
                            if !tabs.contains(&name.to_string()) {
                                tabs.push(name.to_string());
                            }
                        },"""

replacement = """                        onclick: move |_| {
                            let mut tabs = state.open_tabs.write();
                            if !tabs.contains(&name.to_string()) {
                                tabs.push(name.to_string());
                            }
                            // Add routing logic here
                            navigator().push(format!("/workspace/{}", name));
                        },"""

if target in content:
    content = content.replace(target, replacement)
    with open("crates/desk-components/src/layout/sidebar.rs", "w") as f:
        f.write(content)
    print("Replaced successfully")
else:
    print("Target not found")
