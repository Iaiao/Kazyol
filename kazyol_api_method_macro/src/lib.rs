use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn api_method(attr: TokenStream, method: TokenStream) -> TokenStream {
    // TODO input validation
    let code = format!("{{
kazyol_lib::tracking::name({}.to_string());
struct __APIMethod;
impl std::ops::Drop for __APIMethod {{
    fn drop(&mut self) {{
        kazyol_lib::tracking::pop();
    }}
}}
let __apimethodvariable = __APIMethod;", attr);
    method.to_string().replacen("{", &code, 1).parse().unwrap()
}
