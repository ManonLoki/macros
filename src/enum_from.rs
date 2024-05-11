use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// 处理EnumFrom
pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    // 获取类型的IDentity
    let ident = input.ident;
    // 加入泛型支持
    let generics = input.generics; // 这里是包含尖括号的

    // 如果是enum，拿到所有的variant 否则不支持
    let variants = match input.data {
        syn::Data::Enum(data_enum) => data_enum.variants,
        _ => panic!("only support enum "),
    };

    let from_impls = variants.iter().map(|variment| {
        match &variment.fields {
            syn::Fields::Unnamed(fields) => {
                // 只处理1个参数的
                if fields.unnamed.len() != 1 {
                    return quote! {};
                }

                let field = fields.unnamed.first().expect("not found field");
                // 获取类型
                let field_type = &field.ty;
                // 获取变量名
                let variant_var = &variment.ident;
                // 生成From代码
                quote! {
                    impl #generics From<#field_type> for #ident #generics {
                        fn from(value: #field_type) -> Self {
                            #ident::#variant_var(value)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });
    // 这里利用声明宏的语法
    quote! {
        #(#from_impls)*
    }
}
