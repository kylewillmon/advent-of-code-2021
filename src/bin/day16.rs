use anyhow::anyhow;

use aoc::{aoc_main, Result};
use nom::{sequence::tuple, IResult};

fn main() {
    use nom::Finish;
    aoc_main(
        16,
        |s| {
            let (_, out) = parse::input(s)
                .finish()
                .map_err(|e| anyhow!("failed to parse input: {:?}", e))?;
            Ok(out)
        },
        |transmission| {
            let (_, pkt) = nom::bits::bits(Packet::from_transmission)(&transmission)
                .finish()
                .map_err(|e: nom::error::Error<&[u8]>| {
                    anyhow!("failed to parse transmission: {:?}", e)
                })?;
            Ok(pkt.version_sum())
        },
        |transmission| {
            let (_, pkt) = nom::bits::bits(Packet::from_transmission)(&transmission)
                .finish()
                .map_err(|e: nom::error::Error<&[u8]>| {
                    anyhow!("failed to parse transmission: {:?}", e)
                })?;
            Ok(pkt.value())
        },
    )
    .unwrap()
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    data: PacketData,
}

impl Packet {
    fn from_transmission(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        use nom::bits::complete::take;
        let (input, version) = take(3usize)(input)?;
        let (input, type_) = take(3usize)(input)?;
        let (input, data) = match type_ {
            4 => PacketData::literal(input)?,
            _ => PacketData::operator(type_, input)?,
        };

        Ok((input, Packet { version, data }))
    }

    fn version_sum(&self) -> u64 {
        let sub = match self.data {
            PacketData::Operator { ref subpackets, .. } => {
                subpackets.iter().map(|s| s.version_sum()).sum()
            }
            _ => 0,
        };
        sub + u64::from(self.version)
    }

    fn value(&self) -> u64 {
        match self.data {
            PacketData::Literal { ref nibbles } => {
                let mut acc = 0;
                for n in nibbles {
                    acc <<= 4;
                    acc += u64::from(*n);
                }
                acc
            }
            PacketData::Operator { op, ref subpackets } => match op {
                0 => subpackets.iter().map(|pkt| pkt.value()).sum(),
                1 => subpackets.iter().map(|pkt| pkt.value()).product(),
                2 => subpackets.iter().map(|pkt| pkt.value()).min().unwrap(),
                3 => subpackets.iter().map(|pkt| pkt.value()).max().unwrap(),
                5 => {
                    if subpackets[0].value() > subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if subpackets[0].value() < subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if subpackets[0].value() == subpackets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("unexpected op type: {}", op),
            },
        }
    }
}

#[derive(Clone, Debug)]
enum PacketData {
    Literal { nibbles: Vec<u8> },
    Operator { op: u8, subpackets: Vec<Packet> },
}

impl PacketData {
    fn literal(input: (&[u8], usize)) -> IResult<(&[u8], usize), PacketData> {
        use nom::bits::complete::take;
        let mut input = input;
        let mut nibbles = Vec::new();
        loop {
            let (i, (next, nibble)): (_, (u8, _)) = tuple((take(1usize), take(4usize)))(input)?;
            input = i;
            nibbles.push(nibble);
            if next == 0 {
                break;
            }
        }
        Ok((input, PacketData::Literal { nibbles }))
    }

    fn operator(op: u8, input: (&[u8], usize)) -> IResult<(&[u8], usize), PacketData> {
        use nom::bits::complete::take;
        let mut subpackets = Vec::new();
        let (input, length_type): (_, u8) = take(1usize)(input)?;
        let mut input = input;

        if length_type == 0 {
            let (i, mut bits): (_, u16) = take(15usize)(input)?;
            input = i;

            let mut sub: Vec<u8> = Vec::new();
            while bits >= 8 {
                let (i, byte) = take(8usize)(input)?;
                input = i;

                sub.push(byte);
                bits -= 8;
            }
            if bits != 0 {
                let (i, byte): (_, u8) = take(bits)(input)?;
                input = i;
                sub.push(byte << (8 - bits));
            }

            let mut sub: &[u8] = &sub;
            let mut offset = 0;
            while sub.len() > 1 {
                let ((s, o), pkt) = Packet::from_transmission((sub, offset)).unwrap();
                sub = s;
                offset = o;
                subpackets.push(pkt);
            }
        } else {
            let (i, num_pkts): (_, u16) = take(11usize)(input)?;
            input = i;

            for _ in 0..num_pkts {
                let (i, pkt) = Packet::from_transmission(input).unwrap();
                input = i;
                subpackets.push(pkt);
            }
        }

        Ok((input, PacketData::Operator { op, subpackets }))
    }
}

mod parse {
    use std::num::ParseIntError;

    use super::*;

    use nom::{bytes::complete::take_while_m_n, combinator::map_res, multi::many1, IResult};

    fn from_hex(input: &str) -> Result<u8, ParseIntError> {
        fn inner(input: &str) -> Result<u8, ParseIntError> {
            u8::from_str_radix(input, 16)
        }
        if input.len() == 1 {
            let mut input = input.to_owned();
            input.push('0');
            inner(&input)
        } else {
            inner(input)
        }
    }

    fn hex_byte(input: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(1, 2, |c: char| c.is_digit(16)), from_hex)(input)
    }

    pub(super) fn input(input: &str) -> IResult<&str, Vec<u8>> {
        many1(hex_byte)(input)
    }
}
