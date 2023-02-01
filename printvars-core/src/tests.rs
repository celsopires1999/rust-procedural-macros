#![cfg(test)]
use super::*;

#[test]
fn it_should_trace_var_a() {
    let args = quote! {a};

    let before = quote! {
        fn add(mut a: u32) -> u32 {
            a = a + 1;
            a
        }
    };

    let expected = quote! {
        fn add(mut a: u32) -> u32 {
            {
                a = a + 1;
                println!(concat!(stringify!(a), " = {:?}"), a);
            };
            a
        }

    };

    let after = trace_vars_core(args, before);
    assert_tokens_eq(&expected, &after);

    fn add(mut a: u32) -> u32 {
        {
            a = a + 1;
            println!(concat!(stringify!(a), " = {:?}"), a);
        };
        a
    }

    assert_eq!(add(1), 2);
}

#[test]
#[should_panic(
    expected = "proc-macro-error API cannot be used outside of `entry_point` invocation, perhaps you forgot to annotate your #[proc_macro] function with `#[proc_macro_error]"
)]
fn it_should_panic() {
    trace_vars_core(quote!(), quote!());
}

// region:       Help Functions
fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
// endregion:    Help Functions
