use std::fs::File;

use symphonia::{
	core::{
		audio::SampleBuffer, codecs::DecoderOptions, errors::Result,
		formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions,
		probe::Hint,
	},
	default::{get_codecs, get_probe},
};

pub struct Track {
	pub data: Vec<f32>,
	// decoder: Box<dyn Decoder>,
	// format: Box<dyn FormatReader>,
	// buffer: SampleBuffer<f32>,
	// spec: SignalSpec,
}

impl Track {
	pub fn try_new(file: Box<File>) -> Result<Track> {
		// Create the media source stream using the boxed media source from
		// above.
		let mss = MediaSourceStream::new(file, Default::default());

		// Create a hint to help the format registry guess what format reader is
		// appropriate. In this example we'll leave it empty.
		let hint = Hint::new();

		// Use the default options when reading and decoding.
		let format_opts = FormatOptions::default();
		let metadata_opts = MetadataOptions::default();
		let decoder_opts = DecoderOptions::default();

		// Probe the media source stream for a format.
		let probed =
			get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

		// Get the format reader yielded by the probe operation.
		let mut format = probed.format;

		// Get the default track.
		let track = format.default_track().unwrap();

		// Create a decoder for the track.
		let mut decoder =
			get_codecs().make(&track.codec_params, &decoder_opts).unwrap();

		// Store the track identifier, we'll use it to filter packets.
		let track_id = track.id;

		let mut sample_buf = None;
		let mut data: Vec<f32> = match track.codec_params.n_frames {
			Some(capacity) => Vec::with_capacity(capacity as usize),
			None => Vec::new(),
		};

		loop {
			// Get the next packet from the format reader.
			let packet = match format.next_packet() {
				Ok(packet) => packet,
				Err(e) => match e {
					symphonia::core::errors::Error::IoError(_) => break,
					_ => return Err(e),
				},
			};

			// If the packet does not belong to the selected track, skip it.
			if packet.track_id() != track_id {
				continue;
			}

			// Decode the packet into audio samples, ignoring any decode errors.
			match decoder.decode(&packet) {
				Ok(audio_buf) => {
					// The decoded audio samples may now be accessed via the
					// audio buffer if per-channel slices of samples in their
					// native decoded format is desired. Use-cases where
					// the samples need to be accessed in an interleaved order
					// or converted into another sample format, or a byte buffer
					// is required, are covered by copying the audio buffer into
					// a sample buffer or raw sample buffer, respectively. In
					// the example below, we will copy the audio buffer into a
					// sample buffer in an interleaved order while also
					// converting to a f32 sample format.

					// If this is the *first* decoded packet, create a sample
					// buffer matching the decoded audio buffer format.
					if sample_buf.is_none() {
						// Get the audio buffer specification.
						let spec = *audio_buf.spec();

						// Get the capacity of the decoded buffer. Note: This is
						// capacity, not length!
						let duration = audio_buf.capacity() as u64;

						// Create the f32 sample buffer.
						sample_buf =
							Some(SampleBuffer::<f32>::new(duration, spec));
					}

					// Copy the decoded audio buffer into the sample buffer in
					// an interleaved format.
					if let Some(buf) = &mut sample_buf {
						buf.copy_interleaved_ref(audio_buf);

						// The samples may now be access via the `samples()`
						// function.
						data.extend_from_slice(buf.samples());
						print!("\rDecoded {} samples", data.len());
					}
				}
				Err(e) => match e {
					symphonia::core::errors::Error::IoError(_) => break,
					symphonia::core::errors::Error::DecodeError(_) => (),
					_ => return Err(e),
				},
			}
		}

		Ok(Self { data })
	}
}
