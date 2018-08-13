// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

extern crate libc;
extern crate nix;
#[macro_use]
extern crate lazy_static;

mod alsa;

lazy_static! {
	static ref CONTEXT: alsa::Context = {
		alsa::Context::new()
	};
}

fn set_settings(pcm: &alsa::pcm::PCM) {
	// Set hardware parameters: 44100 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(&CONTEXT, pcm).unwrap();
	hwp.set_channels(&CONTEXT, 1).unwrap();
	hwp.set_rate(&CONTEXT, 48000, alsa::ValueOr::Nearest).unwrap();
	let rate = hwp.get_rate(&CONTEXT).unwrap();
//	println!("RATE: {}", rate);
	assert_eq!(rate, 48_000);
	hwp.set_format(&CONTEXT, alsa::pcm::Format::s16()).unwrap();
	hwp.set_access(&CONTEXT, alsa::pcm::Access::RWInterleaved).unwrap();
	pcm.hw_params(&CONTEXT, &hwp).unwrap();
	hwp.drop(&CONTEXT);
}

pub struct AudioManager {
//	#[cfg(feature = "speaker")]
	speaker: (i64, alsa::pcm::PCM), // TODO: call drop(), it isn't being called rn.
//	#[cfg(feature = "speaker")]
	speaker_buffer: Vec<i16>,
//	#[cfg(feature = "microphone")]
	microphone: alsa::pcm::PCM, // TODO: call drop(), it isn't being called rn.
}

impl AudioManager {
	/// Create a new `AudioManager`.
	pub fn new() -> Self {
//		#[cfg(feature = "speaker")]
		let (speaker, speaker_buffer) = {
			let pcm = alsa::pcm::PCM::new(&CONTEXT, "default",
				alsa::Direction::Playback).unwrap();
			set_settings(&pcm);
			let mut speaker_max_latency;
			(({
				let hwp = pcm.hw_params_current(&CONTEXT).unwrap();
				let bs = hwp.get_buffer_size(&CONTEXT).unwrap();

				println!("Buffer Size: {}", bs);
				speaker_max_latency
					= hwp.get_period_size(&CONTEXT).unwrap()
						as usize * 2;

				println!("PC: {}", hwp.get_channels(&CONTEXT).unwrap());
				println!("PR: {}", hwp.get_rate(&CONTEXT).unwrap());

				hwp.drop(&CONTEXT);
				bs
			}, pcm), vec![0i16; speaker_max_latency])
		};

//		#[cfg(feature = "microphone")]
		let microphone = {
			let pcm = alsa::pcm::PCM::new(&CONTEXT, "plughw:0,0",
				alsa::Direction::Capture).unwrap();
			set_settings(&pcm);
			{
				let hwp = pcm.hw_params_current(&CONTEXT).unwrap();
				println!("CC: {}", hwp.get_channels(&CONTEXT).unwrap());
				println!("CR: {}", hwp.get_rate(&CONTEXT).unwrap());
				hwp.drop(&CONTEXT);
			}
			pcm
		};

//		#[cfg(feature = "speaker")]
		{
			speaker.1.prepare(&CONTEXT);
		}

//		#[cfg(feature = "microphone")]
		{
			microphone.start(&CONTEXT);
		}

		let am = AudioManager {
//			#[cfg(feature = "speaker")]
			speaker,
//			#[cfg(feature = "speaker")]
			speaker_buffer,
//			#[cfg(feature = "microphone")]
			microphone,
		};

		am
	}

//	#[cfg(feature = "speaker")]
	/// Push data to the speaker output.
	fn push(&self, buffer: &[i16]) {
		if self.speaker.1.writei(&CONTEXT, buffer).unwrap_or_else(|_| {
			0
		}) != buffer.len()
		{
			println!("buffer underrun!");

			self.speaker.1.recover(&CONTEXT, 32, true).unwrap_or_else(|x| {
				panic!("ERROR: {}", x)
			});

			if self.speaker.1.writei(&CONTEXT, buffer).unwrap_or_else(|_| {
				0
			}) != buffer.len() {
				panic!("double buffer underrun!");
			}
		}
	}

	/// Generate & push data to speaker output.  When a new sample is
	/// needed, closure `generator` will be called.  This should be called
	/// in a loop.
//	#[cfg(feature = "speaker")]
	pub fn play(&mut self, generator: &mut FnMut() -> i16) {
		let left = self.left() as usize;
		let write = if left < self.speaker_buffer.len() {
			self.speaker_buffer.len() - left
		} else { 0 };

		for i in 0..write {
			self.speaker_buffer[i] = generator();
		}

		self.push(&self.speaker_buffer[..write]);
	}

//	#[cfg(feature = "microphone")]
	/// Pull data from the microphone input.
	pub fn pull(&self, buffer: &mut [i16]) -> usize {
		self.microphone.readi(&CONTEXT, buffer).unwrap_or(0)
	}

	/// Get the number of samples left in the buffer.
//	#[cfg(feature = "speaker")]
	fn left(&self) -> i64 {
		self.speaker.0 - self.speaker.1.status(&CONTEXT).unwrap().get_avail(&CONTEXT)
	}
}
