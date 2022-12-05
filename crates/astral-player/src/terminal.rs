use std::io::{self, Stdout};

use crossterm::{
	cursor, execute,
	terminal::{
		disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
		LeaveAlternateScreen, SetTitle,
	},
};

pub struct Terminal {
	pub handle: Stdout,
}

impl Terminal {
	pub fn with_stdout(handle: Stdout) -> Self {
		Terminal { handle }
	}

	pub fn init(&mut self) -> io::Result<&mut Self> {
		enable_raw_mode()?;
		execute!(
			self.handle,
			EnterAlternateScreen,
			cursor::Hide,
			SetTitle("Astral Player")
		)
		.unwrap();
		Ok(self)
	}
}

impl Drop for Terminal {
	fn drop(&mut self) {
		disable_raw_mode().unwrap();
		execute!(self.handle, LeaveAlternateScreen, cursor::Show).unwrap();
	}
}

pub mod keys {
	use crossterm::event::{KeyCode, KeyModifiers};

	pub type KeyPair = (KeyModifiers, KeyCode);

	pub const QUIT: KeyPair = (KeyModifiers::NONE, KeyCode::Char('q'));
	pub const SIGINT: KeyPair = (KeyModifiers::CONTROL, KeyCode::Char('c'));
	pub const TOGGLE_PLAY: KeyPair = (KeyModifiers::NONE, KeyCode::Char('p'));
}
