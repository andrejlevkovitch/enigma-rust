use crate::device::reflector::Reflector;
use crate::device::rotor::Rotor;
use std::error;
use std::fmt;

pub struct Block {
    reflector: Option<Reflector>,
    rotors: Vec<Rotor>,
}

#[derive(Debug, Clone)]
pub enum BlockError {
    InvalidCountOfSegments(/*actual*/ usize, /*expected*/ usize),
    InvalidInput(/*input*/ char),
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockError::InvalidCountOfSegments(actual, expected) => write!(
                f,
                "invalid count of segments: {}/{} (actual/expected)",
                actual, expected
            ),
            BlockError::InvalidInput(ch) => write!(f, "invalid input character: {}", ch),
        }
    }
}

impl error::Error for BlockError {}

impl Block {
    pub fn new() -> Self {
        Self {
            reflector: None,
            rotors: Vec::<Rotor>::new(),
        }
    }

    pub fn set_reflector(&mut self, reflector: Reflector) {
        self.reflector = Some(reflector);
    }

    pub fn add_rotor(&mut self, rotor: Rotor) {
        self.rotors.push(rotor);
    }

    pub fn advance(self: &mut Self) {
        if self.rotors.is_empty() {
            return;
        }

        if self.rotors.last().unwrap().at_notch() {
            for rotor in self.rotors.iter_mut().rev() {
                if rotor.advance() == false {
                    break;
                }
            }
        } else {
            // check on double step
            // see https://en.wikipedia.org/wiki/Enigma_rotor_details
            // Normalized Enigma sequences
            let mut iter = self.rotors.iter_mut().rev();
            iter.next().unwrap().advance();
            match iter.next() {
                None => return,
                Some(next) => {
                    if next.at_notch() == false {
                        // not a double step case
                        return;
                    }
                    next.advance();
                }
            }

            // double step case
            for rotor in iter {
                if rotor.advance() == false {
                    break;
                }
            }
        }
    }

    pub fn segments(self: &Self) -> String {
        let mut retval = String::new();
        retval.reserve(self.rotors.len());
        for rotor in self.rotors.iter() {
            retval.push(rotor.segment());
        }
        return retval;
    }

    pub fn set_segments(self: &mut Self, segments: &str) -> Result<String, Box<dyn error::Error>> {
        if segments.len() != self.rotors.len() {
            return Err(
                BlockError::InvalidCountOfSegments(segments.len(), self.rotors.len()).into(),
            );
        }

        for i in 0..segments.len() {
            _ = self.rotors[i].set_segment(segments.chars().nth(i).unwrap())?;
        }

        return Ok(self.segments());
    }

    pub fn ring_offsets(&self) -> String {
        let mut offsets = String::new();

        for rotor in self.rotors.iter() {
            offsets.push(rotor.ring_offset());
        }

        return offsets;
    }

    pub fn set_ring_offsets(&mut self, offsets: &str) -> Result<String, Box<dyn error::Error>> {
        if offsets.len() != self.rotors.len() {
            return Err(
                BlockError::InvalidCountOfSegments(offsets.len(), self.rotors.len()).into(),
            );
        }

        for i in 0..self.rotors.len() {
            let rotor = &mut self.rotors[i];
            let offset = offsets.chars().nth(i).unwrap();

            _ = rotor.set_ring_offset(offset)?;
        }

        return Ok(self.ring_offsets());
    }

    pub fn crypt(self: &mut Self, ch: char) -> Result<char, Box<dyn error::Error>> {
        self.advance();

        let mut val = ch;
        for rotor in self.rotors.iter().rev() {
            val = rotor.forward(val)?;
        }

        match &self.reflector {
            None => return Ok(val),
            Some(reflector) => val = reflector.forward(val)?,
        }

        for rotor in self.rotors.iter() {
            val = rotor.backward(val)?;
        }

        return Ok(val);
    }
}

#[cfg(test)]
mod tests {
    use crate::device::block::Block;
    use crate::device::reflector::Reflector;
    use crate::device::rotor::Rotor;

    #[test]
    fn set_segments() {
        let mut block = Block::new();
        block.set_reflector(Reflector::model("B").unwrap());
        block.add_rotor(Rotor::model("I").unwrap());
        block.add_rotor(Rotor::model("II").unwrap());
        block.add_rotor(Rotor::model("III").unwrap());

        assert_eq!(block.set_segments("BhR").unwrap(), block.segments());
        assert_eq!(block.segments(), "BHR");
    }

    #[test]
    fn check_segments_failure_too_much_segments() {
        let mut block = Block::new();
        block.set_reflector(Reflector::model("B").unwrap());
        block.add_rotor(Rotor::model("I").unwrap());
        block.add_rotor(Rotor::model("II").unwrap());
        block.add_rotor(Rotor::model("III").unwrap());

        block.set_segments("AAAA").unwrap_err();
    }

    #[test]
    fn check_segments_failure_invalid_segment() {
        let mut block = Block::new();
        block.set_reflector(Reflector::model("B").unwrap());
        block.add_rotor(Rotor::model("I").unwrap());
        block.add_rotor(Rotor::model("II").unwrap());
        block.add_rotor(Rotor::model("III").unwrap());

        block.set_segments("AA;").unwrap_err();
    }

    #[test]
    fn double_step() {
        let mut block = Block::new();
        block.set_reflector(Reflector::model("B").unwrap());
        block.add_rotor(Rotor::model("I").unwrap());
        block.add_rotor(Rotor::model("II").unwrap());
        block.add_rotor(Rotor::model("III").unwrap());

        _ = block.set_segments("ADU").unwrap();
        block.advance();
        block.advance();
        block.advance();
        block.advance();

        assert_eq!(block.segments(), "BFY");
    }

    #[test]
    fn crypt() {
        let mut block = Block::new();
        block.set_reflector(Reflector::model("B").unwrap());
        block.add_rotor(Rotor::model("I").unwrap());
        block.add_rotor(Rotor::model("II").unwrap());
        block.add_rotor(Rotor::model("III").unwrap());

        _ = block.set_segments("PDU").unwrap();

        let mut encoded = String::new();
        for ch in "HelloWorld".chars() {
            match block.crypt(ch) {
                Ok(out) => encoded.push(out),
                Err(err) => panic!("unexpected error from block: {}", err),
            }
        }

        assert_eq!(encoded, "MPVJAELATQ");
    }
}
