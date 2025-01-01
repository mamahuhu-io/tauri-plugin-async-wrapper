extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn async_wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Analytic function
    let input = parse_macro_input!(item as ItemFn);

    // Extract the function name and contents
    let fn_name = &input.sig.ident;
    let fn_inputs = &input.sig.inputs;
    let fn_output = &input.sig.output;
    let fn_block = &input.block;

    // Generate wrapper code
    let expanded = quote! {
        #[tauri::command]
       pub async fn #fn_name(#fn_inputs) #fn_output {
            tauri::async_runtime::spawn_blocking(move || {
                #fn_block
            })
            .await
            .map_err(|e| e.to_string())?
        }
    };

    TokenStream::from(expanded)
}
