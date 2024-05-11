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
