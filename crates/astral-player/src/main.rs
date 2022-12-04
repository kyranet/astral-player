use std::{
	env,
	fs::File,
	io::{BufReader, Write},
	path::Path,
};

use console::{Emoji, Style, Term};
use rodio::{Decoder, OutputStream, Sink, Source};

fn main() {
	let args: Vec<String> = env::args().collect();
	let path = Path::new(args.get(1).expect("Expected a file path"));
	let file = File::open(path).expect("Could not open specified file");

	let (_stream, handle) = OutputStream::try_default().unwrap();
	let sink = Sink::try_new(&handle).unwrap();

	let source = Decoder::new(BufReader::new(file)).unwrap();

	let cyan = Style::new().cyan();
	let mut term = Term::stdout();
	term.set_title("Astral Player");
	let _ = term.hide_cursor();

	let _ = writeln!(
		term,
		"Playing    : {}",
		cyan.apply_to(
			path.file_name().and_then(|f| f.to_str()).unwrap_or("Unknown")
		)
	);
	let _ = writeln!(
		term,
		"Sample rate: {}Hz",
		cyan.apply_to(source.sample_rate())
	);
	let _ = writeln!(
		term,
		"Duration   : {}\n\n",
		cyan.apply_to(
			source
				.total_duration()
				.map(|f| format!("{f:?}"))
				.unwrap_or("Unknown".to_string())
		)
	);

	let line = format!(
		"[{}] Quit [{}] Stop [{}] Play",
		cyan.apply_to('Q'),
		cyan.apply_to('S'),
		cyan.apply_to('P')
	);
	let _ = term.write_line(line.as_str());
	let _ = term.move_cursor_up(2);

	sink.append(source.repeat_infinite());

	loop {
		if let Ok(char) = term.read_char() {
			match char {
				'q' => {
					let _ = write!(term, "\r{}Quitting...", Emoji("⏹️ ", ""));
					let _ = term.move_cursor_down(1);
					let _ = term.clear_line();
					let _ = term.move_cursor_up(1);
					break;
				}
				's' => {
					let _ = write!(term, "\r{}Paused     ", Emoji("⏸️ ", ""));
					sink.pause();
				}
				'p' => {
					let _ = write!(term, "\r{}Playing    ", Emoji("⏯️ ", ""));
					sink.play();
				}
				_ => (),
			}
		}
	}
}
