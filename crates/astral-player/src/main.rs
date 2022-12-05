mod terminal;

use std::{
	env,
	fs::File,
	io::{BufReader, Write},
	path::Path,
};

use crossterm::{
	cursor,
	event::{read, Event},
	execute,
	style::Stylize,
	Result as TermResult,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use terminal::{keys::*, Terminal};

fn main() -> TermResult<()> {
	let args: Vec<String> = env::args().collect();
	let path = Path::new(args.get(1).expect("Expected a file path"));
	let file = File::open(path).expect("Could not open specified file");

	let (_stream, handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&handle).unwrap();

	let source = Decoder::new(BufReader::new(file)).unwrap();

	let mut term = Terminal::with_stdout(std::io::stdout());
	term.init()?;

	println!(
		"Playing    : {}",
		path.file_name().and_then(|f| f.to_str()).unwrap_or("Unknown").cyan(),
	);
	println!("Sample rate: {} Hz", source.sample_rate().to_string().cyan());
	println!(
		"Duration   : {}\n\n",
		source
			.total_duration()
			.map(|f| format!("{f:?}"))
			.unwrap_or("Unknown".to_string())
			.cyan(),
	);
	print!("[{}] Quit [{}] Toggle Play", "Q".cyan(), "P".cyan());
	execute!(term.handle, cursor::MoveUp(1))?;

	sink.append(source.repeat_infinite());

	loop {
		if let Event::Key(key_event) = read()? {
			match (key_event.modifiers, key_event.code) {
				SIGINT | QUIT => {
					write!(term.handle, "\r⏹️ Quitting...")?;
					break;
				}
				TOGGLE_PLAY => {
					if sink.is_paused() {
						write!(term.handle, "\r⏯️ Playing")?;
						sink.play();
					} else {
						write!(term.handle, "\r⏸️ Paused ")?;
						sink.pause();
					}
				}
				_ => (),
			}
			term.handle.flush()?;
		}
	}
	Ok(())
}
