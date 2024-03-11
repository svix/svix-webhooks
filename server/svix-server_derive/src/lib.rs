use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, GenericParam, Generics, ItemFn};

mod aide;

use self::aide::{expand_aide_annotate, AideAnnotateArgumentList};

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

#[proc_macro_attribute]
/// Attribute macro for axum/aide handler functions that creates a new function
/// with the same name as the handler, suffixed with `_operation`, that acts as
/// an operation transformation function, automatically setting the operation
/// ID, summary and description.
///
/// # Example
/// ```ignore
/// /// This is foo!
/// #[aide_annotate]
/// fn foo() {
/// }
///
/// /// This is bar, with a custom op ID and summary
/// #[aide_annotate(op_id = "custom_id", op_summary = "Bar Operation!")]
/// fn bar() {
/// }
/// ```
pub fn aide_annotate(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args with AideAnnotateArgumentList::parse_terminated);
    let item = parse_macro_input!(input as ItemFn);

    expand_aide_annotate(args, item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
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
