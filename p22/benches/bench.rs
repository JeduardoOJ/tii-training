#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use std::hint::black_box;
    use test::Bencher;

    #[bench]
    fn benchmark_celsius2farenheit(b: &mut Bencher) {
        b.iter(|| black_box(p22::calc::celsius2farenheit(10)));
    }

    #[bench]
    fn benchmark_farenheit2celsius(b: &mut Bencher) {
        b.iter(|| black_box(p22::calc::farenheit2celsius(50)));
    }

    #[bench]
    fn benchmark_fibonacci_loop(b: &mut Bencher) {
        b.iter(|| black_box(p22::calc::fibonacci_loop(6)));
    }

    #[bench]
    fn benchmark_fibonacci_loop2(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10 {
                black_box(p22::calc::fibonacci_loop(i));
            }
        });
    }

    #[bench]
    fn benchmark_fibonacci_rec(b: &mut Bencher) {
        b.iter(|| black_box(p22::calc::fibonacci_rec(6)));
    }

    #[bench]
    fn benchmark_fibonacci_rec2(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..10 {
                black_box(p22::calc::fibonacci_rec(i));
            }
        });
    }
}
