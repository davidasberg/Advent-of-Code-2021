

use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_two(){
    let binary_string = read_input("input.txt");
    let mut chars = binary_string.chars().peekable();
    let mut bit_counter = 0;
    
    let packet = read_packet(&mut chars, &mut bit_counter);
    let evaluated_value = packet.evaluate_packet();
    println!("Evaluated expression: {}", evaluated_value);
}

fn part_one(){
    let binary_string = read_input("input.txt");
    let mut chars = binary_string.chars().peekable();
    let mut bit_counter = 0;
    
    let packet = read_packet(&mut chars, &mut bit_counter);
    let version_sum = packet.get_version_sum();
    println!("Part one: {}", version_sum);
}

fn read_packet(
    chars: &mut std::iter::Peekable<std::str::Chars>, 
    bit_counter: &mut i32,
) -> Packet {
    let packet_version = chars.take(3).collect::<String>();
    let packet_version = i32::from_str_radix(&packet_version, 2).unwrap();
    let packet_type = chars.take(3).collect::<String>();
    let packet_type = i32::from_str_radix(&packet_type, 2).unwrap();

    *bit_counter += 6;

    if packet_type == 4 {
        let mut literal: String = String::new();
        loop {
            let leading_bit = chars.next();
            for _ in 0..4 {
                literal.push(chars.next().unwrap());
            }
            *bit_counter += 5;
            if leading_bit == Some('0') {
                break;
            }
        }
        //parse binary string into i32
        let literal_decimal = i64::from_str_radix(&literal, 2).unwrap();
        //create new packet with literal
        let literal = PacketType::Literal{literal: literal_decimal};
        //add dyn literal to packets
        let packet = Packet::new(literal, packet_version);
        packet
    }
    else {
        let length_id = chars.next().unwrap();
        *bit_counter += 1;
        let length_id = i32::from_str_radix(&length_id.to_string(), 2).unwrap();
        let mut sub_packets: Vec<Packet> = Vec::new();
        match length_id {
            0 => {
                let l = chars.take(15).collect::<String>();
                *bit_counter += 15;
                let num_bits = i32::from_str_radix(&l, 2).unwrap();
                let current_bit_count = *bit_counter;
                while *bit_counter < num_bits + current_bit_count {
                    sub_packets.push(read_packet(chars, bit_counter));
                }
            }
            1 => {
                let l = chars.take(11).collect::<String>();
                *bit_counter += 11;
                let num_sub_packets = i32::from_str_radix(&l, 2).unwrap();
                for _ in 0..num_sub_packets {
                    sub_packets.push(read_packet(chars, bit_counter));
                }
            }
            _ => panic!("Invalid length id"),
        };
        let operator = PacketType::Operator{operator: packet_type};
        let mut packet = Packet::new(operator, packet_version);
        for sub_packet in sub_packets {
            packet.add_sub_packet(sub_packet);
        }
        packet
    }
    
}

enum PacketType {
    Literal {literal: i64},
    Operator {operator: i32},
}

struct Packet {
    packet_type: PacketType,
    packet_version: i32,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(packet_type: PacketType, packet_version: i32) -> Packet {
        Packet {
            packet_type,
            packet_version,
            sub_packets: Vec::new(),
        }
    }
    fn add_sub_packet(&mut self, sub_packet: Packet) {
        self.sub_packets.push(sub_packet);
    }
    fn get_version_sum(&self) -> i32 {
        let mut sum = self.packet_version;
        if self.sub_packets.len() == 0 {
            return self.packet_version
        }
        for sub_packet in &self.sub_packets {
            sum += sub_packet.get_version_sum();
        }
        sum        
    }
    // evaluate packet
    fn evaluate_packet(&self) -> i64 {
        match self.packet_type {
            PacketType::Literal {literal} => literal,
            PacketType::Operator {operator} => {
                match operator {
                    0 => self.sub_packets.iter().map(|p| p.evaluate_packet()).sum(),
                    1 => self.sub_packets.iter().map(|p| p.evaluate_packet()).product(),
                    2 => self.sub_packets.iter().map(|p| p.evaluate_packet()).min().unwrap(),
                    3 => self.sub_packets.iter().map(|p| p.evaluate_packet()).max().unwrap(),
                    5 => (self.sub_packets[0].evaluate_packet() > self.sub_packets[1].evaluate_packet()) as i64,
                    6 => (self.sub_packets[0].evaluate_packet() < self.sub_packets[1].evaluate_packet()) as i64,
                    7 => (self.sub_packets[0].evaluate_packet() == self.sub_packets[1].evaluate_packet()) as i64,
                    _ => panic!("Invalid operator"),
                }
            }
        }
    }
}

fn read_input(file: &str) -> String {
    let input = fs::read_to_string(file).expect("Unable to read file");
    hex_string_to_binary(input.trim())
}

//convert a hex string to a binary string
fn hex_string_to_binary(hex: &str) -> String {
    let mut result = String::new();
    for c in hex.chars() {
        match c {
            '0' => result.push_str("0000"),
            '1' => result.push_str("0001"),
            '2' => result.push_str("0010"),
            '3' => result.push_str("0011"),
            '4' => result.push_str("0100"),
            '5' => result.push_str("0101"),
            '6' => result.push_str("0110"),
            '7' => result.push_str("0111"),
            '8' => result.push_str("1000"),
            '9' => result.push_str("1001"),
            'A' => result.push_str("1010"),
            'B' => result.push_str("1011"),
            'C' => result.push_str("1100"),
            'D' => result.push_str("1101"),
            'E' => result.push_str("1110"),
            'F' => result.push_str("1111"),
            _ => panic!("Invalid hex character: {}", c),
        }
    }
    result
}