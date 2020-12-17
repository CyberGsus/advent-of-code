pub mod ticket_info;

use bit_set::BitSet;
use itertools::{max, Itertools};
use lexical::parse_partial;
use std::{collections::HashMap, ops::RangeInclusive};
use ticket_info::parse_range;

// LF length
#[cfg(windows)]
const LINE_LEN: usize = 2;
#[cfg(not(windows))]
const LINE_LEN: usize = 1;

#[inline]
pub fn skip_until(input: &mut &[u8], until: u8) {
    while unsafe { input.get_unchecked(0) } != &until {
        *input = &input[1..];
    }
    *input = &input[1..];
}

#[inline]
unsafe fn parse_line_nums(input: &mut &[u8]) -> Option<Vec<u16>> {
    if input.len() == 0 {
        return None;
    }
    let mut v = Vec::new();
    while input.len() > 0 && input.get_unchecked(0) != &b'\n' {
        if input.get_unchecked(0) == &b',' {
            *input = &input[1..];
        }
        let (n, ndigits) = parse_partial::<u16, _>(&input).unwrap();
        *input = &input[ndigits..];
        v.push(n);
    }
    Some(v)
}

#[inline]
fn parse_line_ranges(mut input: &mut &[u8]) -> [RangeInclusive<u16>; 2] {
    skip_until(&mut input, b':');
    *input = &input[1..];
    let first_range = parse_range(&mut input);
    *input = &input[4..];
    let second_range = parse_range(&mut input);

    [first_range, second_range]
}

#[inline]
pub fn part1(mut input: &[u8]) -> u16 {
    // parses until "your ticket"
    let ticket_options = ticket_info::TicketInfo::parse_part1(&mut input);
    // ticket_options.normalize();
    input = &input[LINE_LEN..];
    // at "your ticket..."
    // skip until first is newline
    while unsafe { input.get_unchecked(0) } != &b's' {
        input = &input[1..];
    }
    // at "s:\n...." (from "nearby input[s]:\n...")
    input = &input[2 + LINE_LEN..];

    let mut sum = 0;
    //    now I can start getting numbers
    while input.len() > 1 {
        if b",\n".contains(unsafe { input.get_unchecked(0) }) {
            input = &input[1..];
        }
        let (n, ndigits) = parse_partial::<u16, _>(input).unwrap();
        input = &input[ndigits..];
        if !ticket_options.check_number(&n) {
            sum += n;
        }
    }

    sum
}

#[inline]
pub fn check_ticket_range(value: &u16, ranges: &[RangeInclusive<u16>; 2]) -> bool {
    ranges.iter().any(|v| v.contains(value))
}

#[inline]
fn get_ticket_impossible(value: &u16, ranges: &[[RangeInclusive<u16>; 2]]) -> Vec<usize> {
    ranges
        .iter()
        .enumerate()
        .filter_map(|(i, r)| {
            if check_ticket_range(value, &r) {
                None
            } else {
                Some(i)
            }
        })
        .collect()
}

#[inline]
fn check_ticket(ticket: &[u16], ranges: &[[RangeInclusive<u16>; 2]]) -> bool {
    ticket
        .iter()
        .all(|t| ranges.iter().any(|r| check_ticket_range(t, r)))
}

#[inline]
fn full_bitset(len: usize) -> BitSet {
    let mut b = BitSet::with_capacity(len);
    for i in 0..=len {
        b.insert(i);
    }
    b
}

// parsing on all right now
// TODO: parse just the 'departure' stuff
// NOTE: doesn't perform well on the actual input :(
pub fn part2(mut input: &[u8]) -> u64 {
    let mut ranges = Vec::with_capacity(6);
    while unsafe { input.get_unchecked(0) } != &b'\n' {
        ranges.push(parse_line_ranges(&mut input));
        input = &input[1..];
    }
    input = &input[1..];
    skip_until(&mut input, b'\n');
    let first_values = unsafe { parse_line_nums(&mut input).unwrap() };
    let max_len = first_values.len() - 1;
    input = &input[2..]; // 2 newlines
    skip_until(&mut input, b'\n');

    // do all allocation now.
    let mut map = vec![full_bitset(max_len); first_values.len()];
    let mut impossible = HashMap::with_capacity(max_len);


    /*
        algorithm:
        for each valid ticket:
            for each (index, value) of ticket:
                - if map[i] has length of 1 (it already has the solution), skip it.
                - remove every 'index' (property) for which the current value is not suitable from map[i]
                - if map[i] ends with length of 1, it already has its solution, therefore remove that solution
                  from the other sets of the map.

        There are N sets (one for each property) which should have only 1 index each at the end
        of the algorithm (their solution).
    */

    while let Some(next) = unsafe { parse_line_nums(&mut input) } {
        if check_ticket(&next, &ranges) {
            for (i, &v) in next.iter().enumerate() {
                let current = unsafe { map.get_unchecked_mut(i) };
                if current.len() == 1 {
                    continue;
                }
                for invalid in impossible
                    .entry(v)
                    .or_insert(get_ticket_impossible(&v, &ranges))
                {
                    current.remove(*invalid);
                }
                if current.len() == 1 {
                    // remove all from others which match the current
                    let v = unsafe { *current.iter().collect::<Vec<usize>>().get_unchecked(0) };
                    for j in 0..=max_len {
                        if j == i {
                            continue;
                        }
                        unsafe { map.get_unchecked_mut(j) }.remove(v);
                    }
                }
            }
        }
        if input.len() > 0 {
            input = &input[1..];
        }
    }
    dbg!(&map[..6]);

    unsafe {
        // right now each bitset must contain only one item
        map
            .iter()
            .map(|v| first_values[*v.iter().collect::<Vec<usize>>().get_unchecked(0)] as u64)
            .take(6)
            .product()
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT.as_bytes()), 71);
    }
    #[test]
    fn part2() {
        assert_eq!(
            super::part2(
                "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
                    .as_bytes(),
            ),
            12 * 11 * 13
        );
    }

    #[test]
    fn check_ticket_range() {
        let range = [1..=3, 13..=34];
        for (v, b) in [
            (1, true),
            (2, true),
            (3, true),
            (100, false),
            (6, false),
            (13, true),
            (34, true),
            (32, true),
            (35, false),
        ]
        .iter()
        {
            assert_eq!(super::check_ticket_range(v, &range), *b);
        }
    }

    #[test]
    fn check_ticket() {
        let ranges = [[1..=3, 5..=7], [6..=11, 33..=44], [13..=40, 45..=50]];
        for (v, b) in [
            (vec![7, 3, 47], true),
            (vec![40, 4, 50], false),
            (vec![55, 2, 20], false),
            (vec![38, 6, 12], false),
        ]
        .iter()
        {
            eprintln!("testing that check({:?}) == {}", &v, b);
            assert_eq!(super::check_ticket(v, &ranges), *b);
        }
    }
}
