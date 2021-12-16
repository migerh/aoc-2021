use std::cmp::{min, max};
use std::num::ParseIntError;
use crate::utils::ParseError;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Result<Vec<u8>, ParseError> {
    Ok(input
        .lines()
        .filter(|s| *s != "")
        .take(1)
        .next().ok_or(ParseError::new("No input"))?
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| u8::from_str_radix(&c.iter().collect::<String>(), 16))
        .collect::<Result<Vec<_>, ParseIntError>>()?)
}

#[derive(Debug)]
struct Operator {
    version: u8,
    type_id: u8,
    length_id: u8,
    length: usize,
    packets: Vec<Packet>
}

#[derive(Debug)]
struct Value {
    version: u8,
    value: usize,
}

#[derive(Debug)]
enum Packet {
    Op(Operator),
    Val(Value),
}

impl Packet {
    fn value(&self) -> Result<usize, ParseError> {
        if let Packet::Val(v) = self {
            return Ok(v.value);
        }

        if let Packet::Op(o) = self {
            let value = match o.type_id {
                0 => o.packets.iter().map(|p| p.value()).try_fold(0, |a, v| -> Result<usize, ParseError> { Ok(a + v?) })?,
                1 => o.packets.iter().map(|p| p.value()).try_fold(1, |a, v| -> Result<usize, ParseError> { Ok(a * v?) })?,
                2 => o.packets.iter().map(|p| p.value()).try_fold(usize::MAX, |a, v| -> Result<usize, ParseError> { Ok(min(a, v?)) })?,
                3 => o.packets.iter().map(|p| p.value()).try_fold(0, |a, v| -> Result<usize, ParseError> { Ok(max(a, v?)) })?,
                5 => Packet::gt(o)?,
                6 => Packet::lt(o)?,
                7 => Packet::eq(o)?,
                _ => Err(ParseError::new("Unknown operator"))?
            };
            return Ok(value);
        }

        Err(ParseError::new("Unknown packet type"))
    }

    fn gt(o: &Operator) -> Result<usize, ParseError> {
        if o.packets.len() != 2 {
            return Err(ParseError::new("Invalid number of operands"));
        }

        Ok(if o.packets[0].value()? > o.packets[1].value()? {
            1
        } else {
            0
        })
    }

    fn lt(o: &Operator) -> Result<usize, ParseError> {
        if o.packets.len() != 2 {
            return Err(ParseError::new("Invalid number of operands"));
        }

        Ok(if o.packets[0].value()? < o.packets[1].value()? {
            1
        } else {
            0
        })
    }

    fn eq(o: &Operator) -> Result<usize, ParseError> {
        if o.packets.len() != 2 {
            return Err(ParseError::new("Invalid number of operands"));
        }

        Ok(if o.packets[0].value()? == o.packets[1].value()? {
            1
        } else {
            0
        })
    }
}

#[derive(Debug)]
struct Parser {
    value: Vec<char>,
    pointer: usize,
}

impl Parser {
    fn new(value: Vec<char>) -> Self {
        Parser { value, pointer: 0 }
    }

    fn parse(&mut self) -> Result<Packet, ParseError> {
        let packet = self.parse_packet()?;
        Ok(packet)
    }

    fn parse_packet(&mut self) -> Result<Packet, ParseError> {
        let version = Parser::to_u8(&self.next(3))?;
        let type_id = Parser::to_u8(&self.next(3))?;

        let packet = if type_id == 4 {
            let value = self.read_value()?;
            Packet::Val(Value { version, value })
        } else {
            let length_id = Parser::to_u8(&self.next(1))?;
            let length = if length_id == 0 {
                Parser::to_usize(&self.next(15))?
            } else {
                Parser::to_usize(&self.next(11))?
            };

            let packets = if length_id == 0 {
                self.parse_packets_bit_length(length)?
            } else {
                self.parse_packets_number(length)?
            };

            Packet::Op(Operator { version, type_id, length_id, length, packets })
        };

        Ok(packet)
    }

    fn parse_packets_bit_length(&mut self, length: usize) -> Result<Vec<Packet>, ParseError> {
        let mut children = vec![];
        let current_pointer = self.pointer;
        while self.pointer < current_pointer + length {
            let packet = self.parse_packet()?;
            children.push(packet);
        }

        Ok(children)
    }

    fn parse_packets_number(&mut self, length: usize) -> Result<Vec<Packet>, ParseError> {
        let mut children = vec![];
        while children.len() != length {
            let packet = self.parse_packet()?;
            children.push(packet);
        }

        Ok(children)
    }

    fn next(&mut self, n: usize) -> Vec<char> {
        let result = self.value.iter().skip(self.pointer).take(n).cloned().collect::<Vec<_>>();
        self.pointer += n;
        result
    }

    fn to_usize(v: &Vec<char>) -> Result<usize, ParseIntError> {
        usize::from_str_radix(&v.iter().collect::<String>(), 2)
    }

    fn to_u8(v: &Vec<char>) -> Result<u8, ParseIntError> {
        u8::from_str_radix(&v.iter().collect::<String>(), 2)
    }

    fn read_value(&mut self) -> Result<usize, ParseIntError> {
        let mut continuation = self.next(1)[0];
        let mut total = 0;

        while continuation == '1' {
            let n = Parser::to_usize(&self.next(4))?;
            total = total * 16 + n;

            continuation = self.next(1)[0];
        }

        let n = Parser::to_usize(&self.next(4))?;
        total = total * 16 + n;

        Ok(total)
    }
}

fn get_version(packet: &Packet) -> usize {
    if let Packet::Val(v) = packet {
        v.version as usize
    } else if let Packet::Op(o) = packet {
        let version_sum: usize = o.packets.iter().map(|p| get_version(p)).sum();
        (o.version as usize) + version_sum
    } else {
        0
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<u8>) -> Result<usize, ParseError> {
    let bits = input.iter().map(|v| format!("{:08b}", v)).collect::<String>().chars().collect::<Vec<_>>();
    let mut p = Parser::new(bits);

    let packet = p.parse()?;
    let version = get_version(&packet);

    Ok(version)
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &Vec<u8>) -> Result<usize, ParseError> {
    let bits = input.iter().map(|v| format!("{:08b}", v)).collect::<String>().chars().collect::<Vec<_>>();
    let mut p = Parser::new(bits);
    let packet = p.parse()?;

    Ok(packet.value()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn sample() -> &'static str {
        "A0016C880162017C3686B18A3D4780"
    }

    fn input() -> Result<Vec<u8>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(31, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(54, solve_part2(&data)?))
    }
}
