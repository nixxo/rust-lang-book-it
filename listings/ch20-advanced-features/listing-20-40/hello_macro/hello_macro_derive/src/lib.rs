use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Costruisci una rappresentazione di codice Rust come
    // albero sintattico che possiamo manipolare
    let ast = syn::parse(input).unwrap();

    // Costruisci l'implementazione del trait
    impl_hello_macro(&ast)
}
