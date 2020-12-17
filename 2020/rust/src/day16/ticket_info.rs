use lexical::parse_partial;
use std::cmp::{max, min};
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct TicketInfo {
    included_ranges: Vec<RangeInclusive<u16>>,
}

#[inline]
pub fn parse_range(input: &mut &[u8]) -> RangeInclusive<u16> {
    let (first, ndigits) = parse_partial::<u16, _>(&input).unwrap();
    *input = &input[ndigits + 1..]; // skip '-' as well
    let (last, ndigits) = parse_partial::<u16, _>(&input).unwrap();
    *input = &input[ndigits..];
    first..=last
}

impl TicketInfo {
    #[inline]
    pub fn parse(input: &mut &[u8], delimeter: u8) -> Self {
        let mut v = Self::new();
        while unsafe { input.get_unchecked(0) } != &delimeter {
            // find ':'
            while unsafe { input.get_unchecked(0) } != &b':' {
                *input = &input[1..];
            }
            // skip space
            *input = &input[2..];
            v.add_range(parse_range(input));
            *input = &input[4..];
            v.add_range(parse_range(input));
            *input = &input[1..];
        }

        v
    }

    #[inline]
    pub fn parse_part1(mut input: &mut &[u8]) -> Self {
        Self::parse(&mut input, b'\n')
    }

    #[inline]
    pub fn parse_part2(mut input: &mut &[u8]) -> Self {
        Self::parse(&mut input, b'a')
    }

    #[inline]
    pub fn new() -> Self {
        TicketInfo {
            included_ranges: Vec::new(),
        }
    }

    // TODO: use some processing to get better ranges
    #[inline]
    pub fn add_range(&mut self, r: RangeInclusive<u16>) {
        self.included_ranges.push(r);
    }

    // join overlapping ranges so I make less checks
    // on part 1
    #[inline]
    pub fn normalize(&mut self) -> &Self {
        // worst case no ranges overlap
        let mut map = Vec::with_capacity(self.included_ranges.len());

        'outer: for r in self.included_ranges.iter() {
            // if r.is_empty() { continue; }// don't add redundant stuff
            let (start, end) = (r.start(), r.end());
            for (a, b) in map.iter_mut() {
                if *start - 1 > *b {
                    continue;
                }
                *a = min(*a, *start);
                *b = max(*b, *end);
                continue 'outer;
            }
            map.push((*start, *end));
        }

        self.included_ranges.clear();
        for (a, b) in map.iter() {
            self.included_ranges.push(*a..=*b);
        }

        self
    }

    #[inline]
    pub fn check_number(&self, n: &u16) -> bool {
        for r in self.included_ranges.iter() {
            if r.contains(n) {
                return true;
            }
        }
        false
    }
}

impl PartialEq for TicketInfo {
    fn eq(&self, other: &TicketInfo) -> bool {
        self.included_ranges == other.included_ranges
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
    use super::*;
    macro_rules! ticket {
    ($($gen:expr),+) => {
       {
           let mut t = TicketInfo::new();
           $(
               t.add_range($gen);
           )+
           t
       }
    };
}

    #[test]
    fn parse() {
        let mut i = INPUT.as_bytes();
        assert_eq!(
            TicketInfo::parse_part1(&mut i),
            ticket!(1..=3, 5..=7, 6..=11, 33..=44, 13..=40, 45..=50)
        );
    }

    #[test]
    fn normalize() {
        let mut i = INPUT.as_bytes();
        assert_eq!(
            *TicketInfo::parse_part1(&mut i).normalize(),
            ticket!(1..=3, 5..=11, 13..=50)
        );
    }
}
