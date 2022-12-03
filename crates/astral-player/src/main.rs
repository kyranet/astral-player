use std::{env, fs::File, path::Path};

use audio::track::Track;

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = Path::new(args.get(1).expect("Expected a file path"));
	let file =
		Box::new(File::open(path).expect("Could not open specified file"));
	let track = Track::try_new(file).expect("Could not decode audio file");

	println!("\nReceived {:?} decoded samples", track.data.len());
}
