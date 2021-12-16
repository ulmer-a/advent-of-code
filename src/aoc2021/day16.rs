struct HexLex {
    data: Vec<u8>,
}

enum Packet {
    LiteralValue(u64),
    Operator(u8, Vec<Packet>),
}

impl HexLex {
    fn new(source: &str) -> Self {
        let mut bits: Vec<u8> = Vec::with_capacity(source.len() * 4);
        for hex_digit in source.chars().rev() {
            let binary_rep = hex_digit.to_digit(16).unwrap() as u8;
            bits.push(Self::nth_bit_set(binary_rep, 0));
            bits.push(Self::nth_bit_set(binary_rep, 1));
            bits.push(Self::nth_bit_set(binary_rep, 2));
            bits.push(Self::nth_bit_set(binary_rep, 3));
        }
        HexLex { data: bits }
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
                //self.discard_padding();
                break Some(number);
            }
        }
    }

    fn parse_all_packets(&mut self) -> (u64, Vec<Packet>) {
        let mut version_sum = 0;
        let packets = self.parse_packets(&mut version_sum, self.data.len());
        (version_sum, packets.unwrap())
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

pub fn main(input: String) -> (u64, u64) {
    let mut lexer = HexLex::new(&input);
    let (version_sum, _) = lexer.parse_all_packets();
    (version_sum, 0)
}

#[test]
fn test1() {
    let input = r#"8A004A801A8002F478"#;
    let r = main(input.into());
    assert_eq!(r.0, 16);
}

#[test]
fn test2() {
    let input = r#"620080001611562C8802118E34"#;
    let r = main(input.into());
    assert_eq!(r.0, 12);
}

#[test]
fn test3() {
    let input = r#"C0015000016115A2E0802F182340"#;
    let r = main(input.into());
    assert_eq!(r.0, 23);
}

#[test]
fn test4() {
    let input = r#"A0016C880162017C3686B18A3D4780"#;
    let r = main(input.into());
    assert_eq!(r.0, 31);
}

#[test]
fn test_hex_lex() {
    let mut lexer = HexLex::new("DEADBEEF");
    assert_eq!(lexer.number_from_n_bits(3).unwrap(), 6);
    assert_eq!(lexer.number_from_n_bits(3).unwrap(), 7);
}
