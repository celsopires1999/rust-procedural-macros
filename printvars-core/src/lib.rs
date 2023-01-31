extern crate proc_macro;
use std::collections::HashSet;

use proc_macro2::TokenStream;
// use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{
    fold::{self, Fold},
    parse::{Parse, ParseStream},
    parse2, parse_quote,
    punctuated::Punctuated,
    Expr, Ident, ItemFn, Local, Pat, Result, Stmt, Token,
};

pub fn trace_vars_core(args: TokenStream, input: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(input as ItemFn);
    let input = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };
    // parse the arguments
    // let mut args = parse_macro_input!(args as Args);
    let mut args = match parse2::<Args>(args) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };
    // create the output
    let output = args.fold_item_fn(input);
    // return the TokenStream
    TokenStream::from(quote!(#output))
}

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        // parses a,b,c, or a,b,c where a,b and c are Ident
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            // for changing assignment like a=5
            Expr::Assign(e) => {
                // check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
                    // continue with default transversal using default methods
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
            // for changing assignment and operation like a+=1
            Expr::AssignOp(e) => {
                // check should print
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
                    // continue with default behavior
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
            // continue with default behavior for rest of expressions
            _ => fold::fold_expr(self, e),
        }
    }

    // for let statements like let d=9
    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

impl Args {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
                // variable shouldn't start with ::
                if e.path.leading_colon.is_some() {
                    false
                // should be a single variable like `x=8` not n::x=0
                } else if e.path.segments.len() != 1 {
                    false
                } else {
                    // get the first part
                    let first = e.path.segments.first().unwrap();
                    // check if the variable name is in the Args.vars hashset
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

    // used for checking if to print let i=0 etc or not
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
            // check if variable name is present in set
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

    // manipulate tree to insert print statement
    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
        // recursive call on right of the assignment statement
        let right = fold::fold_expr(self, right);
        // returning manipulated sub-tree
        parse_quote!({
            #left #op #right;
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        })
    }

    // manipulating let statement
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
        // get the variable name of assigned variable
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
        // new sub tree
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}

mod tests;
