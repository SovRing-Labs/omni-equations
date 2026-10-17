#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64, a1: i64, a2: i64) -> i64 { { let mut st = (((1i64) % a2), (a0 % a2), a1); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2) = st; if ((s1_2 > (0i64)) as i64) == 0 { break; } st = ((if (((s1_2 & (1i64)) == (1i64)) as i64) != 0 { (s1_0.wrapping_mul(s1_1) % a2) } else { s1_0 }), (s1_1.wrapping_mul(s1_1) % a2), s1_2.wrapping_shr(((1i64) as u32) & 63)); } st }.0 }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let a0: i64 = t.next().unwrap().parse().unwrap();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let w: i64 = t.next().unwrap().parse().unwrap(); assert_eq!(f(a0, a1, a2), w, "row {n}");
    }
    println!("ok {n}");
}
