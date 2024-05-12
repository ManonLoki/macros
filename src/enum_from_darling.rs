use core::panic;

use darling::{
    ast::{self, Data},
    util, FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
pub struct EnumFromDeriveInput {
    /// 标识符
    ident: syn::Ident,
    /// 数据
    data: ast::Data<EnumFromVariant, util::Ignored>,
    // 泛型
    generics: syn::Generics,
}

#[derive(Debug, FromVariant)]
pub struct EnumFromVariant {
    /// 标识符
    ident: syn::Ident,
    /// 字段
    fields: ast::Fields<EnumFromField>,
}

#[derive(Debug, FromField)]
pub struct EnumFromField {
    /// 类型
    ty: syn::Type,
}

/// 处理EnumFrom
pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    let EnumFromDeriveInput {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDeriveInput::from_derive_input(&input).expect("Parse Input Error")
    else {
        panic!("Only support enum")
    };

    // 处理每一个variant
    let from_impls = data.iter().map(|variant| {
        let variant_ident = &variant.ident;

        // 处理不是unname的以及不为1的
        if !variant.fields.is_tuple() || variant.fields.fields.len() != 1 {
            return quote! {};
        }

        // 拿到第一个字段的类型
        let field_type = &variant.fields.fields[0].ty;

        // 生成From代码
        quote! {
            impl #generics From<#field_type> for #ident #generics {
                fn from(value: #field_type) -> Self {
                    #ident::#variant_ident(value)
                }
            }
        }
    });

    quote! {
        #(#from_impls)*
    }
}
