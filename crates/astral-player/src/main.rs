use std::{env, fs::File, path::Path, thread};

use audio::{stream, track::Track};

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = Path::new(args.get(1).expect("Expected a file path"));
	let file =
		Box::new(File::open(path).expect("Could not open specified file"));
	let track = Track::try_new(file).expect("Could not decode audio file");
	let duration = track.duration();

	println!("");
	println!("Duration: {:?}", duration);

	let mut stream = stream::OutputStream::try_default()
		.expect("Could not open an output stream")
		.init();
	stream.set_track(track);
	stream.play().expect("Failed to play the stream");

	thread::sleep(duration);
}
