use printvars::trace_vars;
#[test]
fn trace_a() {
    #[trace_vars(a)]
    fn add(mut a: u32) -> u32 {
        a = a + 1;
        a += 1;
        a
    }

    assert!(add(2) == 4);
}
