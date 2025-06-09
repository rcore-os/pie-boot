use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn idmap(_args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);
    let attrs = f.attrs;
    let vis = f.vis;
    let name = f.sig.ident;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    quote!(
        #[unsafe(no_mangle)]
        #[unsafe(naked)]
        #[unsafe(link_section = ".idmap.text")]
        #(#attrs)*
        #vis unsafe extern "C" fn #name(#args) {
            #(#stmts)*
        }
    )
    .into()
}
