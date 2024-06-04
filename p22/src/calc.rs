/// Computes the conversion from celsius to farenheit
/// ```
/// assert_eq!(p22::calc::celsius2farenheit(10), 50);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    ((celsius * 9) / 5) + 32
}

/// Computes the conversion from farenheit to celsius
/// ```
/// assert_eq!(p22::calc::farenheit2celsius(50), 10);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    ((farenheit - 32) * 5) / 9
}

/// Computes the Fibonacci number of a n using loop
/// ```
/// assert_eq!(p22::calc::fibonacci_loop(6), 8);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut next = f0 + f1;
    let mut x = 3;

    if n == 0 {
        return f0;
    } else if n == 1 {
        return f1;
    } else if n == 2 {
        return next;
    }

    loop {
        if x > n {
            break next;
        }
        f0 = f1;
        f1 = next;
        next = f0 + f1;
        x += 1;
    }
}

/// Computes the Fibonacci number of a n using match and recursion
/// ```
/// assert_eq!(p22::calc::fibonacci_rec(6), 8);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0u64,
        1 => 1u64,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
