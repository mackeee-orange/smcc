// #![feature(test)]
//
// extern crate test;
//
// use test::Bencher;

use std::env;
use std::path::PathBuf;
use crate::kaibun::check_kaibun;
use crate::soin::soin;

mod kaibun;
mod tree;
mod soin;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    //
    // tree::tree(PathBuf::from(&args[1]), 0)

    soin(args[1].parse().unwrap());

    Ok(())
}

fn draw_checkered_pattern() {
    let mut matrix = [[0; 10]; 10];
    for y in 0..10 {
        for x in 0..10 {
            if x % 2 == y % 2 {
                matrix[y][x] = 4;
            }
        }
        println!("{:?}", matrix[y]);
    }
}
//
// #[bench]
// fn bench_draw_checkered_pattern(b: &mut Bencher) {
//     b.iter(|| draw_checkered_pattern());
// }

fn draw_checkered_pattern_str() {
    for y in 0..10 {
        let mut s = String::new();
        for x in 0..10 {
            if x % 2 == y % 2 {
                s.push('4');
            } else {
                s.push('0');
            }
        }
        println!("{s}");
    }
}

// #[bench]
// fn bench_draw_checkered_pattern_str(b: &mut Bencher) {
//     b.iter(|| draw_checkered_pattern_str());
// }

fn pickup_first_unique_str(s: String) -> Option<String> {
    let mut ss = s.clone();
    let mut buf = vec![];
    for (i, x) in s.chars().enumerate() {
        buf.push(x);
        if i > 0 {
            let mut idx = -1 as i32;
            for (j, b) in buf.iter().enumerate() {
                if b.to_string() == x.to_string() {
                    println!("{:?}", b);
                    idx = j as i32;
                    if idx > 0 {
                        println!("{:?}", ss);
                        println!("{idx}");
                        ss.remove((idx) as usize);
                    }
                }
            }

        }
    }
    println!("{:?}", ss);
    Some(ss)
}

fn fibo(prepre: u128, pre: u128, idx: i32, max_count: i32) {
    let num = if idx <= 2 {
        1
    } else {
        pre + prepre
    };
    println!("{idx}: {num}");
    if idx < max_count {
        fibo(pre, num, idx + 1, max_count);
    }
}

fn fibo2(n: i128) -> i128 {
    let nn = match n {
        0 => 0,
        1 => 1,
        _ => fibo2(n - 2) + fibo2(n - 1)
    };
    println!("{nn}");
    nn
}


// #[bench]
// fn bench_fibo2(b: &mut Bencher) {
//     b.iter(|| fibo2(100));
// }
//
// #[bench]
// fn bench_fibo(b: &mut Bencher) {
//     b.iter(|| fibo(1, 1, 1, 100));
// }