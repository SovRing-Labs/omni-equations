#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: &[i64], a2: i64, a3: &[i64]) -> Vec<i64> { (if (((((a0.len() as i64) == (a3.len() as i64)) as i64) != 0 && (((a1.len() as i64) > (0i64)) as i64) != 0) as i64) != 0 { { let mut acc1: Vec<i64> = { let mut acc2: Vec<i64> = ((0i64)..(a0.len() as i64)).map(|item3: i64| a0[(item3) as usize]).collect::<Vec<_>>(); for item2 in ((0i64)..(a1.len() as i64)) { acc2 = { let mut v = acc2.to_vec(); v.push(a2.wrapping_mul(a1[(item2) as usize]).wrapping_add((16i64).wrapping_mul({ let mut acc3: i64 = (0i64); for item3 in ((0i64)..(a3.len() as i64)) { acc3 = acc3.wrapping_add(acc2[((a3.len() as i64).wrapping_add(item2).wrapping_sub(item3.wrapping_add((1i64)))) as usize].wrapping_mul(a3[(item3) as usize])); } acc3 })).wrapping_add((32768i64)).wrapping_shr(((16i64) as u32) & 63)); v }; } acc2 }; for item1 in ((0i64)..(1i64)) { acc1 = ((0i64)..(a1.len() as i64)).map(|item2: i64| acc1[(item2.wrapping_add((a3.len() as i64))) as usize]).collect::<Vec<_>>(); } acc1 } } else { (((0i64))..((0i64))).collect::<Vec<i64>>() }) }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let n1: usize = t.next().unwrap().parse().unwrap(); let v1: Vec<i64> = (0..n1).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let n3: usize = t.next().unwrap().parse().unwrap(); let v3: Vec<i64> = (0..n3).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(&v0, &v1, a2, &v3), w, "row {n}");
    }
    println!("ok {n}");
}
