/*
day15::part1            time:   [55.457 us 55.558 us 55.700 us]
day15::part2            time:   [2.1921 s 2.2231 s 2.2546 s]
*/

use arrayvec::ArrayVec;
use lexical::parse_partial;
use std::u32;

#[inline]
pub fn parse(mut input: &[u8]) -> ArrayVec<[u32; 8]> {
    let mut vec = ArrayVec::new();
    while input.len() > 1 {
        if unsafe { input.get_unchecked(0) } == &b',' {
            input = &input[1..];
        }
        let (n, ndigits) = parse_partial::<u32, _>(input).unwrap();
        input = &input[ndigits..];
        vec.push(n);
    }
    vec
}

#[inline]
fn count_turns(input: &[u8], nth: u32) -> u32 {
    let mut history = vec![u32::MAX; (nth + 1) as usize];
    let parsed = parse(input);

    let (&last, initial) = parsed.split_first().unwrap();

    let mut last = last;

    for (i, &n) in initial.iter().enumerate() {
        history[n as usize] = (i + 1) as u32;
    }

    let mut current_turn = parsed.len() as u32;

    while current_turn < nth {
        let previous = unsafe { history.get_unchecked_mut(last as usize) };
        last = current_turn.saturating_sub(*previous); // if "previous" is not there then it'll be u32::MAX therefore last will be 0
        *previous = current_turn;
        current_turn += 1;
    }
    last
}

#[inline(always)]
pub fn part1(input: &[u8]) -> u32 {
    count_turns(input, 2020)
}

#[inline(always)]
pub fn part2(input: &[u8]) -> u32 {
    count_turns(input, 30000000)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        for (input, n) in [
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ]
        .iter()
        {
            assert_eq!(super::part1(input.as_bytes()), *n);
        }
    }
}
