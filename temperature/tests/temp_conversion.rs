use temperature::{c_to_f, f_to_c};

#[test]
fn faren_to_celcius() {
    // freezing
    assert_eq!(f_to_c(32.0), 0.0);
    // boiling
    assert_eq!(f_to_c(212.0), 100.0);
}

#[test]
fn celcius_to_faren() {
    // freezing
    assert_eq!(c_to_f(0.0), 32.0);
    // boiling
    assert_eq!(c_to_f(100.0), 212.0);
}
