#![cfg(test)]
use super::*;

#[test]
fn test_first() {
    let args = quote! {a};

    let input = quote! {
        fn add(mut a: u32) -> u32 {
            a = a + 1;
            a
        }
    };

    let output = trace_vars_core(args, input);
    let _expected = "fn add (mut a : u32) -> u32 { { a = a + 1 ; println ! (concat ! (stringify ! (a) , \" = {:?}\") , a) ; } ; a }";
    // assert_eq!(expected, output.to_string());
    dbg!(output.to_string());
}
