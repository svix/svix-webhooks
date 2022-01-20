use quote::quote;

use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics};

#[proc_macro_derive(ModelIn)]
pub fn derive_model_in(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    let expanded = quote! {
        impl From<#name> for <#name as crate::v1::utils::ModelIn>::ActiveModel {
            fn from(data: #name) -> Self {
                let mut ret = Self {
                    ..Default::default()
                };
                data.update_model(&mut ret);
                ret
            }
        }

    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(ModelOut)]
pub fn derive_model_out(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: BaseId` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = if name == "EventTypeOut" {
        // We want to use name as the id in this case
        quote! {
            impl #impl_generics crate::v1::utils::ModelOut for #name #ty_generics #where_clause {
                fn id_copy(&self) -> String {
                    self.name.0.clone()
                }
            }
        }
    } else {
        quote! {
            impl #impl_generics crate::v1::utils::ModelOut for #name #ty_generics #where_clause {
                fn id_copy(&self) -> String {
                    self.id.0.clone()
                }
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}
