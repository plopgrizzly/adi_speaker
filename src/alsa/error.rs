#![macro_use]

use libc::{c_int};
use std::{fmt, str};
use nix;
use std::error::Error as StdError;

/// ALSA error
///
/// Most ALSA functions can return a negative error code.
/// If so, then that error code is wrapped into this `Error` struct.
/// An Error is also returned in case ALSA returns a string that
/// cannot be translated into Rust's UTF-8 strings.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Error(&'static str, nix::Error);

pub type Result<T> = ::std::result::Result<T, Error>;

macro_rules! acheck {
    ($context: expr, $f: ident ( $($x: expr),* ) ) => {{
        let r = unsafe { ($context.$f)( $($x),* ) };
        if r < 0 { Err(Error::new(stringify!($f), -r as ::libc::c_int)) }
        else { Ok(r) }
    }}
}

impl Error {
    pub fn new(func: &'static str, res: c_int) -> Error { 
        let errno = nix::Errno::from_i32(res as i32);
        Error(func, nix::Error::from_errno(errno))
    }

    pub fn unsupported(func: &'static str) -> Error { 
        Error(func, nix::Error::UnsupportedOperation)
    }

    /// The function which failed.
    pub fn func(&self) -> &'static str { self.0 }

    /// The errno, if any. 
    pub fn errno(&self) -> Option<nix::Errno> { if let nix::Error::Sys(x) = self.1 { Some(x) } else { None } }

    /// Underlying error
    pub fn nix_error(&self) -> nix::Error { self.1 }
}

impl StdError for Error {
    fn description(&self) -> &str { "ALSA error" }
    fn cause(&self) -> Option<&StdError> { Some(&self.1) }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ALSA function '{}' failed with error '{}'", self.0, self.1)
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error { fmt::Error }
}


#[test]
fn broken_pcm_name() {
    use std::ffi::CString;
    let e = ::PCM::open(&*CString::new("this_PCM_does_not_exist").unwrap(), ::Direction::Playback, false).err().unwrap();
    assert_eq!(e.func(), "snd_pcm_open");
    assert_eq!(e.errno().unwrap(), nix::Errno::ENOENT);
}
