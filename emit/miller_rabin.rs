#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64, a1: i64) -> i64 { { let mut acc1: i64 = { let mut st = (((1i64) % a0), (a1 % a0), (a0.wrapping_sub((1i64)) / (a0.wrapping_sub((1i64)) & (0i64).wrapping_sub(a0.wrapping_sub((1i64)))))); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2) = st; if ((s1_2 > (0i64)) as i64) == 0 { break; } st = ((if (((s1_2 & (1i64)) == (1i64)) as i64) != 0 { (s1_0.wrapping_mul(s1_1) % a0) } else { s1_0 }), (s1_1.wrapping_mul(s1_1) % a0), s1_2.wrapping_shr(((1i64) as u32) & 63)); } st }.0; for item1 in ((0i64)..(1i64)) { acc1 = (if ((((acc1 == (1i64)) as i64) != 0 || ((acc1 == a0.wrapping_sub((1i64))) as i64) != 0) as i64) != 0 { (1i64) } else { { let mut st = (acc1, (1i64), (if ((acc1 == a0.wrapping_sub((1i64))) as i64) != 0 { (1i64) } else { (0i64) })); loop { #[allow(unused_variables)] let (s2_0, s2_1, s2_2) = st; if (((((a0.wrapping_sub((1i64)) & (0i64).wrapping_sub(a0.wrapping_sub((1i64)))) > (2i64).wrapping_mul(s2_1)) as i64) != 0 && ((s2_2 == (0i64)) as i64) != 0) as i64) == 0 { break; } st = ((s2_0.wrapping_mul(s2_0) % a0), (2i64).wrapping_mul(s2_1), (if (((s2_0.wrapping_mul(s2_0) % a0) == a0.wrapping_sub((1i64))) as i64) != 0 { (1i64) } else { (0i64) })); } st }.2 }); } acc1 } }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let a0: i64 = t.next().unwrap().parse().unwrap();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let w: i64 = t.next().unwrap().parse().unwrap(); assert_eq!(f(a0, a1), w, "row {n}");
    }
    println!("ok {n}");
}
