fn fibo(prepre: u128, pre: u128, idx: i32, max_count: i32) {
    let num = if idx <= 2 { 1 } else { pre + prepre };
    println!("{idx}: {num}");
    if idx < max_count {
        fibo(pre, num, idx + 1, max_count);
    }
}

fn fibo2(n: i128) -> i128 {
    let nn = match n {
        0 => 0,
        1 => 1,
        _ => fibo2(n - 2) + fibo2(n - 1),
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
