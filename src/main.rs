fn main() {
    let test = index_cartesian(10);
	cartesian_human(test);
}

fn index_cartesian(index: u8) -> (u8, u8) {
	let cartesian: (u8, u8) = (index % 8, index / 8);
	println!("convert index {} to cartesian {:?}", index, cartesian);
	return cartesian;
}

fn cartesian_human(cartesian: (u8,u8)) -> String {
	let human: String = format!("{}{}", char::from_u32(cartesian.0 as u32 + 65).unwrap(), (cartesian.1 + 1));
	println!("convert cartesian {:?} to human {}", cartesian, human);
	return human;
}
