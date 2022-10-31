use std::env;

// 指定した金額を100円玉と10円玉と1円玉だけで、できるだけ少ない枚数で支払いたい。金額を入力するとそれぞれの枚数を計算して表示するプログラムを作成せよ。
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let price: u64 = args[1].parse()?;
    println!("{price}円は");

    let (n_100, x) = divide(price, 100, 1);
    let (n_10, x) = divide(x, 10, 1);
    let (n_1, x) = divide(x, 1, 1);

    println!("100: {n_100}, 10: {n_10}, 1: {n_1}");

    Ok(())
}

fn divide(n: u64, d: u64, times: usize) -> (usize, u64) {
    if n < d {
        return (times, n);
    }

    let m = n % d;
    let x = (n - m) / d;

    (x as usize, m)
}
