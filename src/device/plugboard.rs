use crate::device::details::ALPHABET;
use std::error;
use std::fmt;

pub struct PlugBoard {
    inputs: String,
    outputs: String,
}

#[derive(Debug, Clone)]
pub enum PlugBoardError {
    InvalidCharacter(char),
    Duplicate(char),
    SameCharacters,
    PairCountMismatch,
}

impl fmt::Display for PlugBoardError {
    fn fmt(self: &PlugBoardError, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlugBoardError::InvalidCharacter(ch) => write!(f, "invalid character: {}", ch),
            PlugBoardError::Duplicate(ch) => {
                write!(f, "plugboard already have connection for: {}", ch)
            }
            PlugBoardError::SameCharacters => {
                write!(f, "input and output of plug pair can not be same")
            }
            PlugBoardError::PairCountMismatch => {
                write!(f, "count of characters in pair settings should be even")
            }
        }
    }
}

impl error::Error for PlugBoardError {}

impl PlugBoard {
    pub fn new() -> Self {
        Self {
            inputs: String::new(),
            outputs: String::new(),
        }
    }

    // return added plug pairs
    pub fn pairs(self: &PlugBoard) -> &String {
        return &self.inputs;
    }

    pub fn add_pair(
        self: &mut PlugBoard,
        inp: char,
        out: char,
    ) -> Result<(char, char), PlugBoardError> {
        let input = inp.to_ascii_uppercase();
        let output = out.to_ascii_uppercase();

        if input == output {
            return Err(PlugBoardError::SameCharacters);
        }

        if ALPHABET.contains(input) == false {
            return Err(PlugBoardError::InvalidCharacter(input));
        } else if ALPHABET.contains(output) == false {
            return Err(PlugBoardError::InvalidCharacter(output));
        }

        if self.inputs.contains(input) {
            return Err(PlugBoardError::Duplicate(input));
        } else if self.outputs.contains(output) {
            return Err(PlugBoardError::Duplicate(output));
        }

        // forward
        self.inputs.push(input);
        self.outputs.push(output);

        // backward
        self.inputs.push(output);
        self.outputs.push(input);

        return Ok((input, output));
    }

    pub fn set_plug_pairs(&mut self, plug_pairs: &str) -> Result<&String, PlugBoardError> {
        if plug_pairs.len() % 2 != 0 {
            return Err(PlugBoardError::PairCountMismatch);
        }

        for i in 0..(plug_pairs.len() / 2) {
            let input = plug_pairs.chars().nth(i * 2).unwrap();
            let output = plug_pairs.chars().nth(i * 2 + 1).unwrap();

            self.add_pair(input, output)?;
        }

        return Ok(self.pairs());
    }

    pub fn crypt(self: &Self, ch: char) -> Result<char, PlugBoardError> {
        let val = ch.to_ascii_uppercase();

        if ALPHABET.contains(val) == false {
            return Err(PlugBoardError::InvalidCharacter(ch));
        }

        match self.inputs.find(val) {
            Some(pos) => {
                return Ok(self.outputs.chars().nth(pos).unwrap());
            }
            None => (),
        }

        match ALPHABET.find(val) {
            Some(_) => {
                return Ok(val);
            }
            None => {
                return Err(PlugBoardError::InvalidCharacter(ch));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::device::plugboard::PlugBoard;

    #[test]
    fn add_pair() {
        let mut board = PlugBoard::new();
        board.add_pair('A', 'b').unwrap();
        board.add_pair('C', 'X').unwrap();
        board.add_pair('C', 'D').unwrap_err(); // duplicate
        board.add_pair(':', 'V').unwrap_err();

        assert_eq!(board.pairs(), "ABCX");
    }

    #[test]
    fn crypt() {
        let mut board = PlugBoard::new();
        board.add_pair('A', 'B').unwrap();
        board.add_pair('C', 'D').unwrap();
        board.add_pair('E', 'F').unwrap();

        assert_eq!(board.pairs(), "ABCDEF");

        assert_eq!(board.crypt('Z').unwrap(), 'Z');
        assert_eq!(board.crypt('G').unwrap(), 'G');

        board.crypt(';').unwrap_err();
        board.crypt(',').unwrap_err();

        assert_eq!(board.crypt('A').unwrap(), 'B');
    }
}
