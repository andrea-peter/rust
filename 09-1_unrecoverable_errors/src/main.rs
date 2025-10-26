// To have a stack trace printed
// export RUST_BACKTRACE=1
// or
// export RUST_BACKTRACE=full
fn main() {
    // Implicit panic
    let v = vec![1, 2, 3];
    v[99];

    // Explicit panic
    panic!("Crash and burn");
}
