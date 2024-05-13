use darling::{
    ast::{self, Data},
    FromDeriveInput, FromField,
};
use quote::quote;
use syn::DeriveInput;

/**
 * 处理AutoDeref
 *  #[derive(Debug,AutoDeref)]
 * #[auto_deref(mutable=false,field="data")
 *   struct XXX<T>{
 *      data:T,
 *      other:()
 *   }
 *
 * impl<T> std::ops::Deref for XXX<T>{
 *   // data的类型
 *  type Target = T;
 *  fn deref(&self) -> &Self::Target {
 *     &self.xxx
 *  }
 * }
 *
 * 需要拿到 generics, ident, fields
 * 外加 attributes 决定是否实现mutable,对应的Deref Field是哪个
 */

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(auto_deref))]
pub(crate) struct AutoDerefDeriveInput {
    /// 标识符
    ident: syn::Ident,
    /// 泛型信息
    generics: syn::Generics,
    /// 数据
    data: ast::Data<(), AutoDerefField>,
    /// 是否可变
    #[darling(default)]
    mutable: bool,
    /// 字段
    #[darling(default)]
    field: Option<syn::Ident>,
}

/// 字段信息
#[derive(Debug, FromField)]
pub(crate) struct AutoDerefField {
    /// 字段类型
    ty: syn::Type,
    /// 字段名称
    ident: Option<syn::Ident>,
}

/// 处理AutoDeref
pub(crate) fn process_auto_deref(input: DeriveInput) -> proc_macro2::TokenStream {
    let AutoDerefDeriveInput {
        ident,
        generics,
        data: Data::Struct(struct_fields),
        mutable,
        field,
    } = AutoDerefDeriveInput::from_derive_input(&input).expect("Parse Input Error")
    else {
        panic!("Only Support Struct")
    };

    // 代码片段
    let mut codes = vec![];

    // 尝试匹配字段 获取类型
    let field_ty = if field.is_some() {
        // 如果field是存在的 则进行匹配
        struct_fields
            .iter()
            .find(|struct_field| struct_field.ident == field)
            .map(|field| field.ty.clone())
            .expect("Field Not Found")
    } else {
        // 如果field不存在,则取第一个字段
        struct_fields
            .iter()
            .next()
            .map(|field| field.ty.clone())
            .expect("Field Not Found")
    };

    // 生成Dref实现
    let deref_impl = quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #field_ty;
            fn deref(&self) -> &#field_ty {
                &self.#field
            }
        }
    };

    codes.push(deref_impl);

    // 生成Mut实现
    let mut_impl = if mutable {
        quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut #field_ty {
                    &mut self.#field
                }
            }
        }
    } else {
        quote! {}
    };

    codes.push(mut_impl);

    quote! { #(#codes)* }
}
