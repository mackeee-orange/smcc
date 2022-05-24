#![feature(test)]

extern crate test;

use test::Bencher;

fn main() {
    draw_checkered_pattern();
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

#[bench]
fn bench_draw_checkered_pattern(b: &mut Bencher) {
    b.iter(|| draw_checkered_pattern());
}

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

#[bench]
fn bench_draw_checkered_pattern_str(b: &mut Bencher) {
    b.iter(|| draw_checkered_pattern_str());
}
