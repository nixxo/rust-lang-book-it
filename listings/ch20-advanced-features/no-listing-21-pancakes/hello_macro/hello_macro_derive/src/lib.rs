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

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let nome = &ast.ident;
    let generato = quote! {
        impl HelloMacro for #nome {
            fn hello_macro() {
                println!("Ciao, Macro! Il mio nome è {}!", stringify!(#nome));
            }
        }
    };
    generato.into()
}
