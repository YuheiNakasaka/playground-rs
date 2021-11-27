use proconio::input;

// https://atcoder.jp/contests/abc184/tasks/abc184_a
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    input! {
        c: i32,
        d: i32,
    }
    println!("{}", a * d - b * c);
}
