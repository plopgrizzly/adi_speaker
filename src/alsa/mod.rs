//! Thin but safe wrappers for [ALSA](http://http://alsa-project.org).
//!
//! [Github repo](https://github.com/diwic/alsa-rs)
//!
//! [Crates.io](https://crates.io/crates/alsa)
//! 
//! This ALSA API wrapper/binding is WIP - the ALSA API is huge, and new
//! functions and structs might be added as requested.
//!
//! Most functions map 1-to-1 to alsa-lib functions, e g, `ctl::CardInfo::get_id()` is a wrapper around
//! `snd_ctl_card_info_get_id` and the [alsa-lib documentation](http://www.alsa-project.org/alsa-doc/alsa-lib/)
//! can be consulted for additional information. 
//!
//! Enjoy!

macro_rules! alsa_enum {
 ($(#[$attr:meta])+ $name:ident, $static_name:ident [$count:expr], $( $a:ident = $b:ident),* ,) =>
{
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
$(#[$attr])*
pub enum $name {
$(
    $a = alsa::$b as isize,
)*
}

static $static_name: [$name; $count] =
  [ $( $name::$a, )* ];

impl $name {
    /// Returns a slice of all possible values; useful for iteration
    pub fn all() -> &'static [$name] { &$static_name[..] }

    #[allow(dead_code)]
    fn from_c_int(c: ::libc::c_int, s: &'static str) -> Result<$name> {
        Self::all().iter().find(|&&x| c == x as ::libc::c_int).map(|&x| x)
            .ok_or_else(|| Error::unsupported(s))
    }
}

}
}

/// Replaces constants ending with PLAYBACK/CAPTURE as well as
/// INPUT/OUTPUT
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Playback,
    Capture
}

/// Used to restrict hw parameters. In case the submitted
/// value is unavailable, in which direction should one search
/// for available values?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueOr {
    /// The value set is the submitted value, or the nearest
    Nearest = 0,
}

mod error;
pub use self::error::{Error, Result};

pub mod pcm;
pub use self::pcm::PCM as PCM;

mod alsa;
pub use self::alsa::Context;
