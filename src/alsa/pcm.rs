//! Audio playback and capture
//!
//! # Example
//! Playback a sine wave through the "default" device.
//!
//! ```
//! use alsa::{Direction, ValueOr};
//! use alsa::pcm::{PCM, HwParams, Format, Access, State};
//!
//! // Open default playback device
//! let pcm = PCM::new("default", Direction::Playback, false).unwrap();
//!
//! // Set hardware parameters: 44100 Hz / Mono / 16 bit
//! let hwp = HwParams::any(&pcm).unwrap();
//! hwp.set_channels(1).unwrap();
//! hwp.set_rate(44100, ValueOr::Nearest).unwrap();
//! hwp.set_format(Format::s16()).unwrap();
//! hwp.set_access(Access::RWInterleaved).unwrap();
//! pcm.hw_params(&hwp).unwrap();
//! let io = pcm.io_i16().unwrap();
//!
//! // Make sure we don't start the stream too early
//! let hwp = pcm.hw_params_current().unwrap();
//! let swp = pcm.sw_params_current().unwrap();
//! swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
//! pcm.sw_params(&swp).unwrap();
//!
//! // Make a sine wave
//! let mut buf = [0i16; 1024];
//! for (i, a) in buf.iter_mut().enumerate() {
//!	 *a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
//! }
//!
//! // Play it back for 2 seconds.
//! for _ in 0..2*44100/1024 {
//!	 assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
//! }
//!
//! // In case the buffer was larger than 2 seconds, start the stream manually.
//! if pcm.state() != State::Running { pcm.start().unwrap() };
//! // Wait for the stream to finish playback.
//! pcm.drain().unwrap();
//! ```


use libc::{c_int, c_uint, c_void, ssize_t};
use alsa::alsa;
use alsa::alsa::Context;
use std::ffi::{CString};
use std::{ptr};
use super::error::*;
use super::{Direction, ValueOr};

/// [snd_pcm_sframes_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html)
pub type Frames = alsa::snd_pcm_sframes_t;

/// [snd_pcm_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) wrapper - start here for audio playback and recording
pub struct PCM(*mut alsa::snd_pcm_t);

unsafe impl Send for PCM {}

impl PCM {
	/// Wrapper around open that takes a &str instead of a &CStr
	pub fn new(context: &Context, name: &str, dir: Direction/*, nonblock: bool*/) -> Result<PCM> {
		let name = CString::new(name).unwrap();
		let mut r = ptr::null_mut();
		let stream = match dir {
			Direction::Capture => alsa::SND_PCM_STREAM_CAPTURE,
			Direction::Playback => alsa::SND_PCM_STREAM_PLAYBACK
		};
		let flags = match dir {
			Direction::Capture => 2,
			Direction::Playback => 1,
		};/*if nonblock { *//* } else { 0;*/
		acheck!(context, snd_pcm_open(&mut r, name.as_ptr(), stream, flags)).map(|_| PCM(r))
	}

	pub fn prepare(&self, context: &Context) {
		unsafe {
			(context.snd_pcm_prepare)(self.0);
		}
	}

	pub fn start(&self, context: &Context) {
		unsafe {
			(context.snd_pcm_start)(self.0);
		}
	}

	pub fn recover(&self, context: &Context, err: c_int, silent: bool) -> Result<()> {
		acheck!(context, snd_pcm_recover(self.0, err, if silent { 1 } else { 0 })).map(|_| ()) }

	pub fn bytes_to_frames(&self, context: &Context, i: isize) -> Frames { unsafe { (context.snd_pcm_bytes_to_frames)(self.0, i as ssize_t) }}
	pub fn frames_to_bytes(&self, context: &Context, i: Frames) -> isize { unsafe { (context.snd_pcm_frames_to_bytes)(self.0, i) as isize }}

	pub fn status(&self, context: &Context) -> Result<Status> {
		let z = Status::new(context);
		acheck!(context, snd_pcm_status(self.0, z.ptr())).map(|_| z)
	}

	fn verify_format(&self, f: Format, context: &Context) -> Result<()> {
		let ff = try!(self.hw_params_current(context).and_then(|h| h.get_format(context)));
		if ff == f { Ok(()) }
		else {
			// let s = format!("Invalid sample format ({:?}, expected {:?})", ff, f);
			Err(Error::unsupported("io_xx"))
		}
	}

