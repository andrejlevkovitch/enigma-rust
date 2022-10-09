use crate::device::details::NOTCH_I;
use crate::device::details::NOTCH_II;
use crate::device::details::NOTCH_III;
use crate::device::details::NOTCH_IV;
use crate::device::details::NOTCH_V;
use crate::device::details::NOTCH_VI;
use crate::device::details::NOTCH_VII;
use crate::device::details::NOTCH_VIII;
use crate::device::details::RING_A;
use crate::device::details::RING_B;
use crate::device::details::RING_C;
use crate::device::details::RING_I;
use crate::device::details::RING_II;
use crate::device::details::RING_III;
use crate::device::details::RING_IV;
use crate::device::details::RING_V;
use crate::device::details::RING_VI;
use crate::device::details::RING_VII;
use crate::device::details::RING_VIII;
use crate::device::details::SEGMENTS;
use std::error;
use std::fmt;

// Rotor have 26 segments that represented by letters from A to Z. Right side
// of each segment connected with left side of other segment via wire. Also
// Rotor can have notches that turnover other ring on left side of the rotor
// when current rotor position at the notch.
// Reflector is a Rotor that can't rotate. Also it have connections only on
// left side, so it sends signals back to rotors.
pub struct Rotor {
    // left side is always static, A-Z
    ring: String,       // right side
    notches: String,    // if current segment have notch, then it turnover rotor from left
    position: usize,    // current segment
    ring_offset: usize, // from 0 to sements.len()
}

#[derive(Debug, Clone)]
pub enum RotorError {
    InvalidRingSize(/*actual*/ usize, /*expected*/ usize),
    MissedRingSegment(/*missed*/ char),
    InvalidNotch(/*notch*/ char),
    InvalidSegmentPosition(/*pos*/ char),
    InvalidRingOffset(/*pos*/ char),
}

impl fmt::Display for RotorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RotorError::InvalidRingSize(actual, expected) => write!(
                f,
                "invalid ring size: {}/{} (actual/expected)",
                actual, expected
            ),
            RotorError::MissedRingSegment(missed) => write!(f, "missed ring segment: {}", missed),
            RotorError::InvalidNotch(notch) => write!(f, "invalid notch: {}", notch),
            RotorError::InvalidSegmentPosition(pos) => {
                write!(f, "invalid segment position: {}", pos)
            }
            RotorError::InvalidRingOffset(pos) => {
                write!(f, "invalid ring offset: {}", pos)
            }
        }
    }
}

impl error::Error for RotorError {}

impl Rotor {
    fn check_ring(outputs: &String) -> Result<(), RotorError> {
        if SEGMENTS.len() != outputs.len() {
            return Err(RotorError::InvalidRingSize(outputs.len(), SEGMENTS.len()));
        }

        for val in SEGMENTS.chars() {
            if outputs.contains(val) == false {
                return Err(RotorError::MissedRingSegment(val));
            }
        }

        return Ok(());
    }

    fn check_notches(notches: &String) -> Result<(), RotorError> {
        for notch in notches.chars() {
            if SEGMENTS.contains(notch) == false {
                return Err(RotorError::InvalidNotch(notch));
            }
        }

        return Ok(());
    }

    pub fn new_static(outputs: &str) -> Result<Self, RotorError> {
        let s = outputs.to_string().to_ascii_uppercase();
        Rotor::check_ring(&s)?;

        Ok(Self {
            ring: s,
            notches: String::new(),
            position: 0,
            ring_offset: 0,
        })
    }

    pub fn new(outputs: &str, notches: &str) -> Result<Self, RotorError> {
        let so = outputs.to_string().to_ascii_uppercase();
        let sn = notches.to_string().to_ascii_uppercase();
        Rotor::check_ring(&so)?;
        Rotor::check_notches(&sn)?;

        Ok(Self {
            ring: so,
            notches: sn,
            position: 0,
            ring_offset: 0,
        })
    }

