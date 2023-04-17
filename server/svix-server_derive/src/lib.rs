use quote::{format_ident, quote, ToTokens};

use syn::{
    parse_macro_input, parse_quote, AttributeArgs, DeriveInput, GenericParam, Generics, ItemFn,
    NestedMeta,
};

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

fn doc_comment_from_attributes(attributes: &Vec<syn::Attribute>) -> Option<String> {
    let mut doc_comment_lines = Vec::new();

    for attr in attributes {
        let meta = attr
            .parse_meta()
            .expect("Failed to parse fn attribute as Meta");

        if let syn::Meta::NameValue(meta) = meta {
            if meta.path.to_token_stream().to_string() != "doc" {
                continue;
            }
            if let syn::Lit::Str(doc) = meta.lit {
                doc_comment_lines.push(doc.value().trim().to_string());
            }
        }
    }

    if doc_comment_lines.is_empty() {
        return None;
    }
    Some(doc_comment_lines.join("\n"))
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[proc_macro_attribute]
/// Attribute macro for axum/aide handler functions that creates a new function
/// with the same name as the handler, suffixed with `_operation`, that acts as
/// an operation transformation function, automatically setting the operation
/// ID, summary and description.
///
/// # Example
/// ```
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
    let args = parse_macro_input!(args as AttributeArgs);

    let item = parse_macro_input!(input as ItemFn);

    // By default, use the function's name as the operation id.
    let mut operation_id = item.sig.ident.to_string();
    // The operation summary is the title-cased version of the original
    // function name.
    let mut operation_summary = operation_id
        .split('_')
        .map(title_case)
        .collect::<Vec<String>>()
        .join(" ");
    // The documentation function's name will always be the name of the
    // original function suffixed with `_operation`.
    let operation_ident = format_ident!("{}_operation", item.sig.ident);
    let visibility = item.vis.clone();

    // Allow overriding operation ID and summary via arguments
    for arg in args {
        if let NestedMeta::Meta(syn::Meta::NameValue(meta)) = arg {
            let arg_id = meta.path.to_token_stream().to_string();

            let value = if let syn::Lit::Str(s) = meta.lit {
                s.value()
            } else {
                return syn::Error::new_spanned(meta.lit, "Unexpected literal, expected a string")
                    .into_compile_error()
                    .into();
            };

            match arg_id.as_str() {
                "op_id" => operation_id = value,
                "op_summary" => operation_summary = value,
                _ => {
                    let path = meta.path.to_token_stream().to_string();
                    let msg =
                        format!("Unknown argument `{path}`, expected `op_id` or `op_summary`",);
                    return syn::Error::new_spanned(meta.path, msg)
                        .into_compile_error()
                        .into();
                }
            }
        } else {
            return syn::Error::new_spanned(arg, "Unexpected argument")
                .into_compile_error()
                .into();
        }
    }

    let description = doc_comment_from_attributes(&item.attrs);

    if description.is_none() {
        let msg = "An annotated handler must have a doc comment for its description.";
        return syn::Error::new(item.sig.ident.span(), msg)
            .into_compile_error()
            .into();
    }

    let f = item.into_token_stream();

    let out = quote! {
        #f

        #visibility fn #operation_ident(op: ::aide::transform::TransformOperation) -> ::aide::transform::TransformOperation {
            op
                .id(#operation_id)
                .summary(#operation_summary)
                .description(#description)
                .response_with::<401, ::axum::Json<crate::error::StandardHttpError>, _>(|op| {
                    op.description("Unauthorized")
                })
                .response_with::<403, ::axum::Json<crate::error::StandardHttpError>, _>(|op| {
                    op.description("Forbidden")
                })
                .response_with::<404, ::axum::Json<crate::error::StandardHttpError>, _>(|op| {
                    op.description("Not Found")
                })
                .response_with::<409, ::axum::Json<crate::error::StandardHttpError>, _>(|op| {
                    op.description("Conflict")
                })
                .response_with::<422, ::axum::Json<crate::error::ValidationHttpError>, _>(|op| {
                    op.description("Validation Error")
                })
                .response_with::<429, ::axum::Json<crate::error::StandardHttpError>, _>(|op| {
                    op.description("Too Many Requests")
                })
        }
    };
    proc_macro::TokenStream::from(out)
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
