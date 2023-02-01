use printvars::trace_vars;

#[trace_vars()]
fn add(mut a: u32) -> u32 {
    a = a + 1;
    a += 1;
    a
}

fn main() {}
