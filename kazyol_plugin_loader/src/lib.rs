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
        output += &format!("plugins.push({}::Plugin::init());\n", plugin);
        output += &format!("kazyol_lib::tracking::clear();");
    }
    output.parse().unwrap()
}
