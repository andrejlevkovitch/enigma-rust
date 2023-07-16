use std::error;
use std::fmt;

pub const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const SEGMENTS: &str = ALPHABET;

// for rotors
pub const RING_I: &str = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
pub const RING_II: &str = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
pub const RING_III: &str = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
pub const RING_IV: &str = "ESOVPZJAYQUIRHXLNFTGKDCMWB";
pub const RING_V: &str = "VZBRGITYUPSDNHLXAWMJQOFECK";
pub const RING_VI: &str = "JPGVOUMFYQBENHZRDKASXLICTW";
pub const RING_VII: &str = "NZJHGRCXMYSWBOUFAIVLPEKQDT";
pub const RING_VIII: &str = "FKQHTLXOCBJSPDZRAMEWNIUYGV";
// for reflectors
pub const RING_A: &str = "EJMZALYXVBWFCRQUONTSPIKHGD";
pub const RING_B: &str = "YRUHQSLDPXNGOKMIEBFZCWVJAT";
pub const RING_C: &str = "FVPJIAOYEDRZXWGCTKUQSBNMHL";

pub const NOTCH_I: &str = "Q";
pub const NOTCH_II: &str = "E";
pub const NOTCH_III: &str = "V";
pub const NOTCH_IV: &str = "J";
pub const NOTCH_V: &str = "Z";
pub const NOTCH_VI: &str = "ZM";
pub const NOTCH_VII: &str = "ZM";
pub const NOTCH_VIII: &str = "ZM";

#[derive(Debug, Clone)]
pub enum SegmentError {
    InvalidRingSize(/*actual*/ usize, /*expected*/ usize),
    MissedRingSegment(/*missed*/ char),
    InvalidNotch(/*notch*/ char),
}

impl fmt::Display for SegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SegmentError::InvalidRingSize(actual, expected) => write!(
                f,
                "invalid ring size: {}/{} (actual/expected)",
                actual, expected
            ),
            SegmentError::MissedRingSegment(missed) => write!(f, "missed ring segment: {}", missed),
            SegmentError::InvalidNotch(notch) => write!(f, "invalid notch: {}", notch),
        }
    }
}

impl error::Error for SegmentError {}

pub fn check_ring(outputs: &String) -> Result<(), SegmentError> {
    if SEGMENTS.len() != outputs.len() {
        return Err(SegmentError::InvalidRingSize(outputs.len(), SEGMENTS.len()));
    }

    for val in SEGMENTS.chars() {
        if outputs.contains(val) == false {
            return Err(SegmentError::MissedRingSegment(val));
        }
    }

    return Ok(());
}

pub fn check_notches(notches: &String) -> Result<(), SegmentError> {
    for notch in notches.chars() {
        if SEGMENTS.contains(notch) == false {
            return Err(SegmentError::InvalidNotch(notch));
        }
    }

    return Ok(());
}
