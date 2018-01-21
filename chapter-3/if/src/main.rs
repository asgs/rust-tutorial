fn main() {
	println!("if conditions in Rust are similar to languages like C, Java in that they have the typical if, else if, else structure.");
	let x = 4;
	if x != 4 {
		println!("x != 4");
	} else {
		println!("x == 4");
	}
	println!("in addition to that, if/else/else-if blocks evaluate to the last expression in their corresponding block.");
	let y = if x == 4 {
		5
		} else {
		4
		};
	println!("y is {}.", y);
	let z = if x != 4 { 5 } else { 6 };
	println!("z is {}.", z);
}

