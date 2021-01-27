use proc_macro::TokenStream;

#[proc_macro]
pub fn load_plugins(_: TokenStream) -> TokenStream {
    let s = std::fs::read_to_string("plugins.txt").expect("Unable to read `plugins.txt`");
    let mut output = String::new();
    for plugin in s.lines() {
        if plugin.trim().is_empty() {
            println!("empty");
            continue;
        }
        output += &format!("kazyol_lib::tracking::name(\"{}\".to_string());\n", plugin);
        output += &format!("let p = {}::Plugin::init(); if let Ok(plugin) = p {{ plugins.push(Box::new(plugin)) }} else {{ println!(\"Couldn't load {{}}: {{}}\", \"{}\", p.err().unwrap()); return }}\n", plugin, plugin);
        output += "kazyol_lib::tracking::pop();";
        output += &format!("PluginManager::set_enabled::<{}::Plugin>();", plugin);
    }
    output.parse().unwrap()
}
