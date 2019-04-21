fn main() {
    stack_types();
    heap_types();
}

fn stack_types() {
    let mut x = 5;
    let y = x;
    x = x + 1;
    println!("x is {} and y is {}.", x, y);
    multiply(x);
    println!("the value of x after being passed to another function is still {}.", x);
}

fn multiply(x: i32) {
    println!("2x is {}.", 2 * x);
}

fn heap_types() {
    let str = String::from("Hello, World!");
    println!("str is {}.", str);
    mutate_and_do_not_return(str);
    //println!("str after being passed to another function is {}.", str);
    println!("heap type String couldn't be referenced in the originally declared scope once it's owned by another function, unless Copy Trait is implemented");
    let str1 = String::from("Bye, World!");
    println!("str1 is {}.", str1);
    let str2 = mutate_and_return(str1);
    println!("str1 after passing to a function which took ownership and also returned it is {}.", str2);
}

fn mutate_and_do_not_return(mut str: String) {
    str.push_str(" OK!");
    println!("Inside mutate fn which now takes the ownership, the mutated str is {:?}.", str);
}

fn mutate_and_return(mut str: String) -> String {
    str.push_str(" OK!");
    println!("Inside mutate fn which now takes the ownership, the mutated str is {:?}.", str);
    str
}
