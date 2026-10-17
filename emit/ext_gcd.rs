#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64, a1: i64) -> Vec<i64> { { let mut v = { let mut v = { let mut v = (((0i64))..((0i64))).collect::<Vec<i64>>().to_vec(); v.push({ let mut st = (a0, a1, (1i64), (0i64), (0i64), (1i64)); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2, s1_3, s1_4, s1_5) = st; if ((s1_1 > (0i64)) as i64) == 0 { break; } st = (s1_1, s1_0.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_1)), s1_3, s1_2.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_3)), s1_5, s1_4.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_5))); } st }.0); v }.to_vec(); v.push({ let mut st = (a0, a1, (1i64), (0i64), (0i64), (1i64)); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2, s1_3, s1_4, s1_5) = st; if ((s1_1 > (0i64)) as i64) == 0 { break; } st = (s1_1, s1_0.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_1)), s1_3, s1_2.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_3)), s1_5, s1_4.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_5))); } st }.2); v }.to_vec(); v.push({ let mut st = (a0, a1, (1i64), (0i64), (0i64), (1i64)); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2, s1_3, s1_4, s1_5) = st; if ((s1_1 > (0i64)) as i64) == 0 { break; } st = (s1_1, s1_0.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_1)), s1_3, s1_2.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_3)), s1_5, s1_4.wrapping_sub((s1_0 / s1_1).wrapping_mul(s1_5))); } st }.4); v } }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let a0: i64 = t.next().unwrap().parse().unwrap();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(a0, a1), w, "row {n}");
    }
    println!("ok {n}");
}
