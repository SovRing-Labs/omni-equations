#[allow(unused_parens, unused_variables, unused_mut, unused_braces, clippy::all)]
fn f(a0: i64, a1: i64) -> i64 { a0.wrapping_sub(a0.wrapping_mul(((4294967296i64) / a1)).wrapping_shr(((32i64) as u32) & 63).wrapping_mul(a1)).wrapping_sub((2i64).wrapping_mul(a1).wrapping_mul(((2i64).wrapping_mul(a1).wrapping_sub((1i64)).wrapping_sub(a0.wrapping_sub(a0.wrapping_mul(((4294967296i64) / a1)).wrapping_shr(((32i64) as u32) & 63).wrapping_mul(a1))).wrapping_shr(((63i64) as u32) & 63) & (1i64)))).wrapping_sub(a1.wrapping_mul((a1.wrapping_sub((1i64)).wrapping_sub(a0.wrapping_sub(a0.wrapping_mul(((4294967296i64) / a1)).wrapping_shr(((32i64) as u32) & 63).wrapping_mul(a1)).wrapping_sub((2i64).wrapping_mul(a1).wrapping_mul(((2i64).wrapping_mul(a1).wrapping_sub((1i64)).wrapping_sub(a0.wrapping_sub(a0.wrapping_mul(((4294967296i64) / a1)).wrapping_shr(((32i64) as u32) & 63).wrapping_mul(a1))).wrapping_shr(((63i64) as u32) & 63) & (1i64))))).wrapping_shr(((63i64) as u32) & 63) & (1i64)))) }
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
