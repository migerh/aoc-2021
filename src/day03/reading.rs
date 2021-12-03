use crate::day03::binary_to_decimal;
use std::num::ParseIntError;
use crate::utils::ParseError;
use core::str::FromStr;

#[derive(Clone, Debug)]
pub struct Reading {
    binary: Vec<char>,
}

impl FromStr for Reading {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binary = s.chars().collect::<Vec<_>>();
        Ok(Self { binary })
    }
}

impl Reading {
    pub fn get_bit(&self, pos: usize) -> Option<&char> {
        self.binary.get(pos)
    }

    pub fn len(&self) -> usize {
        self.binary.len()
    }

    pub fn decimal(&self) -> Result<usize, ParseIntError> {
        binary_to_decimal(&self.binary)
    }
}

