#![allow(non_camel_case_types)]

use libc;
use libc::{ c_ulong, c_long, c_int, c_uint, size_t, c_void, ssize_t, c_char };

pub enum snd_pcm_status_t { }
pub enum snd_pcm_sw_params_t { }
pub enum snd_pcm_t { }
pub enum snd_pcm_hw_params_t { }

pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = c_long;
pub type snd_pcm_format_t = c_int;
pub type snd_pcm_access_t = c_uint;
pub type snd_pcm_stream_t = c_uint;

pub const SND_PCM_ACCESS_MMAP_INTERLEAVED:    c_uint = 0;
pub const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: c_uint = 1;
pub const SND_PCM_ACCESS_MMAP_COMPLEX:        c_uint = 2;
pub const SND_PCM_ACCESS_RW_INTERLEAVED:      c_uint = 3;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED:   c_uint = 4;

pub const SND_PCM_FORMAT_UNKNOWN:            c_int = -1;
pub const SND_PCM_FORMAT_S8:                 c_int = 0;
pub const SND_PCM_FORMAT_U8:                 c_int = 1;
pub const SND_PCM_FORMAT_S16_LE:             c_int = 2;
pub const SND_PCM_FORMAT_S16_BE:             c_int = 3;
pub const SND_PCM_FORMAT_U16_LE:             c_int = 4;
pub const SND_PCM_FORMAT_U16_BE:             c_int = 5;
pub const SND_PCM_FORMAT_S24_LE:             c_int = 6;
pub const SND_PCM_FORMAT_S24_BE:             c_int = 7;
pub const SND_PCM_FORMAT_U24_LE:             c_int = 8;
pub const SND_PCM_FORMAT_U24_BE:             c_int = 9;
pub const SND_PCM_FORMAT_S32_LE:             c_int = 10;
pub const SND_PCM_FORMAT_S32_BE:             c_int = 11;
pub const SND_PCM_FORMAT_U32_LE:             c_int = 12;
pub const SND_PCM_FORMAT_U32_BE:             c_int = 13;
pub const SND_PCM_FORMAT_FLOAT_LE:           c_int = 14;
pub const SND_PCM_FORMAT_FLOAT_BE:           c_int = 15;
pub const SND_PCM_FORMAT_FLOAT64_LE:         c_int = 16;
pub const SND_PCM_FORMAT_FLOAT64_BE:         c_int = 17;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_LE: c_int = 18;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_BE: c_int = 19;
pub const SND_PCM_FORMAT_MU_LAW:             c_int = 20;
pub const SND_PCM_FORMAT_A_LAW:              c_int = 21;
pub const SND_PCM_FORMAT_IMA_ADPCM:          c_int = 22;
pub const SND_PCM_FORMAT_MPEG:               c_int = 23;
pub const SND_PCM_FORMAT_GSM:                c_int = 24;
pub const SND_PCM_FORMAT_SPECIAL:            c_int = 31;
pub const SND_PCM_FORMAT_S24_3LE:            c_int = 32;
pub const SND_PCM_FORMAT_S24_3BE:            c_int = 33;
pub const SND_PCM_FORMAT_U24_3LE:            c_int = 34;
pub const SND_PCM_FORMAT_U24_3BE:            c_int = 35;
pub const SND_PCM_FORMAT_S20_3LE:            c_int = 36;
pub const SND_PCM_FORMAT_S20_3BE:            c_int = 37;
pub const SND_PCM_FORMAT_U20_3LE:            c_int = 38;
pub const SND_PCM_FORMAT_U20_3BE:            c_int = 39;
pub const SND_PCM_FORMAT_S18_3LE:            c_int = 40;
pub const SND_PCM_FORMAT_S18_3BE:            c_int = 41;
pub const SND_PCM_FORMAT_U18_3LE:            c_int = 42;
pub const SND_PCM_FORMAT_U18_3BE:            c_int = 43;
pub const SND_PCM_FORMAT_G723_24:            c_int = 44;
pub const SND_PCM_FORMAT_G723_24_1B:         c_int = 45;
pub const SND_PCM_FORMAT_G723_40:            c_int = 46;
pub const SND_PCM_FORMAT_G723_40_1B:         c_int = 47;
pub const SND_PCM_FORMAT_DSD_U8:             c_int = 48;
pub const SND_PCM_FORMAT_DSD_U16_LE:         c_int = 49;

pub const SND_PCM_STATE_OPEN:         c_uint = 0;
pub const SND_PCM_STATE_SETUP:        c_uint = 1;
pub const SND_PCM_STATE_PREPARED:     c_uint = 2;
pub const SND_PCM_STATE_RUNNING:      c_uint = 3;
pub const SND_PCM_STATE_XRUN:         c_uint = 4;
pub const SND_PCM_STATE_DRAINING:     c_uint = 5;
pub const SND_PCM_STATE_PAUSED:       c_uint = 6;
pub const SND_PCM_STATE_SUSPENDED:    c_uint = 7;
pub const SND_PCM_STATE_DISCONNECTED: c_uint = 8;

