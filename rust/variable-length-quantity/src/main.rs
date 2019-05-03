use variable_length_quantity as vlq;

fn main() {
	println!("{:?}", vlq::bytes_value_to_single(vec![0x7f]));
}