    pub fn rotor_i() -> Self {
        Self::new(RING_I, NOTCH_I).unwrap()
    }
    pub fn rotor_ii() -> Self {
        Self::new(RING_II, NOTCH_II).unwrap()
    }
    pub fn rotor_iii() -> Self {
        Self::new(RING_III, NOTCH_III).unwrap()
    }
    pub fn rotor_iv() -> Self {
        Self::new(RING_IV, NOTCH_IV).unwrap()
    }
    pub fn rotor_v() -> Self {
        Self::new(RING_V, NOTCH_V).unwrap()
    }
    pub fn rotor_vi() -> Self {
        Self::new(RING_VI, NOTCH_VI).unwrap()
    }
    pub fn rotor_vii() -> Self {
        Self::new(RING_VII, NOTCH_VII).unwrap()
    }
    pub fn rotor_viii() -> Self {
        Self::new(RING_VIII, NOTCH_VIII).unwrap()
    }
    pub fn reflector_a() -> Self {
        Self::new_static(RING_A).unwrap()
    }
    pub fn reflector_b() -> Self {
        Self::new_static(RING_B).unwrap()
    }
    pub fn reflector_c() -> Self {
        Self::new_static(RING_C).unwrap()
    }

    pub fn segment(self: &Self) -> char {
        return SEGMENTS.chars().nth(self.position).unwrap();
    }

    pub fn set_segment(self: &mut Self, pos: char) -> Result<char, RotorError> {
        let val = pos.to_ascii_uppercase();

        self.position = SEGMENTS
            .find(val)
            .ok_or(RotorError::InvalidSegmentPosition(pos))?;

        return Ok(val);
    }

    pub fn ring_offset(&self) -> char {
        return SEGMENTS.chars().nth(self.ring_offset).unwrap();
    }

    pub fn set_ring_offset(self: &mut Self, pos: char) -> Result<char, RotorError> {
        let val = pos.to_ascii_uppercase();

        self.ring_offset = SEGMENTS
            .find(val)
            .ok_or(RotorError::InvalidRingOffset(pos))?;

        return Ok(val);
    }

    // return true if at turnover notch
    pub fn at_notch(self: &Self) -> bool {
        if self
            .notches
            .contains(SEGMENTS.chars().nth(self.position).unwrap())
        {
            return true;
        }
        return false;
    }

    // return true if need turnover next rotor (was at notch position)
    pub fn advance(self: &mut Self) -> bool {
        let need_turnover = self.at_notch();
        if self.position == SEGMENTS.len() - 1 {
            self.position = 0;
        } else {
            self.position += 1;
        }
        return need_turnover;
    }

    pub fn forward(self: &Self, mut ch: char) -> Result<char, RotorError> {
        ch.make_ascii_uppercase();

        let segment = SEGMENTS
            .find(ch)
            .ok_or(RotorError::InvalidSegmentPosition(ch))?;

        let correction = self.position as isize - self.ring_offset as isize;

        // find firing segment index
        let mut index = correction + segment as isize;
        if index < 0 {
            index = SEGMENTS.len() as isize + index;
        } else if index as usize >= SEGMENTS.len() {
            index = (index as usize % SEGMENTS.len()) as isize;
        }

        // get output segment
        let val = self.ring.chars().nth(index as usize).unwrap();

        // apply correction for output segment index
        let mut index = SEGMENTS.find(val).unwrap() as isize - correction as isize;
        if index < 0 {
            index = SEGMENTS.len() as isize + index;
        } else if index as usize >= SEGMENTS.len() {
            index = (index as usize % SEGMENTS.len()) as isize;
        }

        return Ok(SEGMENTS.chars().nth(index as usize).unwrap());
    }