pub const SND_PCM_STREAM_PLAYBACK: c_uint = 0;
pub const SND_PCM_STREAM_CAPTURE:  c_uint = 1;

pub static SND_PCM_ASYNC: c_int = 2;

pub struct Context {
	pub snd_pcm_status_get_avail: unsafe extern "C" fn(obj: *const snd_pcm_status_t) -> snd_pcm_uframes_t,
	pub snd_pcm_status_sizeof: unsafe extern "C" fn() -> size_t,
	pub snd_pcm_sw_params_set_start_threshold: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t, val: snd_pcm_uframes_t) -> c_int,
	pub snd_pcm_sw_params_get_start_threshold: unsafe extern "C" fn(paramsm: *const snd_pcm_sw_params_t, val: *mut snd_pcm_uframes_t) -> c_int,
	pub snd_pcm_sw_params_get_stop_threshold: unsafe extern "C" fn(params: *const snd_pcm_sw_params_t, val: *mut snd_pcm_uframes_t) -> c_int,
	pub snd_pcm_sw_params_get_avail_min: unsafe extern "C" fn(params: *const snd_pcm_sw_params_t, val: *mut snd_pcm_uframes_t) -> c_int,
	pub snd_pcm_hw_params_malloc: unsafe extern "C" fn(ptr: *mut *mut snd_pcm_hw_params_t) -> c_int,
	pub snd_pcm_hw_params_free: unsafe extern "C" fn(obj: *mut snd_pcm_hw_params_t),
	pub snd_pcm_sw_params_malloc: unsafe extern "C" fn(ptr: *mut *mut snd_pcm_sw_params_t) -> c_int,
	pub snd_pcm_sw_params_free: unsafe extern "C" fn(obj: *mut snd_pcm_sw_params_t),
	pub snd_pcm_hw_params_copy: unsafe extern "C" fn(dst: *mut snd_pcm_hw_params_t, src: *const snd_pcm_hw_params_t),
	pub snd_pcm_hw_params_get_buffer_size: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, val: *mut snd_pcm_uframes_t) -> c_int,
	pub snd_pcm_hw_params_get_period_size: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, frames: *mut snd_pcm_uframes_t, dir: *mut c_int) -> c_int,
	pub snd_pcm_hw_params_set_period_size: unsafe extern "C" fn(*mut snd_pcm_t, *mut snd_pcm_hw_params_t, snd_pcm_uframes_t, c_int) -> c_int,
	pub snd_pcm_hw_params_get_access: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, _access: *mut snd_pcm_access_t) -> c_int,
	pub snd_pcm_hw_params_get_format: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, val: *mut snd_pcm_format_t) -> c_int,
	pub snd_pcm_hw_params_set_access: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t, _access: snd_pcm_access_t) -> c_int,
	pub snd_pcm_hw_params_set_format: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t, val: snd_pcm_format_t) -> c_int,
	pub snd_pcm_hw_params_get_rate: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, val: *mut c_uint, dir: *mut c_int) -> c_int,
	pub snd_pcm_hw_params_set_rate: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t, val: c_uint, dir: c_int) -> c_int,
	pub snd_pcm_hw_params_get_channels: unsafe extern "C" fn(params: *const snd_pcm_hw_params_t, val: *mut c_uint) -> c_int,
	pub snd_pcm_hw_params_set_channels: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t, val: c_uint) -> c_int,
	pub snd_pcm_hw_params_any: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int,
	pub snd_pcm_writei: unsafe extern "C" fn(pcm: *mut snd_pcm_t, buffer: *const c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t,
	pub snd_pcm_readi: unsafe extern "C" fn(pcm: *mut snd_pcm_t, buffer: *mut c_void, size: snd_pcm_uframes_t) -> snd_pcm_sframes_t,
	pub snd_pcm_close: unsafe extern "C" fn(pcm: *mut snd_pcm_t) -> c_int,
	pub snd_pcm_sw_params_current: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int,
	pub snd_pcm_sw_params: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_sw_params_t) -> c_int,
	pub snd_pcm_hw_params_current: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int,
	pub snd_pcm_hw_params: unsafe extern "C" fn(pcm: *mut snd_pcm_t, params: *mut snd_pcm_hw_params_t) -> c_int,
	pub snd_pcm_status: unsafe extern "C" fn(pcm: *mut snd_pcm_t, status: *mut snd_pcm_status_t) -> c_int,
	pub snd_pcm_frames_to_bytes: unsafe extern "C" fn(pcm: *mut snd_pcm_t, frames: snd_pcm_sframes_t) -> ssize_t,
	pub snd_pcm_open: unsafe extern "C" fn(pcm: *mut *mut snd_pcm_t, name: *const c_char, stream: snd_pcm_stream_t, mode: c_int) -> c_int,
	pub snd_pcm_bytes_to_frames: unsafe extern "C" fn(pcm: *mut snd_pcm_t, bytes: ssize_t) -> snd_pcm_sframes_t,
	pub snd_pcm_recover: unsafe extern "C" fn(pcm: *mut snd_pcm_t, err: c_int, silent: c_int) -> c_int,
	pub snd_pcm_prepare: unsafe extern "C" fn(pcm: *mut snd_pcm_t) -> c_int,
	pub snd_pcm_start: unsafe extern "C" fn(pcm: *mut snd_pcm_t) -> c_int,
	pub snd_pcm_avail: unsafe extern "C" fn(pcm: *mut snd_pcm_t) -> snd_pcm_sframes_t,
}

