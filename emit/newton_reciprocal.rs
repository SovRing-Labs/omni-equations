#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64) -> i64 { { let mut acc1: i64 = { let mut acc1: i64 = (0i64); for item1 in ((0i64)..(32i64)) { acc1 = acc1.wrapping_add((if ((a0.wrapping_shr((item1 as u32) & 63) > (0i64)) as i64) != 0 { (1i64) } else { (0i64) })); } acc1 }; for item1 in ((0i64)..(1i64)) { acc1 = { let mut acc2: i64 = { let mut acc2: i64 = (2147483648i64).wrapping_shr((acc1 as u32) & 63); for item2 in ((0i64)..(6i64)) { acc2 = acc2.wrapping_mul((4294967296i64).wrapping_sub(a0.wrapping_mul(acc2))).wrapping_shr(((31i64) as u32) & 63); } acc2 }; for item2 in ((0i64)..(4i64)) { acc2 = (if ((a0.wrapping_mul(acc2.wrapping_add((1i64))) <= (2147483648i64)) as i64) != 0 { acc2.wrapping_add((1i64)) } else { acc2 }); } acc2 }; } acc1 } }
fn close(a: f32, b: f32) -> bool { (a.is_nan() && b.is_nan()) || (a - b).abs() <= 1e-5 + 1e-4 * a.abs().max(b.abs()) }
fn main() {
    let txt = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap(); let mut n = 0usize;
for line in txt.lines() { let mut t = line.split_whitespace(); n += 1;
        let a0: i64 = t.next().unwrap().parse().unwrap();
        let w: i64 = t.next().unwrap().parse().unwrap(); assert_eq!(f(a0), w, "row {n}");
    }
    println!("ok {n}");
}
