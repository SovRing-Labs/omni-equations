#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: i64) -> i64 { { let mut st = ((0i64), (0i64), (0i64)); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2) = st; if ((s1_0 < (a0.len() as i64)) as i64) == 0 { break; } st = (s1_0.wrapping_add((1i64)), s1_1.wrapping_add({ let mut acc2: i64 = (0i64); for item2 in ((0i64)..(63i64)) { acc2 = acc2.wrapping_add((if ((a0[(s1_0) as usize].wrapping_shr((item2 as u32) & 63) > (0i64)) as i64) != 0 { (1i64) } else { (0i64) })); } acc2 }.wrapping_sub((1i64))), s1_2.wrapping_add((if ((a1.wrapping_add((1i64)) > s1_1) as i64) != 0 { (1i64) } else { (0i64) }))); } st }.2 }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let w: i64 = t.next().unwrap().parse().unwrap(); assert_eq!(f(&v0, a1), w, "row {n}");
    }
    println!("ok {n}");
}