	pub fn hw_params(&self, context: &Context, h: &HwParams) -> Result<()> {
		acheck!(context, snd_pcm_hw_params(self.0, h.0)).map(|_| ())
	}

	pub fn hw_params_current<'a>(&'a self, context: &Context) -> Result<HwParams<'a>> {
		HwParams::new(context, &self).and_then(|h|
			acheck!(context, snd_pcm_hw_params_current(self.0, h.0)).map(|_| h))
	}

	pub fn sw_params(&self, context: &Context, h: &SwParams) -> Result<()> {
		acheck!(context, snd_pcm_sw_params(self.0, h.0)).map(|_| ())
	}

	pub fn sw_params_current<'a>(&'a self, context: &Context) -> Result<SwParams<'a>> {
		SwParams::new(context, &self).and_then(|h|
			acheck!(context, snd_pcm_sw_params_current(self.0, h.0)).map(|_| h))
	}

	pub fn drop(&self, context: &Context) {
		unsafe { (context.snd_pcm_close)(self.0) };
	}

	/// Get the number of *frames* available.
	pub fn avail(&self, context: &Context) -> Frames {
		unsafe {
			(context.snd_pcm_avail)(self.0) as Frames
		}
	}

	/// On success, returns number of *frames* written.
	/// (Multiply with number of channels to get number of items in buf successfully written.)
	pub fn writei(&self, context: &Context, buf: &[i16]) -> Result<usize> {
		let nsamples = buf.len() as alsa::snd_pcm_uframes_t;

		acheck!(context, snd_pcm_writei(self.0, buf.as_ptr() as *const c_void, nsamples)).map(|r| r as usize)
	}

	/// On success, returns number of *frames* read.
	/// (Multiply with number of channels to get number of items in buf successfully read.)
	pub fn readi(&self, context: &Context, buf: &mut [i16]) -> Result<usize> {
		let nsamples = buf.len() as alsa::snd_pcm_uframes_t;

		acheck!(context, snd_pcm_readi(self.0, buf.as_mut_ptr() as *mut c_void, nsamples)).map(|r| r as usize)
	}
}

alsa_enum!(
	/// [SND_PCM_STATE_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
	State, ALL_STATES[9],

	Open = SND_PCM_STATE_OPEN,
	Setup = SND_PCM_STATE_SETUP,
	Prepared = SND_PCM_STATE_PREPARED,
	Running = SND_PCM_STATE_RUNNING,
	XRun = SND_PCM_STATE_XRUN,
	Draining = SND_PCM_STATE_DRAINING,
	Paused = SND_PCM_STATE_PAUSED,
	Suspended = SND_PCM_STATE_SUSPENDED,
	Disconnected = SND_PCM_STATE_DISCONNECTED,
);

