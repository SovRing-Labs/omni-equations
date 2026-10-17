#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: &[i64], a2: i64, a3: i64, a4: i64, a5: i64) -> Vec<i64> { { let mut acc1: Vec<i64> = (((0i64))..((0i64))).collect::<Vec<i64>>(); for item1 in ((0i64)..(a0.len() as i64)) { acc1 = { let mut acc2: Vec<i64> = acc1; for item2 in ((0i64)..(a0.len() as i64)) { acc2 = { let mut v = acc2.to_vec(); v.push((if ((a2 != 0 && a3 != 0) as i64) != 0 { a0.iter().fold(0i64, |s, x| s.wrapping_add(*x)).wrapping_add(a1.iter().fold(0i64, |s, x| s.wrapping_add(*x))).wrapping_add((a0.len() as i64)).wrapping_shr((a4.wrapping_add((1i64)) as u32) & 63) } else { (if a3 != 0 { a1.iter().fold(0i64, |s, x| s.wrapping_add(*x)).wrapping_add((1i64).wrapping_shl((a4.wrapping_sub((1i64)) as u32) & 63)).wrapping_shr((a4 as u32) & 63) } else { (if a2 != 0 { a0.iter().fold(0i64, |s, x| s.wrapping_add(*x)).wrapping_add((1i64).wrapping_shl((a4.wrapping_sub((1i64)) as u32) & 63)).wrapping_shr((a4 as u32) & 63) } else { (1i64).wrapping_shl((a5.wrapping_sub((1i64)) as u32) & 63) }) }) })); v }; } acc2 }; } acc1 } }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let n1: usize = t.next().unwrap().parse().unwrap(); let v1: Vec<i64> = (0..n1).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let a3: i64 = t.next().unwrap().parse().unwrap();
        let a4: i64 = t.next().unwrap().parse().unwrap();
        let a5: i64 = t.next().unwrap().parse().unwrap();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(&v0, &v1, a2, a3, a4, a5), w, "row {n}");
    }
    println!("ok {n}");
}
