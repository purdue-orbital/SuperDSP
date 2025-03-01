use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FirstStage)]
pub fn first_stage(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let g = quote! {
        use superdsp::prelude::FirstStageTrait;
        impl<I: Send + Sync> FirstStageTrait<I> for #name<I> {}
    };
    g.into()
}
