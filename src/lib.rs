// Copyright Jeron Lau 2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

//! Play sound through speakers, earbuds or headphones (platform-agnostic).

extern crate libc;
extern crate nix;
#[macro_use]
extern crate lazy_static;

mod alsa;

const HZ_48K: u32 = 48_000;

lazy_static! {
	static ref CONTEXT: alsa::Context = {
		alsa::Context::new()
	};
}

fn set_settings(pcm: &alsa::pcm::PCM, stereo: bool) {
	// Set hardware parameters: 48000 Hz / Mono / 16 bit
	let hwp = alsa::pcm::HwParams::any(&CONTEXT, pcm).unwrap();
	hwp.set_channels(&CONTEXT, if stereo { 2 } else { 1 }).unwrap();
	hwp.set_rate(&CONTEXT, HZ_48K, alsa::ValueOr::Nearest).unwrap();
	let rate = hwp.get_rate(&CONTEXT).unwrap();
	assert_eq!(rate, HZ_48K);
	hwp.set_format(&CONTEXT, {
		if cfg!(target_endian = "little") { 2 }
		else if cfg!(target_endian = "big") { 3 }
		else { unreachable!() }
	}).unwrap();
	hwp.set_access(&CONTEXT, alsa::pcm::Access::RWInterleaved).unwrap();
	pcm.hw_params(&CONTEXT, &hwp).unwrap();
	hwp.drop(&CONTEXT);
}

pub struct Speaker {
	speaker: (i64, alsa::pcm::PCM),
	speaker_buffer: Vec<i16>,
}

impl Speaker {
	/// Connect to a new Speaker.
	pub fn new(speaker: u16, stereo: bool) -> Option<Self> {
		if speaker != 0 { return None }

		let (speaker, speaker_buffer) = {
			let pcm = alsa::pcm::PCM::new(&CONTEXT, "default",
				alsa::Direction::Playback).unwrap();
			set_settings(&pcm, stereo);
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

		speaker.1.prepare(&CONTEXT);

		Some(Self { speaker, speaker_buffer })
	}

	/// Get the number of connected speakers.
	pub fn num(&self) -> u16 {
		1
	}

	/// Generate & push data to speaker output.  When a new sample is
	/// needed, closure `generator` will be called.  This should be called
	/// in a loop.
	pub fn update(&mut self, generator: &mut FnMut() -> i16) {
		let left = self.left() as usize;
		let write = if left < self.speaker_buffer.len() {
			self.speaker_buffer.len() - left
		} else { 0 };

		for i in 0..write {
			self.speaker_buffer[i] = generator();
		}

		self.push(&self.speaker_buffer[..write]);
	}

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

	/// Get the number of samples left in the buffer.
	fn left(&self) -> i64 {
		self.speaker.0 - self.speaker.1.status(&CONTEXT).unwrap().get_avail(&CONTEXT)
	}
}

impl Drop for Speaker {
	fn drop(&mut self) {
		self.speaker.1.drop(&CONTEXT);
	}
}

pub struct Microphone {
	pcm: alsa::pcm::PCM
}

impl Microphone {
	/// Create a new Microphone object.
	pub fn new(microphone: u16, stereo: bool) -> Option<Self> {
		if microphone != 0 { return None }

		let pcm = alsa::pcm::PCM::new(&CONTEXT, "plughw:0,0",
			alsa::Direction::Capture).unwrap();
		set_settings(&pcm, stereo);
		{
			let hwp = pcm.hw_params_current(&CONTEXT).unwrap();
			println!("CC: {}", hwp.get_channels(&CONTEXT).unwrap());
			println!("CR: {}", hwp.get_rate(&CONTEXT).unwrap());
			hwp.drop(&CONTEXT);
		}

		pcm.start(&CONTEXT);

		Some(Self { pcm })
	}

	/// Get the number of connected microphones.
	pub fn num(&self) -> u16 {
		1
	}

	/// Pull data from the microphone input.
	pub fn update(&self, buffer: &mut [i16]) -> usize {
		self.pcm.readi(&CONTEXT, buffer).unwrap_or(0)
	}
}

impl Drop for Microphone {
	fn drop(&mut self) {
		self.pcm.drop(&CONTEXT);
	}
}
