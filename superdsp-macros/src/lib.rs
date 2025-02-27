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

#[proc_macro_derive(LastStage)]
pub fn last_stage(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let g = quote! {
        use superdsp::prelude::LastStageTrait;
        impl<O: Send + Sync> LastStageTrait<O> for #name<O> {}
    };
    g.into()
}


//--------------------------------------------------------------------------------------------------

// Below this is meant for internal use

#[proc_macro_derive(FirstStageCrate)]
pub fn first_stage_crate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let g = quote! {
        use crate::prelude::FirstStageTrait;
        impl<I: Clone + Send + Sync + Debug> FirstStageTrait<I> for #name<I> {}
    };
    g.into()
}

#[proc_macro_derive(LastStageCrate)]
pub fn last_stage_crate(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let g = quote! {
        use crate::prelude::LastStageTrait;
        impl<O: Send + Sync + Clone + Debug + Default> LastStageTrait<O> for #name<O> {}
    };
    g.into()
}