alsa_enum!(
	/// [SND_PCM_FORMAT_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
	Format, ALL_FORMATS[45],

	Unknown = SND_PCM_FORMAT_UNKNOWN,
	S8 = SND_PCM_FORMAT_S8,
	U8 = SND_PCM_FORMAT_U8,
	S16LE = SND_PCM_FORMAT_S16_LE,
	S16BE = SND_PCM_FORMAT_S16_BE,
	U16LE = SND_PCM_FORMAT_U16_LE,
	U16BE = SND_PCM_FORMAT_U16_BE,
	S24LE = SND_PCM_FORMAT_S24_LE,
	S24BE = SND_PCM_FORMAT_S24_BE,
	U24LE = SND_PCM_FORMAT_U24_LE,
	U24BE = SND_PCM_FORMAT_U24_BE,
	S32LE = SND_PCM_FORMAT_S32_LE,
	S32BE = SND_PCM_FORMAT_S32_BE,
	U32LE = SND_PCM_FORMAT_U32_LE,
	U32BE = SND_PCM_FORMAT_U32_BE,
	FloatLE = SND_PCM_FORMAT_FLOAT_LE,
	FloatBE = SND_PCM_FORMAT_FLOAT_BE,
	Float64LE = SND_PCM_FORMAT_FLOAT64_LE,
	Float64BE = SND_PCM_FORMAT_FLOAT64_BE,
	IEC958SubframeLE = SND_PCM_FORMAT_IEC958_SUBFRAME_LE,
	IEC958SubframeBE = SND_PCM_FORMAT_IEC958_SUBFRAME_BE,
	MuLaw = SND_PCM_FORMAT_MU_LAW,
	ALaw = SND_PCM_FORMAT_A_LAW,
	ImaAdPCM = SND_PCM_FORMAT_IMA_ADPCM,
	MPEG = SND_PCM_FORMAT_MPEG,
	GSM = SND_PCM_FORMAT_GSM,
	Special = SND_PCM_FORMAT_SPECIAL,
	S243LE = SND_PCM_FORMAT_S24_3LE,
	S243BE = SND_PCM_FORMAT_S24_3BE,
	U243LE = SND_PCM_FORMAT_U24_3LE,
	U243BE = SND_PCM_FORMAT_U24_3BE,
	S203LE = SND_PCM_FORMAT_S20_3LE,
	S203BE = SND_PCM_FORMAT_S20_3BE,
	U203LE = SND_PCM_FORMAT_U20_3LE,
	U203BE = SND_PCM_FORMAT_U20_3BE,
	S183LE = SND_PCM_FORMAT_S18_3LE,
	S183BE = SND_PCM_FORMAT_S18_3BE,
	U183LE = SND_PCM_FORMAT_U18_3LE,
	U183BE = SND_PCM_FORMAT_U18_3BE,
	G72324 = SND_PCM_FORMAT_G723_24,
	G723241B = SND_PCM_FORMAT_G723_24_1B,
	G72340 = SND_PCM_FORMAT_G723_40,
	G723401B = SND_PCM_FORMAT_G723_40_1B,
	DSDU8 = SND_PCM_FORMAT_DSD_U8,
	DSDU16LE = SND_PCM_FORMAT_DSD_U16_LE,
//	DSDU32LE = SND_PCM_FORMAT_DSD_U32_LE,
//	DSDU16BE = SND_PCM_FORMAT_DSD_U16_BE,
//	DSDU32BE = SND_PCM_FORMAT_DSD_U32_BE,
);

impl Format {
	#[cfg(target_endian = "little")] pub fn s16() -> Format { Format::S16LE }
	#[cfg(target_endian = "big")] pub fn s16() -> Format { Format::S16BE }
}

alsa_enum!(
	/// [SND_PCM_ACCESS_xxx](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m.html) constants
	Access, ALL_ACCESSES[5],

	MMapInterleaved = SND_PCM_ACCESS_MMAP_INTERLEAVED,
	MMapNonInterleaved = SND_PCM_ACCESS_MMAP_NONINTERLEAVED,
	MMapComplex = SND_PCM_ACCESS_MMAP_COMPLEX,
	RWInterleaved = SND_PCM_ACCESS_RW_INTERLEAVED,
	RWNonInterleaved = SND_PCM_ACCESS_RW_NONINTERLEAVED,
);

/// [snd_pcm_hw_params_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___h_w___params.html) wrapper
pub struct HwParams<'a>(*mut alsa::snd_pcm_hw_params_t, &'a PCM);

