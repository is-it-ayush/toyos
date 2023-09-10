//! A custom test runner. We can't use the rust's test runnner
//! since it relies upon std lib.

use super::{println, print};

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::println;

    println!("[ToyOS] Executing tests...");
    for test in tests {
        test();
    }
}


#[test_case]
pub fn test_test() {
    print!("Testing the runner...");
    assert_eq!(1, 1);
    println!("[ok]");
}
