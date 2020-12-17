#[cfg(not(windows))]
use aoc_rust::day11;
//use aoc_rust::day11_faster;
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(not(windows))]
use aoc_rust::day7;
#[cfg(not(windows))]
fn day7(c: &mut Criterion) {
    let contents = fs::read_to_string("/home/gsus/.config/aoc_helper/2020/7.in").unwrap();
    c.bench_function("day7::part1", |b| b.iter(|| day7::part1(&contents)));
}

#[cfg(not(windows))]
fn day11(c: &mut Criterion) {
    let contents = fs::read_to_string("/home/gsus/.config/aoc_helper/2020/11.in").unwrap();
    c.bench_function("day11::part1", |b| b.iter(|| day11::part1(&contents)));
}

#[cfg(not(windows))]
use aoc_rust::day12;

#[cfg(not(windows))]
fn day12(c: &mut Criterion) {
    let mut contents: &'static [u8] = include_bytes!("/home/gsus/.config/aoc_helper/2020/12.in");
    c.bench_function("day12::part1", |b| b.iter(|| day12::part1(&mut contents)));
    c.bench_function("day12::part2", |b| b.iter(|| day12::part2(&mut contents)));
}

#[cfg(not(windows))]
use aoc_rust::day13;
#[cfg(not(windows))]
fn day13(c: &mut Criterion) {
    let mut contents: &'static [u8] = include_bytes!("/home/gsus/.config/aoc_helper/2020/13.in");
    c.bench_function("day13::part1", |b| b.iter(|| day13::part1(&contents)));
}

use aoc_rust::day14;
#[cfg(windows)]
fn day14(c: &mut Criterion) {
    let contents: &'static [u8] = include_bytes!("../../python/inputs/14.txt");
    let line_mask = &contents[..=43];
    // mem[2816] = 272760856
    let line_assignment = &contents[3531..3552];
    c.bench_function("day14::parse_mask", |b| {
        b.iter(|| {
            let mut l = line_mask;
            day14::parse_mask(&mut l);
        });
    });
    c.bench_function("day14::parse_assignment", |b| {
        b.iter(|| {
            let mut l = line_assignment;
            day14::parse_assignment(&mut l);
        })
    });
    c.bench_function("day14::part1", |b| {
        b.iter(|| {
            day14::part1(&contents);
        })
    });
    c.bench_function("day14::part2", |b| {
        b.iter(|| {
            day14::part2(&contents);
        });
    });
}

use aoc_rust::day15;
fn day15(c: &mut Criterion) {
    let contents: &'static [u8] = include_bytes!("../inputs/15.txt");
    c.bench_function("day15::parse", |b| b.iter(|| day15::parse(&contents)));
    c.bench_function("day15::part1", |b| b.iter(|| day15::part1(&contents)));
    c.bench_function("day15::part2", |b| b.iter(|| day15::part2(&contents)));
}

use aoc_rust::day16;
fn day16(c: &mut Criterion) {
    let contents: &'static [u8] = include_bytes!("../inputs/day16.txt");
    c.bench_function("day16::parse(part1)", |b| {
        b.iter(|| {
            let mut c = contents;
            day16::ticket_info::TicketInfo::parse(&mut c);
        })
    });
    c.bench_function("day16::part1", |b| b.iter(|| day16::part1(&contents)));
}

// fn bench_part1(c: &mut Criterion) {
//     let contents = fs::read_to_string("/home/gsus/.config/aoc_helper/2020/11.in").unwrap();
//     c.bench_function("day11::part1", |b| b.iter(|| day11::part1(&contents)));
// }

// fn bench_fast(c: &mut Criterion) {
//     let contents = fs::read_to_string("/home/gsus/.config/aoc_helper/2020/11.in").unwrap();
//     c.bench_function("day11_faster::part1", |b| {
//         b.iter(|| day11_faster::part1_input(&contents))
//     });
// }

// criterion_group!(bench, bench_part1, bench_fast);
#[cfg(not(windows))]
criterion_group!(bench, day7, day11, day12);
#[cfg(windows)]
criterion_group!(bench, day14, day15, day16);
criterion_main!(bench);
