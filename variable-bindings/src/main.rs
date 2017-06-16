fn main() {
    // The output of this program is to be read along with source for better understanding.

    let x = "Hello, World!";
    println!("Value of a variable should be printed by using braces or moustaches. Rust will attempt its best to print the variable based on its type {}", x);

    let x = "keyword `let` lets us declare-and-initialize a variable.";
    println!("{:?}", x);

    let x = "multiple variables can be initialized in one statement; the variables and their values both are separated by comma and enclosed within parantheses.";
    println!("{}", x);

    let (x, y) = (1, 2);
    println!("{} {}", x, y);

    let x = "writing `let x = ...` implies type is inferred by the rustc based on the value assigned";
    println!("{}", x);

    let x = "types of variables can be explicitly specified followed by the variable and a colon :";
    let y : i32 = 3434324;
    println!("{} {}", x, y);

    let x = "the variable bindings declared with let are immutable. so modifying a variable results in a compiler error";
    // x = "sdsad"; // uncomment to see the error.
    println!("{}", x);

    let x = "to make a variable mutable, the keyword mut should be used after let";
    println!("{}", x);
    let mut x = "initial x";
    println!("{}", x);
    x = "x now mutated. tada!";
    println!("{}", x);

    x = "unused variables result in warning from the compiler.";
    let y = "dfdsfd";
    println!("{}", x);

    x = "when a variable is declared, but not initialized, its type needs to be specified as compiler has no chance to infer it upfront";
    let y : i32; // without the type, it would've been a compiler error.
    println!("{}", x);

    x = "uninitialized variables can't be used; attempts to do that will result in compiler error";
    println!("{}", x);
    // println!("{:?}", y); // uncomment to see the error.

    x = "each variable lives in a scope. scope is defined by blocks specified using braces. even functions are blocks in that sense!";
    {
        let (x, z) = (1, 2);
        println!("{}", x + z);
    }
    // println!("{}", z); // z is out-of scope here
    println!("{}{}", x, "; so accessing variables out of their scope will be an error.");

    let x = "we've been shadowing the var x all along. shadowing is just a way to redeclare a variable name with same or different type";
    println!("{}", x);
    let x : i32 = 4;
    println!("x as an i32 - {}", x);
    let x : &str = "random string";
    println!("x as an &str now - {}", x);
}
