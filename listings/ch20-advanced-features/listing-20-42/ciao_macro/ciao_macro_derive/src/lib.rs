use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(CiaoMacro)]
pub fn ciao_macro_derive(input: TokenStream) -> TokenStream {
    // Costruisci una rappresentazione di codice Rust come
    // albero sintattico che possiamo manipolare
    let ast = syn::parse(input).unwrap();

    // Costruisci l'implementazione del trait
    impl_ciao_macro(&ast)
}

// ANCHOR: here
fn impl_ciao_macro(ast: &syn::DeriveInput) -> TokenStream {
    let nome = &ast.ident;
    let generato = quote! {
        impl CiaoMacro for #nome {
            fn ciao_macro() {
                println!("Ciao, Macro! Il mio nome è {}!", stringify!(#nome));
            }
        }
    };
    generato.into()
}
// ANCHOR_END: here
