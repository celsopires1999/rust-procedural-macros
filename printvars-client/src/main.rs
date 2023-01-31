use printvars_derive::trace_vars;

// #[trace_vars(p, n)]
// fn factorial(mut n: u64) -> u64 {
//     let mut p = 1;
//     while n > 1 {
//         p *= n;
//         n -= 1;
//     }
//     p
// }

#[trace_vars(a)]
fn add(mut a: u32) -> u32 {
    a = a + 1;
    a
}

fn main() {
    // factorial(6);
    add(1);
}
