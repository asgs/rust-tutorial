fn main() {
    temp_conversion();
    fib();
}

fn temp_conversion() {
    let c = 30.0;
    println!("{} Celcius is equivalent to {} Fahrenheit.", c, (1.8 * c) + 32.0);
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
