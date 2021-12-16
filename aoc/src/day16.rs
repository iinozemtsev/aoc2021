#![allow(dead_code)]
use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    packet_type: u8,
    payload: Payload,
}

impl Packet {
    fn from_hex_string(text: &str) -> Self {
        Packet::read(&mut BitQueue::from_hex_string(text))
    }
    fn read(queue: &mut BitQueue) -> Self {
        let version = queue.read_bits_as_num(3) as u8;
        let packet_type = queue.read_bits_as_num(3) as u8;
        let payload = match packet_type {
            4 => Packet::read_literal(queue),
            _ => Packet::read_operator(queue),
        };

        Packet {
            version,
            packet_type,
            payload,
        }
    }

    fn read_literal(queue: &mut BitQueue) -> Payload {
        let mut result = 0;
        loop {
            result <<= 4;
            let next = queue.read_bits_as_num(5);
            if next & 0b10000 == 0 {
                // last octet.
                result += next;
                return Payload::Literal { value: result };
            } else {
                result += next & 0b01111
            }
        }
    }

    fn read_operator(queue: &mut BitQueue) -> Payload {
        let len = Packet::read_length(queue);
        let packets = Packet::read_packets(&len, queue);
        Payload::Operator {
            length: len,
            subpackets: packets,
        }
    }

    fn read_length(queue: &mut BitQueue) -> Length {
        let len_type = queue.read_bits_as_num(1);
        if len_type == 0 {
            Length::Bits {
                value: queue.read_bits_as_num(15) as u16,
            }
        } else {
            Length::PacketCount {
                value: queue.read_bits_as_num(11) as u16,
            }
        }
    }

    fn read_packets(len: &Length, queue: &mut BitQueue) -> Vec<Packet> {
        match len {
            Length::Bits { value: bit_count } => {
                let mut result: Vec<Packet> = Vec::new();
                let initial_size = queue.value.len();
                loop {
                    result.push(Packet::read(queue));
                    let read = (initial_size - queue.value.len()) as u16;
                    if read == *bit_count {
                        return result;
                    }
                    if read > *bit_count {
                        panic!(
                            "unaligned packages! read: {}, expected: {}",
                            read, bit_count
                        );
                    }
                }
            }
            Length::PacketCount {
                value: packet_count,
            } => (0..*packet_count).map(|_| Packet::read(queue)).collect(),
        }
    }

    fn eval(&self) -> u64 {
        match &self.payload {
            Payload::Literal { value } => *value,
            Payload::Operator {
                length: _,
                subpackets,
            } => {
                let args = subpackets.iter().map(|x| x.eval()).collect::<Vec<u64>>();
                match self.packet_type {
                    0 => args.into_iter().sum(),
                    1 => args.into_iter().product(),
                    2 => args.into_iter().min().unwrap(),
                    3 => args.into_iter().max().unwrap(),
                    5 => {
                        if args[0] > args[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if args[0] < args[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if args[0] == args[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unknown packet type: {}", self.packet_type),
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Payload {
    Literal {
        value: u64,
    },
    Operator {
        length: Length,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum Length {
    Bits { value: u16 },
    PacketCount { value: u16 },
}

struct BitQueue {
    value: VecDeque<bool>,
}

impl BitQueue {
    fn new() -> Self {
        BitQueue {
            value: VecDeque::new(),
        }
    }

    fn from_hex_string(text: &str) -> Self {
        let mut result = BitQueue::new();
        for ch in text.trim().chars() {
            result.add_char(ch);
        }
        result
    }
    fn add_char(&mut self, ch: char) {
        if !ch.is_ascii() {
            panic!("not ascii: {}", ch);
        }

        let val = match ch {
            '0'..='9' => ch as u8 - '0' as u8,
            'A'..='F' => ch as u8 - 'A' as u8 + 10,
            _ => panic!("Unsupported char: {}", ch),
        };

        self.value.push_back(val & 0b1000 > 0);
        self.value.push_back(val & 0b0100 > 0);
        self.value.push_back(val & 0b0010 > 0);
        self.value.push_back(val & 0b0001 > 0);
    }

    fn read_bits_as_num(&mut self, count: u8) -> u64 {
        let mut result = 0;
        for _ in 0..count {
            result = result * 2
                + if self.value.pop_front().unwrap() {
                    1
                } else {
                    0
                }
        }
        result
    }
}

fn total_version(p: &Packet) -> u32 {
    match &p.payload {
        Payload::Operator {
            length: _,
            subpackets,
        } => subpackets.iter().map(total_version).sum::<u32>() + p.version as u32,
        Payload::Literal { value: _ } => p.version as u32,
    }
}
fn part1(text: &str) -> u32 {
    total_version(&Packet::from_hex_string(text))
}
fn part2(text: &str) -> u64 {
    Packet::from_hex_string(text).eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_packet() {
        let mut queue = BitQueue::from_hex_string("D2FE28");
        println!("{:?}", Packet::read(&mut queue))
    }

    #[test]
    fn len_type_0_packet() {
        let mut queue = BitQueue::from_hex_string("38006F45291200");
        println!("{:?}", Packet::read(&mut queue))
    }

    #[test]
    fn len_type_1_packet() {
        let mut queue = BitQueue::from_hex_string("EE00D40C823060");
        println!("{:#?}", Packet::read(&mut queue))
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1("8A004A801A8002F478"), 16);
        assert_eq!(super::part1("620080001611562C8802118E34"), 12);
        assert_eq!(super::part1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::part1("A0016C880162017C3686B18A3D4780"), 31);
        println!("part1: {}", super::part1(include_str!("../resources/day16.txt")));
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("C200B40A82"), 3);
        assert_eq!(super::part2("04005AC33890"), 54);
        assert_eq!(super::part2("880086C3E88112"), 7);
        assert_eq!(super::part2("CE00C43D881120"), 9);
        assert_eq!(super::part2("D8005AC2A8F0"), 1);
        assert_eq!(super::part2("F600BC2D8F"), 0);
        assert_eq!(super::part2("9C005AC2F8F0"), 0);
        assert_eq!(super::part2("9C0141080250320F1802104A08"), 1);
        println!("part2: {}", super::part2(include_str!("../resources/day16.txt")));
    }
}
