use itertools::Itertools;

fn parse_literal_varlength<F>(iter: &mut F) -> usize
    where F: Iterator<Item=char>
{
    let mut bits = Vec::new();
    while let Some(continue_reading) = iter.next() {
        match continue_reading {
            '1' => bits.extend(iter.take(4)),
            '0' => {
                bits.extend(iter.take(4));
                break
            },
            _ => unreachable!(),
        }
    }
    let bits = String::from_iter(bits);
    usize::from_str_radix(&bits, 2).unwrap()
}

fn parse_literal_fix_length<F>(iter: &mut F, length: usize) -> usize
    where F: Iterator<Item=char>
{
    let mut ans = 0;
    for _ in 0..length {
        ans = ans * 2 + iter.next().unwrap().to_digit(2).unwrap() as usize;
    }
    ans
}

struct Packet {
    version: u8,
    type_id: u8,
    version_sum: usize,
    value: usize,
}

fn parse_packet<F>(iter: &mut F) -> Packet
where F: Iterator<Item=char> {
    let version = u8::from_str_radix(String::from_iter(iter.by_ref().take(3)).as_str(), 2).unwrap();
    let type_id = u8::from_str_radix(String::from_iter(iter.by_ref().take(3)).as_str(), 2).unwrap();
    let mut version_sum: usize = version as usize;
    let mut literal = 0;
    let mut sub_packets = Vec::new();
    match type_id {
        4 => { // literal package
            literal = parse_literal_varlength(iter.by_ref());
        },
        _id => { // operator package
            match iter.next() {
                Some('0') => {
                    let subpacket_length = parse_literal_fix_length(iter.by_ref(), 15);
                    let subpacket_payload = iter.take(subpacket_length).collect_vec();
                    let mut iter = subpacket_payload.into_iter();
                    while !iter.is_empty() {
                        let p = parse_packet(&mut iter);
                        version_sum += p.version_sum;
                        sub_packets.push(p);
                    }
                },
                Some('1') => {
                    let subpacket_number = parse_literal_fix_length(iter.by_ref(), 11);
                    for _ in 0..subpacket_number {
                        let p = parse_packet(iter);
                        version_sum += p.version_sum;
                        sub_packets.push(p);
                    }
                },
                _ => panic!()
            }
        }
    }

    let value = match type_id {
        0 => sub_packets.into_iter().map(|p| p.value).sum(),
        1 => sub_packets.into_iter().map(|p| p.value).product(),
        2 => sub_packets.into_iter().map(|p| p.value).min().unwrap(),
        3 => sub_packets.into_iter().map(|p| p.value).max().unwrap(),
        4 => literal,
        5 => if sub_packets[0].value > sub_packets[1].value { 1 } else { 0 },
        6 => if sub_packets[0].value < sub_packets[1].value { 1 } else { 0 },
        7 => if sub_packets[0].value == sub_packets[1].value { 1 } else { 0 },
        _ => panic!(),
    };
    println!("value = {}", value);
    Packet { version, type_id, version_sum, value }
}

pub fn part1(input: &str) -> usize {
    let bits: Vec<_> = input.trim().chars().enumerate()
        .map(|(idx, c)| {
            match c.to_digit(16) {
                Some(i) => format!("{:04b}", i),
                _ => panic!("Cannot format {} as position {}", c, idx),
            }
        })
        .flat_map(|c| c.chars().collect_vec())
        .collect();

    let mut iter = bits.into_iter();
    let p = parse_packet(&mut iter);
    p.version_sum
}

pub fn part2(input: &str) -> usize {
    let bits: Vec<_> = input.trim().chars().enumerate()
        .map(|(idx, c)| {
            match c.to_digit(16) {
                Some(i) => format!("{:04b}", i),
                _ => panic!("Cannot format {} as position {}", c, idx),
            }
        })
        .flat_map(|c| c.chars().collect_vec())
        .collect();

    let mut iter = bits.into_iter();
    let p = parse_packet(&mut iter);
    p.value
}

#[cfg(test)]
mod tests {
    #[test]
    fn example1a() {
        assert_eq!(
            super::part1(
                "8A004A801A8002F478"
            ), 16
        )
    }

    #[test]
    fn example2a() {
        assert_eq!(
            super::part2(
                "C200B40A82"
            ), 3
        )
    }
}
