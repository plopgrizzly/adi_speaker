/// Replaces constants ending with PLAYBACK/CAPTURE as well as
/// INPUT/OUTPUT
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Direction {
	Playback,
	Capture
}

/// Used to restrict hw parameters. In case the submitted
/// value is unavailable, in which direction should one search
/// for available values?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ValueOr {
	/// The value set is the submitted value, or the nearest
	Nearest = 0,
}

mod error;

pub mod pcm;

mod alsa;
pub(crate) use self::alsa::Context;
