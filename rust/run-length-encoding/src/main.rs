use run_length_encoding as rle;

fn main() {
	println!("{:?}", rle::encode("A"));
	println!("{:?}", rle::decode("2A3BC"));
}