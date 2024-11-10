use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, ItemFn, AttributeArgs, NestedMeta};

/// Define the custom attribute-like macro, which takes a route parameter.
#[proc_macro_attribute]
pub fn GET(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input parameters
    let args = parse_macro_input!(attr as AttributeArgs);

    // Check if there is exactly one argument and it's a string literal
    let route = match args.as_slice() {
        [NestedMeta::Lit(syn::Lit::Str(lit_str))] => lit_str.value(),
        _ => {
            return syn::Error::new_spanned(
                quote! { #(#args)* },
                "Expected exactly one string literal as argument",
            )
            .to_compile_error()
            .into();
        }
    };

    // Parse the function to which the attribute is applied
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Generate a statement to print the route
    let route_print = quote! {
        println!("Handling route '{}'", #route);
    };

    // Insert the route_print statement at the beginning of the function body
    input_fn.block.stmts.insert(0, syn::parse_quote! { #route_print });

    // Return the modified function as a TokenStream
    let output = quote! {
        #input_fn
    };

    TokenStream::from(output)
}