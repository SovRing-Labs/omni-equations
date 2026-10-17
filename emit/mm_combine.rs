#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: i64, a2: &[i64], a3: i64) -> Vec<i64> { (0..(a0.len() as i64)).map(|item1: i64| a0[(item1) as usize].wrapping_add(a1.wrapping_mul((a2[(item1) as usize].wrapping_sub(a0[(item1) as usize]).wrapping_add(a1.wrapping_mul(a3)).wrapping_mul((({ let mut st = ((1i64), (0i64), (a1 % a3), a3); loop { #[allow(unused_variables)] let (s2_0, s2_1, s2_2, s2_3) = st; if ((s2_3 > (0i64)) as i64) == 0 { break; } st = (s2_1, s2_0.wrapping_sub((s2_2 / s2_3).wrapping_mul(s2_1)), s2_3, s2_2.wrapping_sub((s2_2 / s2_3).wrapping_mul(s2_3))); } st }.0 % a3).wrapping_add(a3) % a3)) % a3)))).collect::<Vec<_>>() }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let n2: usize = t.next().unwrap().parse().unwrap(); let v2: Vec<i64> = (0..n2).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a3: i64 = t.next().unwrap().parse().unwrap();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(&v0, a1, &v2, a3), w, "row {n}");
    }
    println!("ok {n}");
}
