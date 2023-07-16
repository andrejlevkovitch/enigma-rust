pub mod block;
pub mod details;
pub mod plugboard;
pub mod reflector;
pub mod rotor;

use crate::device::block::Block;
use crate::device::details::ALPHABET;
use crate::device::plugboard::PlugBoard;
use crate::device::reflector::Reflector;
use crate::device::rotor::Rotor;
use std::error;
use std::fmt;

pub struct Device {
    board: PlugBoard,
    block: Block,
}

#[derive(Debug, Clone)]
pub enum DeviceError {
    InvalidCharacter(char),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceError::InvalidCharacter(ch) => write!(f, "invalid character: {}", ch),
        }
    }
}

impl error::Error for DeviceError {}

impl Device {
    pub fn new() -> Self {
        Self {
            board: PlugBoard::new(),
            block: Block::new(),
        }
    }

    // always left
    pub fn set_reflector(&mut self, reflector: Reflector) {
        self.block.set_reflector(reflector);
    }

    // from left to right
    pub fn add_rotor(&mut self, rotor: Rotor) {
        self.block.add_rotor(rotor);
    }

    // always left
    pub fn set_reflector_type(
        &mut self,
        reflector_type: &str,
    ) -> Result<(), Box<dyn error::Error>> {
        self.block.set_reflector(Reflector::model(reflector_type)?);
        Ok(())
    }

    // from left to right
    pub fn add_rotor_type(&mut self, rotor_type: &str) -> Result<(), Box<dyn error::Error>> {
        self.block.add_rotor(Rotor::model(rotor_type)?);
        Ok(())
    }

    pub fn segments(self: &Self) -> String {
        return self.block.segments();
    }

    pub fn set_segments(
        self: &mut Self,
        segments: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        return Ok(self.block.set_segments(segments)?);
    }

    pub fn ring_offsets(&self) -> String {
        return self.block.ring_offsets();
    }

    pub fn set_ring_offsets(
        &mut self,
        offsets: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        return Ok(self.block.set_ring_offsets(offsets)?);
    }

    pub fn plugboard(self: &Self) -> &String {
        return self.board.pairs();
    }

    pub fn add_plug_pair(
        &mut self,
        input: char,
        output: char,
    ) -> Result<(char, char), Box<dyn error::Error>> {
        return Ok(self.board.add_pair(input, output)?);
    }

    pub fn set_plug_pairs(&mut self, plug_pairs: &str) -> Result<&String, Box<dyn error::Error>> {
        return Ok(self.board.set_plug_pairs(plug_pairs)?);
    }

    pub fn crypt(self: &mut Self, ch: char) -> Result<char, Box<dyn error::Error>> {
        let mut val = ch.to_ascii_uppercase();
        if ALPHABET.contains(val) == false {
            return Err(DeviceError::InvalidCharacter(ch).into());
        }

        val = self.board.crypt(val)?;
        val = self.block.crypt(val)?;
        val = self.board.crypt(val)?;

        return Ok(val);
    }
}

#[cfg(test)]
mod tests {
    use crate::device::Device;

    #[test]
    fn crypt() {
        let mut device = Device::new();
        device.set_reflector_type("B").unwrap();
        device.add_rotor_type("I").unwrap();
        device.add_rotor_type("II").unwrap();
        device.add_rotor_type("III").unwrap();

        device.set_plug_pairs("HWKLAO").unwrap();
        device.set_segments("PDU").unwrap();
        device.set_ring_offsets("IUP").unwrap();

        let mut encoded = String::new();

        for ch in "Hello, World!!".chars() {
            match device.crypt(ch) {
                Ok(out) => encoded.push(out),
                Err(_) => (), // ignore
            }
        }

        assert_eq!(encoded, "WHJDZGZLEN");
    }
}
