fn main() {
    one_two(7);
    one_two(63);
    one_two(0);
    one_two(10);
}

fn one_two(index: u8) -> (u8, u8) {
	let cartesian: (u8, u8) = (index % 8, index / 8);
	println!("convert 1D {} to cartesian {:?}", index, cartesian);
	return cartesian;
}
