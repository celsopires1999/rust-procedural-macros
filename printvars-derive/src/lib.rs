extern crate proc_macro;
use printvars_core::trace_vars_core;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn trace_vars(args: TokenStream, input: TokenStream) -> TokenStream {
    trace_vars_core(args, input)
}
