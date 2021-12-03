use crate::day03::Reading;
use crate::utils::ParseError;
use crate::day03::binary_to_decimal;

#[derive(Clone)]
pub struct Report {
    readings: Vec<Reading>,
    ignored: Vec<usize>,
}

impl Report {
    pub fn new(readings: Vec<Reading>) -> Self {
        Self { readings, ignored: vec![] }
    }

    pub fn most_common_bit(&self, pos: usize) -> char {
        let mut num0 = 0;
        let mut num1 = 0;

        for i in 0..self.readings.len() {
            if self.ignored.contains(&i) {
                continue;
            }

            if let Some(c) = self.readings[i].get_bit(pos) {
                if c == &'0' {
                    num0 += 1;
                } else {
                    num1 += 1;
                }
            }
        }

        if num0 > num1 {
            '0'
        } else {
            '1'
        }
    }

    pub fn least_common_bit(&self, pos: usize) -> char {
        let most_common = self.most_common_bit(pos);

        if most_common == '1' {
            '0'
        } else {
            '1'
        }
    }

    pub fn ignore(&mut self, pos: usize, c: char) {
        for i in 0..self.readings.len() {
            if self.ignored.contains(&i) {
                continue;
            }

            if let Some(q) = self.readings[i].get_bit(pos) {
                if q == &c {
                    self.ignored.push(i);
                }
            }
        }
    }

    pub fn len(&self) -> Result<usize, ParseError> {
        self.readings.iter().map(|r| r.len()).max().ok_or(ParseError::new("No input?"))
    }

    pub fn number_of_reports(&self) -> usize {
        self.readings.len() - self.ignored.len()
    }

    pub fn first(&self) -> Option<&Reading> {
        for i in 0..self.readings.len() {
            if !self.ignored.contains(&i) {
                return self.readings.get(i);
            }
        }

        None
    }

    pub fn gamma(&self) -> Result<usize, ParseError> {
        let len = self.len()?;

        let mut gamma = vec![];
        for i in 0..len {
            gamma.push(self.most_common_bit(i));
        }

        Ok(binary_to_decimal(&gamma)?)
    }

    pub fn epsilon(&self) -> Result<usize, ParseError> {
        let len = self.len()?;

        let mut epsilon = vec![];
        for i in 0..len {
            epsilon.push(self.least_common_bit(i));
        }

        Ok(binary_to_decimal(&epsilon)?)
    }
}

