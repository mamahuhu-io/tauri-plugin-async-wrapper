extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// A procedural macro that wraps a function with an asynchronous runtime using `spawn_blocking`.
/// This macro converts a synchronous function into an asynchronous one and registers it as a Tauri command.
///
/// # Example
/// ```rust
/// #[async_wrapper]
/// pub fn example_function(x: i32) -> Result<String, String> {
///     Ok(format!("Result: {}", x))
/// }
/// ```
/// After applying the macro, the function will be accessible as an asynchronous Tauri command.
#[proc_macro_attribute]
pub fn async_wrapper(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function definition
    let input = parse_macro_input!(item as ItemFn);
    // Generate the asynchronous wrapper for the input function
    generate_async_wrapper(&input).into()
}

/// Generates an asynchronous wrapper for a given synchronous function.
/// The wrapped function runs on a separate thread to avoid blocking the main thread.
///
/// # Arguments
/// * `input` - A reference to the parsed function definition.
///
/// # Returns
/// A `proc_macro2::TokenStream` containing the generated asynchronous function.
fn generate_async_wrapper(input: &ItemFn) -> proc_macro2::TokenStream {
    let fn_name = &input.sig.ident; // Function name
    let fn_inputs = &input.sig.inputs; // Function arguments
    let fn_output = &input.sig.output; // Function return type
    let fn_block = &input.block; // Function body

    // Generate the asynchronous wrapper code
    quote! {
        #[tauri::command]
        pub async fn #fn_name(#fn_inputs) #fn_output {
            tauri::async_runtime::spawn_blocking(move || {
                #fn_block
            })
            .await
            .map_err(|e| e.to_string())?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use syn::parse_quote;

    /// Tests the `async_wrapper` macro by validating the generated asynchronous wrapper function.
    #[test]
    fn test_async_wrapper() {
        // Simulate a user-defined input function
        let input_fn: ItemFn = parse_quote! {
            pub fn test_function(x: i32, y: String) -> Result<String, String> {
                Ok(format!("{} {}", x, y))
            }
        };

        // Generate the asynchronous wrapper
        let output = generate_async_wrapper(&input_fn);

        // Expected output for the asynchronous wrapper
        let expected_output: TokenStream2 = quote! {
            #[tauri::command]
            pub async fn test_function(x: i32, y: String) -> Result<String, String> {
                tauri::async_runtime::spawn_blocking(move || {
                    { Ok(format!("{} {}", x, y)) }
                })
                .await
                .map_err(|e| e.to_string())?
            }
        };

        // Verify that the generated code matches the expected output
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