    pub fn backward(self: &Self, mut ch: char) -> Result<char, RotorError> {
        ch.make_ascii_uppercase();

        let segment = SEGMENTS
            .find(ch)
            .ok_or(RotorError::InvalidSegmentPosition(ch))?;

        let correction = self.position as isize - self.ring_offset as isize;

        let mut index = correction + segment as isize;
        if index < 0 {
            index = SEGMENTS.len() as isize + index;
        } else if index as usize >= SEGMENTS.len() {
            index = (index as usize % SEGMENTS.len()) as isize;
        }

        let val = SEGMENTS.chars().nth(index as usize).unwrap();

        let mut index = self.ring.find(val).unwrap() as isize - correction as isize;
        if index < 0 {
            index = SEGMENTS.len() as isize + index;
        } else if index as usize >= SEGMENTS.len() {
            index = (index as usize % SEGMENTS.len()) as isize;
        }

        return Ok(SEGMENTS.chars().nth(index as usize).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::device::rotor::Rotor;
    use crate::device::rotor::SEGMENTS;

    #[test]
    fn new_reflector() {
        Rotor::reflector_a();
        Rotor::reflector_b();
        Rotor::reflector_c();
    }

    #[test]
    fn new_rotors() {
        Rotor::rotor_i();
        Rotor::rotor_ii();
        Rotor::rotor_iii();
        Rotor::rotor_iv();
        Rotor::rotor_v();
        Rotor::rotor_vi();
        Rotor::rotor_vii();
        Rotor::rotor_viii();
    }

    #[test]
    fn set_active_segment() {
        let mut rotor = Rotor::rotor_i();
        assert!(rotor.segment() == 'A');
        assert!(rotor.set_segment('B').unwrap() == 'B');
        assert!(rotor.set_segment('b').unwrap() == 'B');
        assert!(rotor.segment() == 'B');
    }

    #[test]
    fn check_active_segment_failure() {
        Rotor::rotor_i().set_segment(',').unwrap_err();
    }

    #[test]
    fn set_ring_offset() {
        for offset in SEGMENTS.chars() {
            assert_eq!(Rotor::rotor_i().set_ring_offset(offset).unwrap(), offset);
        }
    }

    #[test]
    fn check_ring_offset_failure() {
        Rotor::rotor_i().set_ring_offset(':').unwrap_err();
    }

    #[test]
    fn encryption() {
        let rotors = vec![
            Rotor::rotor_i(),
            Rotor::rotor_ii(),
            Rotor::rotor_iii(),
            Rotor::rotor_iv(),
            Rotor::rotor_v(),
            Rotor::rotor_vi(),
            Rotor::rotor_vii(),
            Rotor::rotor_viii(),
            Rotor::reflector_a(),
            Rotor::reflector_b(),
            Rotor::reflector_c(),
        ];

        for mut rotor in rotors {
            for _ in 0..SEGMENTS.len() {
                rotor.advance();
                for ch in SEGMENTS.chars() {
                    for input in vec![ch, ch.to_ascii_lowercase()] {
                        let intermediate = rotor.forward(input).unwrap();
                        let output = rotor.backward(intermediate).unwrap();

                        assert_eq!(
                            input.to_ascii_uppercase(),
                            output,
                            "forward + backward not equal to original value"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn check_encryption_failure() {
        Rotor::rotor_i().forward('.').unwrap_err();
    }

    #[test]
    fn position_and_offset_are_equivalents() {
        let mut rotors = vec![
            Rotor::rotor_i(),
            Rotor::rotor_ii(),
            Rotor::rotor_iii(),
            Rotor::rotor_iv(),
            Rotor::rotor_v(),
            Rotor::rotor_vi(),
            Rotor::rotor_vii(),
            Rotor::rotor_viii(),
        ];
        let mut controls = vec![
            Rotor::rotor_i(),
            Rotor::rotor_ii(),
            Rotor::rotor_iii(),
            Rotor::rotor_iv(),
            Rotor::rotor_v(),
            Rotor::rotor_vi(),
            Rotor::rotor_vii(),
            Rotor::rotor_viii(),
        ];

        assert_eq!(controls.len(), rotors.len());

        for i in 0..controls.len() {
            let rotor = &mut rotors[i];
            let control = &mut controls[i];

            for segment in SEGMENTS.chars() {
                rotor.set_segment(segment).unwrap();
                rotor.set_ring_offset(segment).unwrap();

                // because ring_offset setting compensate segment setting outputs of
                // both rotors must be same
                for _ in 0..SEGMENTS.len() {
                    rotor.advance();
                    control.advance();
                    for ch in SEGMENTS.chars() {
                        assert_eq!(rotor.forward(ch).unwrap(), control.forward(ch).unwrap());
                    }
                }
            }
        }
    }
}
