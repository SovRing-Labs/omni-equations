#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: &[i64], a2: i64) -> Vec<i64> { (0..(a0.len() as i64)).map(|item1: i64| ({ let mut acc2: i64 = (0i64); for item2 in ((0i64)..(a0.len() as i64)) { acc2 = acc2.wrapping_add((if ((item1 >= item2) as i64) != 0 { a0[(item2) as usize].wrapping_mul(a1[(item1.wrapping_sub(item2)) as usize]) } else { (0i64).wrapping_sub(a0[(item2) as usize].wrapping_mul(a1[(item1.wrapping_sub(item2).wrapping_add((a0.len() as i64))) as usize])) })); } acc2 }.wrapping_add((a0.len() as i64).wrapping_mul(a2.wrapping_mul(a2))) % a2)).collect::<Vec<_>>() }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let n1: usize = t.next().unwrap().parse().unwrap(); let v1: Vec<i64> = (0..n1).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(&v0, &v1, a2), w, "row {n}");
    }
    println!("ok {n}");
}
