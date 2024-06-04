#[test]
fn test_celsius2farenheit() {
    assert_eq!(p22::calc::celsius2farenheit(10), 50);
}
#[test]
fn test_farenheit2celsius() {
    assert_eq!(p22::calc::farenheit2celsius(50), 10);
}
#[test]
fn test_fibonacci_loop() {
    assert_eq!(p22::calc::fibonacci_loop(6), 8);
}
#[test]
fn test_fibonacci_rec() {
    assert_eq!(p22::calc::fibonacci_rec(6), 8);
}
