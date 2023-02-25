fn main() {
    let fib = nth_fibonacci(50);
}

fn nth_fibonacci(n: u32) -> u64 {
    let mut prev = 0;
    let mut current = 1;

    if n == 0 {
        println!("Fibonacci 0 is 0");
        return 0
    }

    let mut total = 0;

    for i in 1..n {
        total = current + prev;

        println!("Fibonacci {i} is {total}");

        prev = current;
        current = total;
    }

    total
}
