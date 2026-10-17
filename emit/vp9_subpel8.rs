#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: &[i64], a1: i64, a2: i64, a3: i64, a4: i64, a5: &[i64], a6: i64) -> Vec<i64> { (if (((((((a0.len() as i64) == a2.wrapping_add((7i64)).wrapping_mul(a1.wrapping_add((7i64)))) as i64) != 0 && (((a5.len() as i64) == (128i64)) as i64) != 0) as i64) != 0 && ((((((((a3 >= (0i64)) as i64) != 0 && ((a3 < (16i64)) as i64) != 0) as i64) != 0 && ((((a4 >= (0i64)) as i64) != 0 && ((a4 < (16i64)) as i64) != 0) as i64) != 0) as i64) != 0 && ((((((a1 > (0i64)) as i64) != 0 && ((a1 <= (32i64)) as i64) != 0) as i64) != 0 && ((((a2 > (0i64)) as i64) != 0 && ((a2 <= (32i64)) as i64) != 0) as i64) != 0) as i64) != 0) as i64) != 0) as i64) != 0 { { let mut acc1: Vec<i64> = { let mut acc2: Vec<i64> = (((0i64))..((0i64))).collect::<Vec<i64>>(); for item2 in ((0i64)..a2.wrapping_add((7i64))) { acc2 = { let mut acc3: Vec<i64> = acc2; for item3 in ((0i64)..a1) { acc3 = { let mut v = acc3.to_vec(); v.push((0i64).max((1i64).wrapping_shl((a6 as u32) & 63).wrapping_sub((1i64)).min({ let mut acc4: i64 = (0i64); for item4 in ((0i64)..(8i64)) { acc4 = acc4.wrapping_add(a5[(a3.wrapping_mul((8i64)).wrapping_add(item4)) as usize].wrapping_mul(a0[(item2.wrapping_mul(a1.wrapping_add((7i64))).wrapping_add(item3.wrapping_add(item4))) as usize])); } acc4 }.wrapping_add((64i64)).wrapping_shr(((7i64) as u32) & 63)))); v }; } acc3 }; } acc2 }; for item1 in ((0i64)..(1i64)) { acc1 = { let mut acc2: Vec<i64> = (((0i64))..((0i64))).collect::<Vec<i64>>(); for item2 in ((0i64)..a2) { acc2 = { let mut acc3: Vec<i64> = acc2; for item3 in ((0i64)..a1) { acc3 = { let mut v = acc3.to_vec(); v.push((0i64).max((1i64).wrapping_shl((a6 as u32) & 63).wrapping_sub((1i64)).min({ let mut acc4: i64 = (0i64); for item4 in ((0i64)..(8i64)) { acc4 = acc4.wrapping_add(a5[(a4.wrapping_mul((8i64)).wrapping_add(item4)) as usize].wrapping_mul(acc1[(item2.wrapping_add(item4).wrapping_mul(a1).wrapping_add(item3)) as usize])); } acc4 }.wrapping_add((64i64)).wrapping_shr(((7i64) as u32) & 63)))); v }; } acc3 }; } acc2 }; } acc1 } } else { (((0i64))..((0i64))).collect::<Vec<i64>>() }) }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let n0: usize = t.next().unwrap().parse().unwrap(); let v0: Vec<i64> = (0..n0).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a1: i64 = t.next().unwrap().parse().unwrap();
        let a2: i64 = t.next().unwrap().parse().unwrap();
        let a3: i64 = t.next().unwrap().parse().unwrap();
        let a4: i64 = t.next().unwrap().parse().unwrap();
        let n5: usize = t.next().unwrap().parse().unwrap(); let v5: Vec<i64> = (0..n5).map(|_| t.next().unwrap().parse().unwrap()).collect();
        let a6: i64 = t.next().unwrap().parse().unwrap();
        let m: usize = t.next().unwrap().parse().unwrap(); let w: Vec<i64> = (0..m).map(|_| t.next().unwrap().parse().unwrap()).collect(); assert_eq!(f(&v0, a1, a2, a3, a4, &v5, a6), w, "row {n}");
    }
    println!("ok {n}");
}
