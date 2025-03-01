use fibonacci::{fibonacci, fibonacci_recursive};

#[test]
fn number_10() {
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci_recursive(10), 55);
}

#[test]
fn number_50() {
    assert_eq!(fibonacci(50), 12_586_269_025);
    assert_eq!(fibonacci_recursive(50), 12_586_269_025);
    
}
