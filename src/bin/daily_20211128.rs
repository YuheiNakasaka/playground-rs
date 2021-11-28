use proconio::input;

// https://atcoder.jp/contests/abc172/tasks/abc172_a
fn main() {
    input! {
        a: i32,
    }
    println!("{}", a + (a * a) + (a * a * a));
}