impl<'a> HwParams<'a> {
	fn new(context: &Context, a: &'a PCM) -> Result<HwParams<'a>> {
		let mut p = ptr::null_mut();
		acheck!(context, snd_pcm_hw_params_malloc(&mut p)).map(|_| HwParams(p, a))
	}

	pub fn any(context: &Context, a: &'a PCM) -> Result<HwParams<'a>> {
		HwParams::new(context, a).and_then(|p|
			acheck!(context, snd_pcm_hw_params_any(a.0, p.0)).map(|_| p)
		)
	}

	pub fn set_channels(&self, context: &Context, v: u32) -> Result<()> {
		acheck!(context, snd_pcm_hw_params_set_channels((self.1).0, self.0, v as c_uint)).map(|_| ())
	}

	pub fn get_channels(&self, context: &Context) -> Result<u32> {
		let mut v = 0;
		acheck!(context, snd_pcm_hw_params_get_channels(self.0, &mut v)).map(|_| v as u32)
	}

	pub fn set_rate(&self, context: &Context, v: u32, dir: ValueOr) -> Result<()> {
		acheck!(context, snd_pcm_hw_params_set_rate((self.1).0, self.0, v as c_uint, dir as c_int)).map(|_| ())
	}

	pub fn get_rate(&self, context: &Context) -> Result<u32> {
		let (mut v, mut d) = (0,0);
		acheck!(context, snd_pcm_hw_params_get_rate(self.0, &mut v, &mut d)).map(|_| v as u32)
	}

	pub fn set_format(&self, context: &Context, v: Format) -> Result<()> {
		acheck!(context, snd_pcm_hw_params_set_format((self.1).0, self.0, v as c_int)).map(|_| ())
	}

	pub fn get_format(&self, context: &Context) -> Result<Format> {
		let mut v = 0;
		acheck!(context, snd_pcm_hw_params_get_format(self.0, &mut v))
			.and_then(|_| Format::from_c_int(v, "snd_pcm_hw_params_get_format"))
	}

	pub fn set_access(&self, context: &Context, v: Access) -> Result<()> {
		acheck!(context, snd_pcm_hw_params_set_access((self.1).0, self.0, v as c_uint)).map(|_| ())
	}

	pub fn get_access(&self, context: &Context) -> Result<Access> {
		let mut v = 0;
		acheck!(context, snd_pcm_hw_params_get_access(self.0, &mut v))
			.and_then(|_| Access::from_c_int(v as c_int, "snd_pcm_hw_params_get_access"))
	}

	pub fn get_period_size(&self, context: &Context) -> Result<Frames> {
		let (mut v, mut d) = (0,0);
		acheck!(context, snd_pcm_hw_params_get_period_size(self.0, &mut v, &mut d)).map(|_| v as Frames)
	}

//	pub fn set_period_size(&self, context: &Context, v: Frames) -> Result<()> {//
//		acheck!(context, snd_pcm_hw_params_set_period_size((self.1).0, self.0, v as c_uint, 0)).map(|_| ())
//	}

	pub fn get_buffer_size(&self, context: &Context) -> Result<Frames> {
		let mut v = 0;
		acheck!(context, snd_pcm_hw_params_get_buffer_size(self.0, &mut v)).map(|_| v as Frames)
	}

	pub fn copy_from(&mut self, context: &Context, other: &HwParams<'a>) {
		self.1 = other.1;
		unsafe { (context.snd_pcm_hw_params_copy)(self.0, other.0) };
	}

	pub fn drop(&self, context: &Context) {
		unsafe { (context.snd_pcm_hw_params_free)(self.0) };
	}
}

/*
impl<'a> Clone for HwParams<'a> {
	fn clone(&self) -> HwParams<'a> {
		let mut r = HwParams::new(self.1).unwrap();
		r.copy_from(&self);
		r
	}
}*/

/*impl<'a> fmt::Debug for HwParams<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		   "HwParams(channels: {:?}, rate: {:?} Hz, format: {:?}, access: {:?}, period size: {:?} frames, buffer size: {:?} frames)",
		   self.get_channels(), self.get_rate(), self.get_format(), self.get_access(), self.get_period_size(), self.get_buffer_size())
	}
}*/

/// [snd_pcm_sw_params_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___s_w___params.html) wrapper
pub struct SwParams<'a>(*mut alsa::snd_pcm_sw_params_t, &'a PCM);

impl<'a> SwParams<'a> {
	fn new(context: &Context, a: &'a PCM) -> Result<SwParams<'a>> {
		let mut p = ptr::null_mut();
		acheck!(context, snd_pcm_sw_params_malloc(&mut p)).map(|_| SwParams(p, a))
	}

	pub fn get_avail_min(&self, context: &Context) -> Result<Frames> {
		let mut v = 0;
		acheck!(context, snd_pcm_sw_params_get_avail_min(self.0, &mut v)).map(|_| v as Frames)
	}

	pub fn set_start_threshold(&self, context: &Context, v: Frames) -> Result<()> {
		acheck!(context, snd_pcm_sw_params_set_start_threshold((self.1).0, self.0, v as alsa::snd_pcm_uframes_t)).map(|_| ())
	}

	pub fn get_start_threshold(&self, context: &Context) -> Result<Frames> {
		let mut v = 0;
		acheck!(context, snd_pcm_sw_params_get_start_threshold(self.0, &mut v)).map(|_| v as Frames)
	}

	pub fn get_stop_threshold(&self, context: &Context) -> Result<Frames> {
		let mut v = 0;
		acheck!(context, snd_pcm_sw_params_get_stop_threshold(self.0, &mut v)).map(|_| v as Frames)
	}

	pub fn drop(&self, context: &Context) {
		unsafe { (context.snd_pcm_sw_params_free)(self.0) };
	}
}

/*impl<'a> fmt::Debug for SwParams<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,
		   "SwParams(avail_min: {:?} frames, start_threshold: {:?} frames, stop_threshold: {:?} frames)",
		   self.get_avail_min(), self.get_start_threshold(), self.get_stop_threshold())
	}
}*/

const STATUS_SIZE: usize = 152;

/// [snd_pcm_status_t](http://www.alsa-project.org/alsa-doc/alsa-lib/group___p_c_m___status.html) wrapper
pub struct Status([u8; STATUS_SIZE]);

impl Status {
	fn new(context: &Context) -> Status {
		assert!(unsafe { (context.snd_pcm_status_sizeof)() } as usize <= STATUS_SIZE);
		Status([0; STATUS_SIZE])
	}

