enum Packet {
    Literal {
        version: u8,
        literal: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        content: Vec<Packet>,
    },
}

pub struct PacketDecoder {
    code: Vec<u8>,
}

impl crate::Advent for PacketDecoder {
    fn new(data: &str) -> Self {
        let mut code = Vec::new();

        // decode each HEX char into four bits and append them to the vector
        for c in data.chars() {
            match c {
                '0' => code = vec![code, vec![0, 0, 0, 0]].concat(),
                '1' => code = vec![code, vec![0, 0, 0, 1]].concat(),
                '2' => code = vec![code, vec![0, 0, 1, 0]].concat(),
                '3' => code = vec![code, vec![0, 0, 1, 1]].concat(),
                '4' => code = vec![code, vec![0, 1, 0, 0]].concat(),
                '5' => code = vec![code, vec![0, 1, 0, 1]].concat(),
                '6' => code = vec![code, vec![0, 1, 1, 0]].concat(),
                '7' => code = vec![code, vec![0, 1, 1, 1]].concat(),
                '8' => code = vec![code, vec![1, 0, 0, 0]].concat(),
                '9' => code = vec![code, vec![1, 0, 0, 1]].concat(),
                'A' => code = vec![code, vec![1, 0, 1, 0]].concat(),
                'B' => code = vec![code, vec![1, 0, 1, 1]].concat(),
                'C' => code = vec![code, vec![1, 1, 0, 0]].concat(),
                'D' => code = vec![code, vec![1, 1, 0, 1]].concat(),
                'E' => code = vec![code, vec![1, 1, 1, 0]].concat(),
                'F' => code = vec![code, vec![1, 1, 1, 1]].concat(),
                _ => ()
            }
        }

        PacketDecoder {
            code
        }
    }

    fn year() -> u16 { 2021 }

    fn day() -> u8 { 16 }

    fn part1(&self) -> usize {
        let mut curren_index = 0;
        let packet = decode(&self.code, &mut curren_index);

        sum_versions(&packet)
    }

    fn part2(&self) -> usize {
        let mut curren_index = 0;
        let packet = decode(&self.code, &mut curren_index);

        get_value(&packet)
    }
}

/// Gets the value of the packet. For literal packets, its value is returned. Operand packets will
/// make a specific operation on its children packets and return the resulting value.
fn get_value(packet: &Packet) -> usize {
    let mut result = 0;

    result += match packet {
        Packet::Literal { literal, .. } => *literal,
        Packet::Operator { type_id, content, .. } => {
            let values = content.iter().map(|f| get_value(f)).collect::<Vec<usize>>();
            // decide what to do with the children based on the type id. uses a recursive call to
            // this function to determine the value of the children.
            match type_id {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => if get_value(&content[0]) > get_value(&content[1]) { 1 } else { 0 },
                6 => if get_value(&content[0]) < get_value(&content[1]) { 1 } else { 0 },
                7 => if get_value(&content[0]) == get_value(&content[1]) { 1 } else { 0 },
                _ => 0
            }
        }
    };

    result
}

fn sum_versions(root: &Packet) -> usize {
    let mut result = 0;

    match root {
        Packet::Literal { version, .. } => result += *version as usize,
        Packet::Operator { version, content, .. } => {
            result += *version as usize;
            result += content.iter().map(|p| sum_versions(p)).sum::<usize>();
        }
    }

    result
}

/// Decodes a packet and moves current_index to the start of the next packet.
fn decode(code: &Vec<u8>, current_index: &mut usize) -> Packet {
    let version = decode_literal(code, current_index, 3) as u8;
    let type_id = decode_literal(code, current_index, 3) as u8;

    match type_id {
        4 => Packet::Literal { version, literal: decode_literal_chain(code, current_index) },
        _ => {
            let operator_type = decode_literal(code, current_index, 1) as u8;
            let mut content = Vec::new();
            let length;

            if operator_type == 0 {
                // For type 0: use the length as number of bits and read more packets as long as
                // the length has not yet been reached.
                length = decode_literal(code, current_index, 15);
                let start_index = *current_index;

                while start_index + length > *current_index {
                    content.push(decode(code, current_index));
                }
            } else {
                // for type 1: use the length as number of packets. read as many packets.
                for _ in 0..decode_literal(code, current_index, 11) {
                    content.push(decode(code, current_index));
                }
            }
            Packet::Operator { version, type_id, content }
        }
    }
}

/// Decodes a chain of literal where the length is unknown. The function always looks at 5 bits at
/// a time. The first bit designates if more values will follow (1: yes, 0: this is the last one).
/// The other 4 bit are concatenated and will give the resulting literal. Moves current_index.
fn decode_literal_chain(code: &Vec<u8>, current_index: &mut usize) -> usize {
    let mut result = 0;

    loop {
        *current_index += 1;
        result <<= 4;
        result += decode_literal(code, current_index, 4);
        if code[*current_index - 5] == 0 { break; }
    }

    result
}


/// Decodes a bit stream of known length into a literal. Moves current_index.
fn decode_literal(code: &Vec<u8>, current_index: &mut usize, length: usize) -> usize {
    let mut result = 0;

    for _ in 0..length {
        result <<= 1;
        result = result + code[*current_index] as usize;
        *current_index += 1;
    }

    result
}