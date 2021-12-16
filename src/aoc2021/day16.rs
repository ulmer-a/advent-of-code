enum Packet {
    LiteralValue(u64),
    Operator(u8, Vec<Packet>),
}

struct HexPacketParser {
    data: Vec<u8>,
}

pub fn main(input: String) -> (u64, u64) {
    let mut lexer = HexPacketParser::new(&input);
    let (version_sum, packet) = lexer.parse_all_packets();
    (version_sum, eval_packet(&packet))
}

impl HexPacketParser {
    fn new(source: &str) -> Self {
        let mut bits: Vec<u8> = Vec::with_capacity(source.len() * 4);
        for hex_digit in source.chars().rev() {
            let binary_rep = hex_digit.to_digit(16).unwrap() as u8;
            bits.push(Self::nth_bit_set(binary_rep, 0));
            bits.push(Self::nth_bit_set(binary_rep, 1));
            bits.push(Self::nth_bit_set(binary_rep, 2));
            bits.push(Self::nth_bit_set(binary_rep, 3));
        }
        HexPacketParser { data: bits }
    }

    fn nth_bit_set(number: u8, n: usize) -> u8 {
        if (number & (1 << n)) != 0 {
            1
        } else {
            0
        }
    }

    fn number_from_n_bits(&mut self, n: usize) -> Option<u64> {
        let mut number = 0u64;
        for _ in 0..n {
            number = (number << 1) | self.data.pop()? as u64;
        }
        Some(number)
    }

    fn read_packet_header(&mut self) -> Option<(u8, u8)> {
        Some((
            self.number_from_n_bits(3)? as u8,
            self.number_from_n_bits(3)? as u8,
        ))
    }

    fn parse_literal_value(&mut self) -> Option<u64> {
        let mut number = 0;
        loop {
            let stop_bit = self.data.pop()?;
            number = (number << 4) | self.number_from_n_bits(4)?;

            if stop_bit == 0 {
                break Some(number);
            }
        }
    }

    fn parse_all_packets(&mut self) -> (u64, Packet) {
        let mut version_sum = 0;
        let packet = self.parse_packet(&mut version_sum);
        (version_sum, packet.unwrap())
    }

    fn parse_packets(
        &mut self,
        version_sum: &mut u64,
        bits: usize,
    ) -> Option<Vec<Packet>> {
        let original_length = self.data.len();
        let mut packets = vec![];
        while self.data.len() > original_length - bits {
            if let Some(packet) = self.parse_packet(version_sum) {
                packets.push(packet);
            } else {
                let sum: u8 = self.data.iter().sum();
                if sum > 0 {
                    None?
                }
            }
        }
        Some(packets)
    }

    fn parse_packet(&mut self, version_sum: &mut u64) -> Option<Packet> {
        let (version, packet_type) = self.read_packet_header()?;
        *version_sum += version as u64;
        Some(match packet_type {
            4 => Packet::LiteralValue(self.parse_literal_value()?),
            _ => Packet::Operator(packet_type, self.parse_operator(version_sum)?),
        })
    }

    fn parse_operator(&mut self, version_sum: &mut u64) -> Option<Vec<Packet>> {
        if self.data.pop()? == 0 {
            let subpacket_bits = self.number_from_n_bits(15)?;
            self.parse_packets(version_sum, subpacket_bits as usize)
        } else {
            let mut packets = vec![];
            let subpacket_count = self.number_from_n_bits(11)?;
            for _ in 0..subpacket_count {
                packets.push(self.parse_packet(version_sum)?);
            }
            Some(packets)
        }
    }
}

fn eval_packet(packet: &Packet) -> u64 {
    match packet {
        Packet::LiteralValue(value) => *value,
        Packet::Operator(op, operands) => {
            let operands: Vec<u64> =
                operands.iter().map(|p| eval_packet(p)).collect();
            match op {
                0 => operands.iter().sum(),
                1 => operands.iter().product(),
                2 => *operands.iter().min().unwrap(),
                3 => *operands.iter().max().unwrap(),
                5 => {
                    if operands[0] > operands[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if operands[0] < operands[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if operands[0] == operands[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        }
    }
}

#[cfg(test)]
fn evaluate(src: &str) -> u64 {
    main(src.into()).1
}

#[cfg(test)]
fn sumup_versions(src: &str) -> u64 {
    main(src.into()).0
}

#[test]
fn test_version_count() {
    assert_eq!(sumup_versions("8A004A801A8002F478"), 16);
    assert_eq!(sumup_versions("620080001611562C8802118E34"), 12);
    assert_eq!(sumup_versions("C0015000016115A2E0802F182340"), 23);
    assert_eq!(sumup_versions("A0016C880162017C3686B18A3D4780"), 31);
}

#[test]
fn test_evaluate() {
    assert_eq!(evaluate("C200B40A82"), 3);
    assert_eq!(evaluate("04005AC33890"), 54);
    assert_eq!(evaluate("880086C3E88112"), 7);
    assert_eq!(evaluate("CE00C43D881120"), 9);
    assert_eq!(evaluate("D8005AC2A8F0"), 1);
    assert_eq!(evaluate("F600BC2D8F"), 0);
    assert_eq!(evaluate("9C005AC2F8F0"), 0);
    assert_eq!(evaluate("9C0141080250320F1802104A08"), 1);
}

#[test]
fn test_hex_lex() {
    let mut lexer = HexPacketParser::new("DEADBEEF");
    assert_eq!(lexer.number_from_n_bits(3).unwrap(), 6);
    assert_eq!(lexer.number_from_n_bits(3).unwrap(), 7);
}
