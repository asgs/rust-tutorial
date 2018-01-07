fn main() {
    print_number(3);
    println!("{:?}", sum_two_numbers(5, 10));
    expressions_vs_statements();
    println!("{:?}", early_return_in_functions());
    //panicing_function(); // Uncomment to see panics!
    println!("A variable can be bound to function - called function-bindings. the type can be explicitly specified or let Rustc infer it.", );
    let function_pointer = square;
    println!("{:?}", function_pointer(3));
    let fn_ptr : fn(i32) -> i32 = square;
    println!("{:?}", fn_ptr(5));
}

fn print_number(x : i32) {
    println!("functions take arguments with types to be specified explicitly.", );
    println!("Type inference could've been applied, but then, code readability is improved when functions clearly declare what the types of the arguments they accept are.", );
    println!("{:?}", x);
}

fn sum_two_numbers(x : i32, y : i32) -> i32 {
    println!("A function can return a value and only one value. The return type is specified after the argument pararantheses separated by a hyphen and greater than symbol.");
    println!("The last expression in a function becomes the return-value sent to the caller");
    println!("Since an expression doesn't end with a semi-colon, the last statement in a function returning the value can't end with a semi-colon.");
    x + y // adding a semi-colon at the end is an error.
}

fn expressions_vs_statements() {
    println!("Only two types of statements in Rust - declaration-based and expression-based.");
    println!("declaration-based statements don't have a value so trying to assign that to another variable is an error.");
    // let x = (let y = 5); // So this is not possible.
    let (x, y) : (i32, i32);
    println!("Assigning a value to a variable doesn't hold a value in itself ('it has only one owner' - to quote the Rust docs), so it's pointless to assign that expression to another variable.", );
    let x = y = 5;
    println!("e.g. x = y = 5 (assuming x and y are declared upfront) assigns 5 to y but not x. x is assigned an empty tuple ().");
    println!("{:?} {:?}", x, y);
}

fn early_return_in_functions() -> i32 {
    let x = 1;
    println!("Rust does support a return keyword which could be used to indicate that a value needs to be returned before reaching the end of the function", );
    return 2;
    x // This should've been a compiler error, but it's pointless!
}

fn panicing_function() -> ! {
    println!("when panic occurs, we can set RUST_BACKTRACE to any non-zero value to print the stacktrace.");
    println!("Interesting part diverging functions (that is, ones that panic) can be assigned to any type.", );
    panic!("I panicked!")
}

fn square(x: i32) -> i32 {
    x * x
}
