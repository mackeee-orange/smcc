fn main() {
    println!("{}", max_diff(vec![1, 3, 5, 7, 78]));
    println!("{}", max_exclude_one(vec![1, 3, 5, 7, 78]));
}

fn max_diff(v: Vec<i32>) -> i32 {
    if v.len() < 1 {
        return 0;
    }

    v.iter().max().unwrap() - v.iter().min().unwrap()
}

fn max_exclude_one(v: Vec<i32>) -> i32 {
    if v.len() < 1 {
        return 0;
    }

    v.iter().fold(0, |sum, n| sum + n) - v.iter().min().unwrap()
}