pub fn soin(n: i32) {
    let mut nokori = soin_exec(n, 2).1;

    let mut waru = 3;
    while waru * waru <= nokori {
        let (_, n) = soin_exec(nokori, waru);

        nokori = n;
        waru += 2;
    }
    if nokori > 1 { print!("{}", nokori); }

    println!("");
}

fn soin_exec(n: i32, warukazu: i32) -> (i32, i32) {
    let mut i = 0;
    let mut next_n = n;
    while next_n % warukazu == 0 {
        i += 1;
        next_n /= warukazu;
    }

    if i > 0 {
        print_soin(warukazu, i);
    }
    (i as i32, next_n)
}

fn print_soin(warukazu: i32, times: usize) {
    for _ in 0..times {
        print!("{}, ", warukazu)
    }
}