	fn ptr(&self) -> *mut alsa::snd_pcm_status_t {
		self.0.as_ptr() as *const _ as *mut alsa::snd_pcm_status_t
	}

	pub fn get_avail(&self, context: &Context) -> Frames {
		unsafe {
			(context.snd_pcm_status_get_avail)(self.ptr()) as Frames
		}
	}
}

#[test]
fn info_from_default() {
	use std::ffi::CString;
	let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
	let info = pcm.info().unwrap();
	println!("PCM Info:");
	println!("\tCard: {}", info.get_card());
	println!("\tDevice: {}", info.get_device());
	println!("\tSubdevice: {}", info.get_subdevice());
	println!("\tId: {}", info.get_id().unwrap());
	println!("\tName: {}", info.get_name().unwrap());
	println!("\tSubdevice Name: {}", info.get_subdevice_name().unwrap());
}

#[test]
fn drop() {
	use std::ffi::CString;
	let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
	// Verify that this does not cause a naming conflict (issue #14)
	let _ = pcm.drop();
}

#[test]
fn record_from_default() {
	use std::ffi::CString;
	let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Capture, false).unwrap();
	let hwp = HwParams::any(&pcm).unwrap();
	hwp.set_channels(2).unwrap();
	hwp.set_rate(44100, ValueOr::Nearest).unwrap();
	hwp.set_format(Format::s16()).unwrap();
	hwp.set_access(Access::RWInterleaved).unwrap();
	pcm.hw_params(&hwp).unwrap();
	pcm.start().unwrap();
	let mut buf = [0i16; 1024];
	assert_eq!(pcm.io_i16().unwrap().readi(&mut buf).unwrap(), 1024/2);
}

#[test]
fn playback_to_default() {
	use std::ffi::CString;
	let pcm = PCM::open(&*CString::new("default").unwrap(), Direction::Playback, false).unwrap();
	let hwp = HwParams::any(&pcm).unwrap();
	hwp.set_channels(1).unwrap();
	hwp.set_rate(44100, ValueOr::Nearest).unwrap();
	hwp.set_format(Format::s16()).unwrap();
	hwp.set_access(Access::RWInterleaved).unwrap();
	pcm.hw_params(&hwp).unwrap();

	let hwp = pcm.hw_params_current().unwrap();
	let swp = pcm.sw_params_current().unwrap();
	swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
	pcm.sw_params(&swp).unwrap();

	println!("PCM status: {:?}, {:?}", pcm.state(), pcm.hw_params_current().unwrap());

	let mut buf = [0i16; 1024];
	for (i, a) in buf.iter_mut().enumerate() {
		*a = ((i as f32 * 2.0 * ::std::f32::consts::PI / 128.0).sin() * 8192.0) as i16
	}
	let io = pcm.io_i16().unwrap();
	for _ in 0..2*44100/1024 { // 2 seconds of playback
		println!("PCM state: {:?}", pcm.state());
		assert_eq!(io.writei(&buf[..]).unwrap(), 1024);
	}
	if pcm.state() != State::Running { pcm.start().unwrap() };

	pcm.drain().unwrap();
}
