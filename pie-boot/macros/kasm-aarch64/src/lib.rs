use std::ops::Deref;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct Asm(Vec<String>);

impl From<String> for Asm {
    fn from(value: String) -> Self {
        Asm(value
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|o| !o.is_empty())
            .collect())
    }
}

impl Deref for Asm {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn dcache_line_size(reg: &str, tmp: &str) -> Asm {
    format!(
        "
    mrs {tmp}, ctr_el0
    ubfm  {tmp}, {tmp}, #16, #19
    mov {reg}, #4
    lsl {reg}, {reg}, {tmp}"
    )
    .into()
}

struct DCacheMacroArgs {
    section: Option<syn::LitStr>,
}
impl Parse for DCacheMacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut section = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            if ident == "section" {
                input.parse::<Token![=]>()?;
                let value: syn::LitStr = input.parse()?;
                section = Some(value);
            } else {
                return Err(input.error("Unknown argument"));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { section })
    }
}
///`pub fn __dcache_inval_poc(_start: usize, _end: usize)`
#[proc_macro]
pub fn def_dcache_inval_poc(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DCacheMacroArgs);

    // 默认 section 名称
    let section = args
        .section
        .map_or_else(|| String::from(".text"), |lit| lit.value());

    let dcache_line_size = dcache_line_size("x2", "x3");

    quote! {
    /// 确保 [start, end) 区间内的 D-cache 行被无效化。区间两端的非对齐 cache line 也会被清理以防止数据丢失。
    ///
    /// # 参数
    ///
    /// - `start`: 要操作区域的起始地址（内核虚拟地址）
    /// - `end`: 要操作区域的结束地址（内核虚拟地址）
    #[unsafe(naked)]
    #[unsafe(link_section = #section)]
    pub unsafe extern "C" fn __dcache_inval_poc(_start: usize, _end: usize) {
        core::arch::naked_asm!(
            #(#dcache_line_size),*,
"sub	x3, x2, #1
	tst	x1, x3				// end cache line aligned?
	bic	x1, x1, x3
	b.eq	1f
	dc	civac, x1			// clean & invalidate D / U line
1:	tst	x0, x3				// start cache line aligned?
	bic	x0, x0, x3
	b.eq	2f
	dc	civac, x0			// clean & invalidate D / U line
	b	3f
2:	dc	ivac, x0			// invalidate D / U line
3:	add	x0, x0, x2
	cmp	x0, x1
	b.lo	2b
	dsb	sy
	ret"
        )
    }
        }
    .into()
}

#[proc_macro]
pub fn def_adr_l(_input: TokenStream) -> TokenStream {
    quote!(
            core::arch::global_asm!(
            r"
	.macro	adr_l, dst, sym
	adrp	\dst, \sym
	add	\dst, \dst, :lo12:\sym
	.endm
"
        );
            )
    .into()
}
