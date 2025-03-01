pub fn fibonacci(n: u64) -> u64 {
    // iterative approach
    let mut current: u64 = 1;
    let mut prev: u64 = 0;
    for _ in 0..n {
        println!("Current: {:?}", current);
        println!("Prev : {:?}", prev);
        current = prev + current;
        prev = current - prev;
    }
    prev
}

pub fn fibonacci_recursive(n: u64) -> u64 {
    fn helper(n: u64, a: u64, b: u64) -> u64 {
        // recursive approach
        if n == 0 {
            return a;
        }
        println!("n = {}\n a = {}\n b = {}\n\r a+b = {}", n, a, b, a + b);
        // we subtract 1 from our desired fibonacci moving closer to base case
        // we give set our current answer to our previous answer
        // we give set our previous to resulint addition
        helper(n - 1, b, a + b)
    }
    helper(n, 0, 1)
}
