use proconio::input;

// https://atcoder.jp/contests/abc183/tasks/abc183_b
fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    let a = (sy - (-gy)) / (sx - gx);
    let b = sy - a * sx;
    println!("{}", b / (-a));
}
