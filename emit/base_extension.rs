#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i64 { (a0.wrapping_add(a1.wrapping_mul(((a2.wrapping_sub((a0 % a3)).wrapping_add(a3) % a3).wrapping_mul((({ let mut st = ((1i64), (0i64), (a1 % a3), a3); loop { #[allow(unused_variables)] let (s1_0, s1_1, s1_2, s1_3) = st; if ((s1_3 > (0i64)) as i64) == 0 { break; } st = (s1_1, s1_0.wrapping_sub((s1_2 / s1_3).wrapping_mul(s1_1)), s1_3, s1_2.wrapping_sub((s1_2 / s1_3).wrapping_mul(s1_3))); } st }.0 % a3).wrapping_add(a3) % a3)) % a3))) % a4) }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let a0: i64 = t.next().unwrap().parse().unwrap();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let a3: i64 = t.next().unwrap().parse().unwrap();
        let a4: i64 = t.next().unwrap().parse().unwrap();
        let w: i64 = t.next().unwrap().parse().unwrap(); assert_eq!(f(a0, a1, a2, a3, a4), w, "row {n}");
    }
    println!("ok {n}");
}
