mod auto_debug;
mod auto_deref;
mod enum_from;
mod enum_from_darling;
use proc_macro::TokenStream;

/// 定义EnumFrom宏
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    // 将输入转换为Derive Input
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    println!("input:{:#?}", input);

    // 处理EnumFrom
    enum_from::process_enum_from(input).into()
}

/// 定义EnumFrom宏
#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    // 将输入转换为Derive Input
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    // 处理EnumFrom
    enum_from_darling::process_enum_from(input).into()
}

/// 定义AutoDeref宏
#[proc_macro_derive(AutoDeref, attributes(auto_deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    // 将输入转换为Derive Input
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    // println!("input:{:#?}", input);

    auto_deref::process_auto_deref(input).into()
}

/// 定义AutoDeref宏
#[proc_macro_derive(AutoDebug, attributes(auto_debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    // 将输入转换为Derive Input
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    auto_debug::process_auto_debug(input).into()
}