fn dlsym<T>(lib: *mut c_void, name: &[u8]) -> T {
	unsafe {
		let fn_ptr = libc::dlsym(lib, &name[0] as *const _ as *const _);

		::std::mem::transmute_copy::<*mut c_void, T>(&fn_ptr)
	}
}

impl Context {
	pub fn new() -> Self {
		// TODO: dropping lib.
		let lib = b"libasound.so.2\0";
		let lib = unsafe {
			libc::dlopen(&lib[0] as *const _ as *const _, libc::RTLD_NOW | libc::RTLD_GLOBAL)
		};

		Context {
			snd_pcm_status_get_avail: dlsym(lib, b"snd_pcm_status_get_avail\0"),
			snd_pcm_status_sizeof: dlsym(lib, b"snd_pcm_status_sizeof\0"),
			snd_pcm_sw_params_set_start_threshold: dlsym(lib, b"snd_pcm_sw_params_set_start_threshold\0"),
			snd_pcm_sw_params_get_start_threshold: dlsym(lib, b"snd_pcm_sw_params_get_start_threshold\0"),
			snd_pcm_sw_params_get_stop_threshold: dlsym(lib, b"snd_pcm_sw_params_get_stop_threshold\0"),
			snd_pcm_sw_params_get_avail_min: dlsym(lib, b"snd_pcm_sw_params_get_avail_min\0"),
			snd_pcm_hw_params_malloc: dlsym(lib, b"snd_pcm_hw_params_malloc\0"),
			snd_pcm_hw_params_free: dlsym(lib, b"snd_pcm_hw_params_free\0"),
			snd_pcm_sw_params_malloc: dlsym(lib, b"snd_pcm_sw_params_malloc\0"),
			snd_pcm_sw_params_free: dlsym(lib, b"snd_pcm_sw_params_free\0"),
			snd_pcm_hw_params_copy: dlsym(lib, b"snd_pcm_hw_params_copy\0"),
			snd_pcm_hw_params_get_buffer_size: dlsym(lib, b"snd_pcm_hw_params_get_buffer_size\0"),
			snd_pcm_hw_params_get_period_size: dlsym(lib, b"snd_pcm_hw_params_get_period_size\0"),
			snd_pcm_hw_params_set_period_size: dlsym(lib, b"snd_pcm_hw_params_set_period_size\0"),
			snd_pcm_hw_params_get_access: dlsym(lib, b"snd_pcm_hw_params_get_access\0"),
			snd_pcm_hw_params_get_format: dlsym(lib, b"snd_pcm_hw_params_get_format\0"),
			snd_pcm_hw_params_set_access: dlsym(lib, b"snd_pcm_hw_params_set_access\0"),
			snd_pcm_hw_params_set_format: dlsym(lib, b"snd_pcm_hw_params_set_format\0"),
			snd_pcm_hw_params_get_rate: dlsym(lib, b"snd_pcm_hw_params_get_rate\0"),
			snd_pcm_hw_params_set_rate: dlsym(lib, b"snd_pcm_hw_params_set_rate\0"),
			snd_pcm_hw_params_get_channels: dlsym(lib, b"snd_pcm_hw_params_get_channels\0"),
			snd_pcm_hw_params_set_channels: dlsym(lib, b"snd_pcm_hw_params_set_channels\0"),
			snd_pcm_hw_params_any: dlsym(lib, b"snd_pcm_hw_params_any\0"),
			snd_pcm_writei: dlsym(lib, b"snd_pcm_writei\0"),
			snd_pcm_readi: dlsym(lib, b"snd_pcm_readi\0"),
			snd_pcm_close: dlsym(lib, b"snd_pcm_close\0"),
			snd_pcm_sw_params_current: dlsym(lib, b"snd_pcm_sw_params_current\0"),
			snd_pcm_sw_params: dlsym(lib, b"snd_pcm_sw_params\0"),
			snd_pcm_hw_params_current: dlsym(lib, b"snd_pcm_hw_params_current\0"),
			snd_pcm_hw_params: dlsym(lib, b"snd_pcm_hw_params\0"),
			snd_pcm_status: dlsym(lib, b"snd_pcm_status\0"),
			snd_pcm_frames_to_bytes: dlsym(lib, b"snd_pcm_frames_to_bytes\0"),
			snd_pcm_open: dlsym(lib, b"snd_pcm_open\0"),
			snd_pcm_bytes_to_frames: dlsym(lib, b"snd_pcm_bytes_to_frames\0"),
			snd_pcm_recover: dlsym(lib, b"snd_pcm_recover\0"),
			snd_pcm_prepare: dlsym(lib, b"snd_pcm_prepare\0"),
			snd_pcm_start: dlsym(lib, b"snd_pcm_start\0"),
			snd_pcm_avail: dlsym(lib, b"snd_pcm_avail\0"),
		}
	}
}
