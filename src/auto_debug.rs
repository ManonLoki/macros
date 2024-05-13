use darling::{self, ast, FromDeriveInput, FromField};
use quote::quote;
use syn::DeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(auto_debug))]
pub(crate) struct AutoDebugDeriveInput {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), AutoDebugField>,
}

#[derive(Debug, FromField)]
#[darling(attributes(auto_debug))]
pub(crate) struct AutoDebugField {
    ident: Option<syn::Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> proc_macro2::TokenStream {
    let AutoDebugDeriveInput {
        ident,
        generics,
        data: ast::Data::Struct(fields),
    } = AutoDebugDeriveInput::from_derive_input(&input).expect("Parse Input Error")
    else {
        panic!("Only Support Struct")
    };

    println!("Fields:{:#?}", fields);

    // 解析Fields
    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("Not Found Ident");
        let skip = field.skip;

        if skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        impl std::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    }
}
