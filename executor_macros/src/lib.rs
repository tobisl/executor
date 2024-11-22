#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse die Eingabe als Funktion
    let input_fn = parse_macro_input!(input as ItemFn);

    // Extrahiere die Funktionseigenschaften
    let fn_vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_sig = &input_fn.sig;
    let fn_body = &input_fn.block;

    // Stelle sicher, dass es eine `async`-Funktion ist
    if fn_sig.asyncness.is_none() {
        return syn::Error::new_spanned(
            fn_sig.fn_token,
            "`async_main` kann nur auf async Funktionen angewendet werden",
        )
        .to_compile_error()
        .into();
    }

    // Extrahiere den Rückgabetyp
    let return_type = match &fn_sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ty) => quote!(#ty),
    };

    // Generiere das neue `fn main`
    let result = quote! {
        #fn_vis fn #fn_name() ->  #return_type  {
            let fut = async {
                #fn_body
            };
            executor::block_on(fut)
        }
    };

    result.into()
}
