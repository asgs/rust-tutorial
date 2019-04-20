fn main() {
    let nth_fib = 25;
    let fib_num = if nth_fib == 0 || nth_fib == 1 {
                     1
                 } else {
                     fibonacci(nth_fib)
                 };
    println!("{}th Fibonacci is {}.", nth_fib, fib_num);                    
}

fn fibonacci(x: i32) -> i32 {
   if x == 1 || x == 2 {
      1
   } else {
      fibonacci(x - 1) + fibonacci(x - 2)
   }
}
