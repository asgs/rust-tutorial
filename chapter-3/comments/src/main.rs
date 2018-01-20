fn main() {
	// This is a line comment extending till the end of this line.
	let x = 4;
	println!("x after incrementing by one is {}.", increment_by_one(x));
}

/// This is a Rust Doc comment supporting MD syntax.
/// This *is* a _Rust DOC_ comment.
/// # Examples
/// ```
/// let five = 5;

/// assert_eq!(6, increment_by_one(5));
/// # fn increment_by_one(x: i32) -> i32 {
/// #	x + 1
/// # }
/// ```
fn increment_by_one(x: i32) -> i32 {
	x + 1
	// This fn has a Rust Doc /// specified above
	// with example code writing a test. To run
	// this test while generating rustdoc, the
	// cmd rustdoc src/main.rs --test is run.
}
