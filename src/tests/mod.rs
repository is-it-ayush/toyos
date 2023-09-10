//! A custom test runner. We can't use the rust's test runnner
//! since it relies upon std lib.

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("[ToyOS] Executing tests...");
    for test in tests {
        test();
    }
}
