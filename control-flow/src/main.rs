fn main() {
    fib();
}

fn fib() {
    let nth_fib = 25;
    let fib_num = if nth_fib == 0 || nth_fib == 1 {
                     1
                 } else {
                     calc_fib(nth_fib)
                 };
    println!("{}th Fibonacci is {}.", nth_fib, fib_num);                    
}

fn calc_fib(x: i32) -> i32 {
   if x == 1 || x == 2 {
      1
   } else {
      calc_fib(x - 1) + calc_fib(x - 2)
   }
}
