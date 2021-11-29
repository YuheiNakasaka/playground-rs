use proconio::input;

// https://atcoder.jp/contests/abc182/tasks/abc182_a
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{}", (2 * a + 100) - b);
}
