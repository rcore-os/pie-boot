use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn, parse::Parse, parse_macro_input, token::Unsafe};

#[proc_macro_attribute]
pub fn start_code(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析参数中的选项，例如 naked
    let parsed_args = parse_macro_input!(args as StartCodeArgs);

    let f = parse_macro_input!(input as ItemFn);
    let attrs = f.attrs;
    let vis = f.vis;
    let name = f.sig.ident;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;

    let naked_prefix;
    let naked_attr;
    if parsed_args.naked {
        naked_attr = quote! {
            #[unsafe(naked)]
        };
        naked_prefix = quote! {
            unsafe extern "C"
        };
    } else {
        naked_attr = quote! {};
        naked_prefix = quote! {};
    };

    quote!(
        #naked_attr
        #[unsafe(link_section = ".idmap.text")]
        #(#attrs)*
        #vis #naked_prefix fn #name(#args) {
            #(#stmts)*
        }
    )
    .into()
}

struct StartCodeArgs {
    naked: bool,
}

impl Parse for StartCodeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut naked = false;

        if !input.is_empty() {
            let ident: Ident = input.parse()?;
            if ident == "naked" {
                naked = true;
            } else {
                return Err(input.error("unexpected argument, expected `naked`"));
            }
        }

        Ok(StartCodeArgs { naked })
    }
}
