use crate::device::details::check_ring;
use crate::device::details::RING_A;
use crate::device::details::RING_B;
use crate::device::details::RING_C;
use crate::device::details::SEGMENTS;
use std::error;
use std::fmt;

// same as Rotor, but static
pub struct Reflector {
    // left side is always static, A-Z
    ring: String, // right side
}

#[derive(Debug, Clone)]
pub enum ReflectorError {
    InvalidReflectorType(/*reflector type*/ String),
    InvalidCharacter(char),
}

impl fmt::Display for ReflectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReflectorError::InvalidReflectorType(t) => write!(f, "invalid reflector type: {}", t),
            ReflectorError::InvalidCharacter(c) => write!(f, "invalid character: {}", c),
        }
    }
}

impl error::Error for ReflectorError {}

impl Reflector {
    pub fn new(outputs: &str) -> Result<Self, Box<dyn error::Error>> {
        let s = outputs.to_string().to_ascii_uppercase();
        check_ring(&s)?;

        Ok(Self { ring: s })
    }

    pub fn model(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let val = s.to_uppercase();

        if val == "A" {
            Self::new(RING_A)
        } else if val == "B" {
            Self::new(RING_B)
        } else if val == "C" {
            Self::new(RING_C)
        } else {
            Err(ReflectorError::InvalidReflectorType(val).into())
        }
    }

    pub fn forward(self: &Self, mut ch: char) -> Result<char, ReflectorError> {
        ch.make_ascii_uppercase();

        let index = SEGMENTS
            .find(ch)
            .ok_or(ReflectorError::InvalidCharacter(ch))?;

        let val = self
            .ring
            .chars()
            .nth(index)
            .ok_or(ReflectorError::InvalidCharacter(ch))?;

        Ok(val)
    }

    pub fn backward(self: &Self, ch: char) -> Result<char, ReflectorError> {
        return self.forward(ch);
    }
}

#[cfg(test)]
mod tests {
    use crate::device::details::SEGMENTS;
    use crate::device::Reflector;

    #[test]
    fn new_reflector() {
        Reflector::model("A").unwrap();
        Reflector::model("B").unwrap();
        Reflector::model("C").unwrap();
    }

    #[test]
    fn encryption() {
        let reflectors = vec![
            Reflector::model("A").unwrap(),
            Reflector::model("B").unwrap(),
            Reflector::model("C").unwrap(),
        ];

        for reflector in reflectors {
            for ch in SEGMENTS.chars() {
                for input in vec![ch, ch.to_ascii_lowercase()] {
                    let intermediate = reflector.forward(input).unwrap();
                    let output = reflector.backward(intermediate).unwrap();

                    assert_eq!(
                        input.to_ascii_uppercase(),
                        output,
                        "forward + backward not equal to original value"
                    );
                }
            }
        }
    }

    #[test]
    fn check_encryption_failure() {
        Reflector::model("A").unwrap().forward('$').unwrap_err();
    }
}
