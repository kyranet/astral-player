use std::sync::{Arc, Mutex};

use cpal::{
	traits::{DeviceTrait, HostTrait, StreamTrait},
	Device, OutputCallbackInfo, PlayStreamError, SampleFormat, SampleRate,
	Stream, StreamConfig, StreamError,
};

use crate::track::Track;

macro_rules! stream_data_callback {
	($x:ty, $track:ident) => {
		move |data: &mut [$x], _| {
			if let Ok(mut d) = $track.lock() {
				d.as_mut().map(|d| d.write_stream(data));
			}
		}
	};
}
pub struct OutputStream {
	track: Arc<Mutex<Option<Track>>>,
	device: Device,
	config: StreamConfig,
	format: SampleFormat,
	rate: SampleRate,
	stream: Option<Stream>,
}

impl OutputStream {
	pub fn try_default() -> Result<Self, StreamError> {
		let device = cpal::default_host()
			.default_output_device()
			.ok_or(StreamError::DeviceNotAvailable)?;
		let config = device
			.supported_output_configs()
			.expect("Error while querying output configs")
			.next()
			.expect("Found no supported output config")
			.with_max_sample_rate();
		let format = config.sample_format();
		let rate = config.sample_rate();

		println!("Format: {:?}. Rate: {:?}", format, rate);

		Ok(Self {
			track: Arc::default(),
			device,
			config: config.into(),
			format,
			rate,
			stream: None,
		})
	}

	pub fn init(mut self) -> Self {
		let track = self.track.clone();
		let error_callback =
			|err| eprintln!("an error occurred on output stream: {}", err);
		let stream = match &self.format {
			SampleFormat::I16 => self.device.build_output_stream(
				&self.config,
				stream_data_callback!(i16, track),
				error_callback,
			),
			SampleFormat::U16 => self.device.build_output_stream(
				&self.config,
				stream_data_callback!(u16, track),
				error_callback,
			),
			SampleFormat::F32 => self.device.build_output_stream(
				&self.config,
				stream_data_callback!(f32, track),
				error_callback,
			),
		}
		.unwrap();

		self.stream = Some(stream);
		self
	}

	pub fn play(&self) -> Result<(), PlayStreamError> {
		if let Some(stream) = &self.stream {
			stream.play()?;
		}

		Ok(())
	}

	pub fn set_track(&mut self, track: Track) {
		self.track.lock().unwrap().replace(track);
	}
}
