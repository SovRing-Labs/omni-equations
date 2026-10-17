# CONCEPTS — signature and definition of every concept

Types: `int` = i64, `f32list` = list of f32, `list` = list of i64. The equation is the s-expression the Rust was emitted from (`emit/index.json`);
very long equations (more than 1,500 characters) are only in `emit/index.json`. Language: `TECHNICAL-DISCLOSURE.md` section 2.

## `absmean_quant`
Arguments: `f32list`. Source: `emit/absmean_quant.rs`.

```
(map a0 (max (min (round (/ (idx a0 item1) (max (/ (fold (range 0 (len a0)) 0 (+ acc2.0 (abs (idx a0 item2)))) (len a0)) 1e-05f))) 1) -1))
```

## `clock_read`
Arguments: `int, int, int`. Source: `emit/clock_read.rs`.

```
(fold (range 0 105) -1 (sel (and (== acc1.0 -1) (and (== (% item1 3) a0) (and (== (% item1 5) a1) (== (% item1 7) a2)))) item1 acc1.0))
```

## `crt2`
Arguments: `int, int, int, int`. Source: `emit/crt2.rs`.

```
(fold (range 0 (* a1 a3)) -1 (sel (and (== acc1.0 -1) (and (== (% item1 a1) a0) (== (% item1 a3) a2))) item1 acc1.0))
```

## `mod_barrett`
Arguments: `int, int`. Source: `emit/mod_barrett.rs`.

```
(sel (>= (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) (* 2 a1)) (- (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) (* 2 a1)) (sel (>= (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) a1) (- (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) a1) (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1))))
```

## `hamming`
Arguments: `int, int`. Source: `emit/hamming.rs`.

```
(fold (range 0 31) 0 (+ acc1.0 (& (>> (^ a0 a1) item1) 1)))
```

## `mixed_radix`
Arguments: `int, int, int, int, int`. Source: `emit/mixed_radix.rs`.

```
(+ a0 (* a3 (+ a1 (* a4 a2))))
```

## `isqrt`
Arguments: `int`. Source: `emit/isqrt.rs`.

```
(get0 (while0.1 (tup a0) (> (* st1.0 st1.0) a0) (tup (/ (+ st1.0 (/ a0 st1.0)) 2))))
```

## `softmax`
Arguments: `f32list`. Source: `emit/softmax.rs`.

```
(map a0 (/ (exp (- (idx a0 item1) (maxall a0))) (fold (range 0 (len a0)) 0 (+ acc2.0 (exp (- (idx a0 item2) (maxall a0)))))))
```

## `period`
Arguments: `list`. Source: `emit/period.rs`.

```
(fold (range 1 (+ (len a0) 1)) 0 (sel (and (== acc1.0 0) (fold (range 0 (- (len a0) item1)) 1 (sel (== (idx a0 item2) (idx a0 (+ item2 item1))) acc2.0 0))) item1 acc1.0))
```

## `fib`
Arguments: `int`. Source: `emit/fib.rs`.

```
(get0 (while0.1 (tup 0 1 a0) (> st1.2 0) (tup st1.1 (+ st1.0 st1.1) (- st1.2 1))))
```

## `mont_mul`
Arguments: `int, int, int`. Source: `emit/mont_mul.rs`.

```
(sel (>= (>> (+ (* a0 a1) (* (& (* (& (* a0 a1) 65535) (- 65536 (fold (range 0 5) 1 (& (* acc1.0 (- 2 (* a2 acc1.0))) 65535)))) 65535) a2)) 16) a2) (- (>> (+ (* a0 a1) (* (& (* (& (* a0 a1) 65535) (- 65536 (fold (range 0 5) 1 (& (* acc1.0 (- 2 (* a2 acc1.0))) 65535)))) 65535) a2)) 16) a2) (>> (+ (* a0 a1) (* (& (* (& (* a0 a1) 65535) (- 65536 (fold (range 0 5) 1 (& (* acc1.0 (- 2 (* a2 acc1.0))) 65535)))) 65535) a2)) 16))
```

## `mod_inverse`
Arguments: `int, int`. Source: `emit/mod_inverse.rs`.

```
(% (+ (% (get0 (while0.1 (tup 1 0 a0 a1) (> st1.3 0) (tup st1.1 (- st1.0 (* (/ st1.2 st1.3) st1.1)) st1.3 (- st1.2 (* (/ st1.2 st1.3) st1.3))))) a1) a1) a1)
```

## `modexp`
Arguments: `int, int, int`. Source: `emit/modexp.rs`.

```
(get0 (while0.1 (tup (% 1 a2) (% a0 a2) a1) (> st1.2 0) (tup (sel (== (& st1.2 1) 1) (% (* st1.0 st1.1) a2) st1.0) (% (* st1.1 st1.1) a2) (>> st1.2 1))))
```

## `base_extension`
Arguments: `int, int, int, int, int`. Source: `emit/base_extension.rs`.

```
(% (+ a0 (* a1 (% (* (% (+ (- a2 (% a0 a3)) a3) a3) (% (+ (% (get0 (while0.1 (tup 1 0 (% a1 a3) a3) (> st1.3 0) (tup st1.1 (- st1.0 (* (/ st1.2 st1.3) st1.1)) st1.3 (- st1.2 (* (/ st1.2 st1.3) st1.3))))) a3) a3) a3)) a3))) a4)
```

## `rns_mul`
Arguments: `list, list, list`. Source: `emit/rns_mul.rs`.

```
(map a0 (% (* (idx a0 item1) (idx a1 item1)) (idx a2 item1)))
```

## `rns_extend`
Arguments: `list, list, list`. Source: `emit/rns_extend.rs`.

```
(fold (range 0 1) (fold (range 0 (len a1)) (range 0 0) (push acc1.0 (fold (range 0 item1) (idx a0 item1) (% (* (% (+ (- acc2.0 (% (idx acc1.0 item2) (idx a1 item1))) (idx a1 item1)) (idx a1 item1)) (% (+ (% (get0 (while0.3 (tup 1 0 (% (idx a1 item2) (idx a1 item1)) (idx a1 item1)) (> st3.3 0) (tup st3.1 (- st3.0 (* (/ st3.2 st3.3) st3.1)) st3.3 (- st3.2 (* (/ st3.2 st3.3) st3.3))))) (idx a1 item1)) (idx a1 item1)) (idx a1 item1))) (idx a1 item1))))) (map a2 (fold (range 0 (len a1)) 0 (% (+ (* acc3.0 (% (idx a1 (- (- (len a1) 1) item3)) (idx a2 item2))) (idx acc1.0 (- (- (len a1) 1) item3))) (idx a2 item2)))))
```

## `rns_mont_mul`
Arguments: `list, list, list, list, list, list, list, list`. Source: `emit/rns_mont_mul.rs`.

```
(fold (range 0 1) (map a6 (% (* (% (* (idx a0 item1) (idx a2 item1)) (idx a6 item1)) (% (- (idx a6 item1) (% (+ (% (get0 (while0.2 (tup 1 0 (% (idx a4 item1) (idx a6 item1)) (idx a6 item1)) (> st2.3 0) (tup st2.1 (- st2.0 (* (/ st2.2 st2.3) st2.1)) st2.3 (- st2.2 (* (/ st2.2 st2.3) st2.3))))) (idx a6 item1)) (idx a6 item1)) (idx a6 item1))) (idx a6 item1))) (idx a6 item1))) (fold (range 0 1) (fold (range 0 (len a6)) (range 0 0) (push acc2.0 (fold (range 0 item2) (idx acc1.0 item2) (% (* (% (+ (- acc3.0 (% (idx acc2.0 item3) (idx a6 item2))) (idx a6 item2)) (idx a6 item2)) (% (+ (% (get0 (while0.4 (tup 1 0 (% (idx a6 item3) (idx a6 item2)) (idx a6 item2)) (> st4.3 0) (tup st4.1 (- st4.0 (* (/ st4.2 st4.3) st4.1)) st4.3 (- st4.2 (* (/ st4.2 st4.3) st4.3))))) (idx a6 item2)) (idx a6 item2)) (idx a6 item2))) (idx a6 item2))))) (map a7 (% (* (% (+ (% (* (idx a1 item3) (idx a3 item3)) (idx a7 item3)) (% (* (fold (range 0 (len a6)) 0 (% (+ (* acc4.0 (% (idx a6 (- (- (len a6) 1) item4)) (idx a7 item3))) (idx acc2.0 (- (- (len a6) 1) item4))) (idx a7 item3))) (idx a5 item3)) (idx a7 item3))) (idx a7 item3)) (% (+ (% (get0 (while0.4 (tup 1 0 (fold (range 0 (len a6)) (% 1 (idx a7 item3)) (% (* acc4.0 (% (idx a6 item4) (idx a7 item3))) (idx a7 item3))) (idx a7 item3)) (> st4.3 0) (tup st4.1 (- st4.0 (* (/ st4.2 st4.3) st4.1)) st4.3 (- st4.2 (* (/ st4.2 st4.3) st4.3))))) (idx a7 item3)) (idx a7 item3)) (idx a7 item3))) (idx a7 item3)))))
```

## `ct_reduce`
Arguments: `int, int`. Source: `emit/ct_reduce.rs`.

```
(- (- (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) (* (* 2 a1) (& (>> (- (- (* 2 a1) 1) (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1))) 63) 1))) (* a1 (& (>> (- (- a1 1) (- (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1)) (* (* 2 a1) (& (>> (- (- (* 2 a1) 1) (- a0 (* (>> (* a0 (/ 4294967296 a1)) 32) a1))) 63) 1)))) 63) 1)))
```

## `rns_sq_extend`
Arguments: `list, list, list`. Source: `emit/rns_sq_extend.rs`.

```
(fold (range 0 1) (fold (range 0 (len a1)) (range 0 0) (push acc1.0 (fold (range 0 item1) (idx (map a0 (% (* (idx a0 item2) (idx a0 item2)) (idx a1 item2))) item1) (% (* (% (+ (- acc2.0 (% (idx acc1.0 item2) (idx a1 item1))) (idx a1 item1)) (idx a1 item1)) (% (+ (% (get0 (while0.3 (tup 1 0 (% (idx a1 item2) (idx a1 item1)) (idx a1 item1)) (> st3.3 0) (tup st3.1 (- st3.0 (* (/ st3.2 st3.3) st3.1)) st3.3 (- st3.2 (* (/ st3.2 st3.3) st3.3))))) (idx a1 item1)) (idx a1 item1)) (idx a1 item1))) (idx a1 item1))))) (map a2 (fold (range 0 (len a1)) 0 (% (+ (* acc3.0 (% (idx a1 (- (- (len a1) 1) item3)) (idx a2 item2))) (idx acc1.0 (- (- (len a1) 1) item3))) (idx a2 item2)))))
```

## `rns_mont_mul_a`
Arguments: `list, list, list, list, list, list, list, list`. Source: `emit/rns_mont_mul_a.rs`.

Equation: 2,062 characters, see `emit/index.json` (`rns_mont_mul_a`).

## `matmul_mod`
Arguments: `list, list, int, int`. Source: `emit/matmul_mod.rs`.

```
(map (range 0 (* a2 a2)) (% (fold (range 0 a2) 0 (+ acc2.0 (* (idx a0 (+ (* (/ item1 a2) a2) item2)) (idx a1 (+ (* item2 a2) (% item1 a2)))))) a3))
```

## `det_mod`
Arguments: `list, int, int`. Source: `emit/det_mod.rs`.

```
(% (+ (* 4 (* a2 (* a2 a2))) (sel (== a1 1) (idx a0 0) (sel (== a1 2) (- (* (idx a0 0) (idx a0 3)) (* (idx a0 1) (idx a0 2))) (+ (- (* (idx a0 0) (- (* (idx a0 4) (idx a0 8)) (* (idx a0 5) (idx a0 7)))) (* (idx a0 1) (- (* (idx a0 3) (idx a0 8)) (* (idx a0 5) (idx a0 6))))) (* (idx a0 2) (- (* (idx a0 3) (idx a0 7)) (* (idx a0 4) (idx a0 6)))))))) a2)
```

## `mr_digits`
Arguments: `list, list`. Source: `emit/mr_digits.rs`.

```
(fold (range 0 (len a1)) (range 0 0) (push acc1.0 (fold (range 0 item1) (idx a0 item1) (% (* (% (+ (- acc2.0 (% (idx acc1.0 item2) (max 1 (idx a1 item1)))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (% (+ (% (get0 (while0.3 (tup 1 0 (% (idx a1 item2) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (> st3.3 0) (tup st3.1 (- st3.0 (* (/ st3.2 st3.3) st3.1)) st3.3 (- st3.2 (* (/ st3.2 st3.3) st3.3))))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1)))) (max 1 (idx a1 item1))))))
```

## `rns_compare`
Arguments: `list, list, list`. Source: `emit/rns_compare.rs`.

```
(sum (fold (range 0 1) (fold (range 0 (len a2)) (range 0 0) (push acc1.0 (fold (range 0 item1) (idx a0 item1) (% (* (% (+ (- acc2.0 (% (idx acc1.0 item2) (max 1 (idx a2 item1)))) (max 1 (idx a2 item1))) (max 1 (idx a2 item1))) (% (+ (% (get0 (while0.3 (tup 1 0 (% (idx a2 item2) (max 1 (idx a2 item1))) (max 1 (idx a2 item1))) (> st3.3 0) (tup st3.1 (- st3.0 (* (/ st3.2 st3.3) st3.1)) st3.3 (- st3.2 (* (/ st3.2 st3.3) st3.3))))) (max 1 (idx a2 item1))) (max 1 (idx a2 item1))) (max 1 (idx a2 item1)))) (max 1 (idx a2 item1)))))) (fold (range 0 1) (fold (range 0 (len a2)) (range 0 0) (push acc2.0 (fold (range 0 item2) (idx a1 item2) (% (* (% (+ (- acc3.0 (% (idx acc2.0 item3) (max 1 (idx a2 item2)))) (max 1 (idx a2 item2))) (max 1 (idx a2 item2))) (% (+ (% (get0 (while0.4 (tup 1 0 (% (idx a2 item3) (max 1 (idx a2 item2))) (max 1 (idx a2 item2))) (> st4.3 0) (tup st4.1 (- st4.0 (* (/ st4.2 st4.3) st4.1)) st4.3 (- st4.2 (* (/ st4.2 st4.3) st4.3))))) (max 1 (idx a2 item2))) (max 1 (idx a2 item2))) (max 1 (idx a2 item2)))) (max 1 (idx a2 item2)))))) (push (range 0 0) (fold (range 0 (len a2)) 0 (sel (== acc3.0 0) (sel (> (idx acc1.0 (- (- (len a2) 1) item3)) (idx acc2.0 (- (- (len a2) 1) item3))) 1 (sel (< (idx acc1.0 (- (- (len a2) 1) item3)) (idx acc2.0 (- (- (len a2) 1) item3))) -1 0)) acc3.0))))))
```

## `sign_detect`
Arguments: `list, list`. Source: `emit/sign_detect.rs`.

Equation: 2,074 characters, see `emit/index.json` (`sign_detect`).

## `exact_scale`
Arguments: `list, list, int`. Source: `emit/exact_scale.rs`.

```
(fold (range 0 1) (fold (range 0 (len a1)) (range 0 0) (push acc1.0 (fold (range 0 item1) (idx a0 item1) (% (* (% (+ (- acc2.0 (% (idx acc1.0 item2) (max 1 (idx a1 item1)))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (% (+ (% (get0 (while0.3 (tup 1 0 (% (idx a1 item2) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (> st3.3 0) (tup st3.1 (- st3.0 (* (/ st3.2 st3.3) st3.1)) st3.3 (- st3.2 (* (/ st3.2 st3.3) st3.3))))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1))) (max 1 (idx a1 item1)))) (max 1 (idx a1 item1)))))) (fold (range a2 (len a1)) (range 0 0) (push acc2.0 (fold (range 0 (- (len a1) a2)) 0 (% (+ (* acc3.0 (idx a1 (- (- (len a1) 1) item3))) (idx acc1.0 (- (- (len a1) 1) item3))) (max 1 (idx a1 item2)))))))
```

## `newton_reciprocal`
Arguments: `int`. Source: `emit/newton_reciprocal.rs`.

```
(fold (range 0 1) (fold (range 0 32) 0 (+ acc1.0 (sel (> (>> a0 item1) 0) 1 0))) (fold (range 0 4) (fold (range 0 6) (>> 2147483648 acc1.0) (>> (* acc2.0 (- 4294967296 (* a0 acc2.0))) 31)) (sel (<= (* a0 (+ acc2.0 1)) 2147483648) (+ acc2.0 1) acc2.0)))
```

## `rational_reconstruction`
Arguments: `int, int`. Source: `emit/rational_reconstruction.rs`.

```
(push (push (range 0 0) (* (sel (< (get3 (while0.2 (tup a1 a0 0 1 (fold (range 0 1) (/ (- a1 1) 2) (get0 (while0.3 (tup acc3.0) (> (* st3.0 st3.0) acc3.0) (tup (/ (+ st3.0 (/ acc3.0 st3.0)) 2)))))) (> st2.1 st2.4) (tup st2.1 (- st2.0 (* (/ st2.0 st2.1) st2.1)) st2.3 (- st2.2 (* (/ st2.0 st2.1) st2.3)) st2.4))) 0) -1 1) (get1 (while0.2 (tup a1 a0 0 1 (fold (range 0 1) (/ (- a1 1) 2) (get0 (while0.3 (tup acc3.0) (> (* st3.0 st3.0) acc3.0) (tup (/ (+ st3.0 (/ acc3.0 st3.0)) 2)))))) (> st2.1 st2.4) (tup st2.1 (- st2.0 (* (/ st2.0 st2.1) st2.1)) st2.3 (- st2.2 (* (/ st2.0 st2.1) st2.3)) st2.4))))) (* (sel (< (get3 (while0.2 (tup a1 a0 0 1 (fold (range 0 1) (/ (- a1 1) 2) (get0 (while0.3 (tup acc3.0) (> (* st3.0 st3.0) acc3.0) (tup (/ (+ st3.0 (/ acc3.0 st3.0)) 2)))))) (> st2.1 st2.4) (tup st2.1 (- st2.0 (* (/ st2.0 st2.1) st2.1)) st2.3 (- st2.2 (* (/ st2.0 st2.1) st2.3)) st2.4))) 0) -1 1) (get3 (while0.2 (tup a1 a0 0 1 (fold (range 0 1) (/ (- a1 1) 2) (get0 (while0.3 (tup acc3.0) (> (* st3.0 st3.0) acc3.0) (tup (/ (+ st3.0 (/ acc3.0 st3.0)) 2)))))) (> st2.1 st2.4) (tup st2.1 (- st2.0 (* (/ st2.0 st2.1) st2.1)) st2.3 (- st2.2 (* (/ st2.0 st2.1) st2.3)) st2.4)))))
```

## `range_plan`
Arguments: `list, int`. Source: `emit/range_plan.rs`.

```
(get2 (while0.1 (tup 0 0 0) (< st1.0 (len a0)) (tup (+ st1.0 1) (+ st1.1 (- (fold (range 0 63) 0 (+ acc2.0 (sel (> (>> (idx a0 st1.0) item2) 0) 1 0))) 1)) (+ st1.2 (sel (> (+ a1 1) st1.1) 1 0)))))
```

## `gcd`
Arguments: `int, int`. Source: `emit/gcd.rs`.

```
(get0 (while0.1 (tup a0 a1) (> st1.1 0) (tup st1.1 (% st1.0 st1.1))))
```

## `mm_combine`
Arguments: `list, int, list, int`. Source: `emit/mm_combine.rs`.

```
(map a0 (+ (idx a0 item1) (* a1 (% (* (+ (- (idx a2 item1) (idx a0 item1)) (* a1 a3)) (% (+ (% (get0 (while0.2 (tup 1 0 (% a1 a3) a3) (> st2.3 0) (tup st2.1 (- st2.0 (* (/ st2.2 st2.3) st2.1)) st2.3 (- st2.2 (* (/ st2.2 st2.3) st2.3))))) a3) a3) a3)) a3))))
```

## `ext_gcd`
Arguments: `int, int`. Source: `emit/ext_gcd.rs`.

```
(push (push (push (range 0 0) (get0 (while0.1 (tup a0 a1 1 0 0 1) (> st1.1 0) (tup st1.1 (- st1.0 (* (/ st1.0 st1.1) st1.1)) st1.3 (- st1.2 (* (/ st1.0 st1.1) st1.3)) st1.5 (- st1.4 (* (/ st1.0 st1.1) st1.5)))))) (get2 (while0.1 (tup a0 a1 1 0 0 1) (> st1.1 0) (tup st1.1 (- st1.0 (* (/ st1.0 st1.1) st1.1)) st1.3 (- st1.2 (* (/ st1.0 st1.1) st1.3)) st1.5 (- st1.4 (* (/ st1.0 st1.1) st1.5)))))) (get4 (while0.1 (tup a0 a1 1 0 0 1) (> st1.1 0) (tup st1.1 (- st1.0 (* (/ st1.0 st1.1) st1.1)) st1.3 (- st1.2 (* (/ st1.0 st1.1) st1.3)) st1.5 (- st1.4 (* (/ st1.0 st1.1) st1.5))))))
```

## `solve_mod`
Arguments: `list, list, int, int`. Source: `emit/solve_mod.rs`.

Equation: 1,562 characters, see `emit/index.json` (`solve_mod`).

## `ntt`
Arguments: `list, list, int`. Source: `emit/ntt.rs`.

```
(map a0 (% (fold (range 0 (len a0)) 0 (+ acc2.0 (* (idx a0 item2) (idx a1 (% (* item2 item1) (len a0)))))) a2))
```

## `intt`
Arguments: `list, list, int`. Source: `emit/intt.rs`.

```
(map a0 (% (* (% (fold (range 0 (len a0)) 0 (+ acc2.0 (* (idx a0 item2) (idx a1 (% (- (len a0) (% (* item2 item1) (len a0))) (len a0)))))) a2) (% (+ (% (get0 (while0.2 (tup 1 0 (% (len a0) a2) a2) (> st2.3 0) (tup st2.1 (- st2.0 (* (/ st2.2 st2.3) st2.1)) st2.3 (- st2.2 (* (/ st2.2 st2.3) st2.3))))) a2) a2) a2)) a2))
```

## `polymul_negacyclic`
Arguments: `list, list, int`. Source: `emit/polymul_negacyclic.rs`.

```
(map a0 (% (+ (fold (range 0 (len a0)) 0 (+ acc2.0 (sel (>= item1 item2) (* (idx a0 item2) (idx a1 (- item1 item2))) (- 0 (* (idx a0 item2) (idx a1 (+ (- item1 item2) (len a0)))))))) (* (len a0) (* a2 a2))) a2))
```

## `miller_rabin`
Arguments: `int, int`. Source: `emit/miller_rabin.rs`.

```
(fold (range 0 1) (get0 (while0.1 (tup (% 1 a0) (% a1 a0) (/ (- a0 1) (& (- a0 1) (- 0 (- a0 1))))) (> st1.2 0) (tup (sel (== (& st1.2 1) 1) (% (* st1.0 st1.1) a0) st1.0) (% (* st1.1 st1.1) a0) (>> st1.2 1)))) (sel (or (== acc1.0 1) (== acc1.0 (- a0 1))) 1 (get2 (while0.2 (tup acc1.0 1 (sel (== acc1.0 (- a0 1)) 1 0)) (and (> (& (- a0 1) (- 0 (- a0 1))) (* 2 st2.1)) (== st2.2 0)) (tup (% (* st2.0 st2.0) a0) (* 2 st2.1) (sel (== (% (* st2.0 st2.0) a0) (- a0 1)) 1 0))))))
```

## `crt_general`
Arguments: `int, int, int, int`. Source: `emit/crt_general.rs`.

```
(fold (range 0 (/ (* a1 a3) (get0 (while0.1 (tup a1 a3) (> st1.1 0) (tup st1.1 (% st1.0 st1.1)))))) -1 (sel (and (== acc1.0 -1) (and (== (% item1 a1) a0) (== (% item1 a3) a2))) item1 acc1.0))
```

## `pollard_rho`
Arguments: `int`. Source: `emit/pollard_rho.rs`.

```
(get0 (while0.2 (tup (get2 (while0.1 (tup 2 2 1 0) (> 16 st1.3) (tup (% (+ (* st1.0 st1.0) 1) a0) (% (+ (* (% (+ (* st1.1 st1.1) 1) a0) (% (+ (* st1.1 st1.1) 1) a0)) 1) a0) (% (* st1.2 (sel (> (% (+ (* st1.0 st1.0) 1) a0) (% (+ (* (% (+ (* st1.1 st1.1) 1) a0) (% (+ (* st1.1 st1.1) 1) a0)) 1) a0)) (- (% (+ (* st1.0 st1.0) 1) a0) (% (+ (* (% (+ (* st1.1 st1.1) 1) a0) (% (+ (* st1.1 st1.1) 1) a0)) 1) a0)) (- (% (+ (* (% (+ (* st1.1 st1.1) 1) a0) (% (+ (* st1.1 st1.1) 1) a0)) 1) a0) (% (+ (* st1.0 st1.0) 1) a0)))) a0) (+ st1.3 1)))) a0) (> st2.1 0) (tup st2.1 (% st2.0 st2.1))))
```

## `shuffle_mul`
Arguments: `list, list, int, int, int`. Source: `emit/shuffle_mul.rs`.

```
(sel (== a2 0) 0 (sel (== a3 0) 0 (idx a1 (% (+ (idx a0 a2) (idx a0 a3)) (max 1 (- a4 1))))))
```

## `shuffle_div`
Arguments: `list, list, int, int, int`. Source: `emit/shuffle_div.rs`.

```
(sel (== a2 0) 0 (idx a1 (% (+ (- (idx a0 a2) (idx a0 a3)) (- a4 1)) (max 1 (- a4 1)))))
```

## `shuffle_pow`
Arguments: `list, list, int, int, int`. Source: `emit/shuffle_pow.rs`.

```
(sel (== a2 0) (sel (== a3 0) 1 0) (idx a1 (% (+ (% (* (idx a0 a2) a3) (max 1 (- a4 1))) (- a4 1)) (max 1 (- a4 1)))))
```

## `rd_step`
Arguments: `list, int, int, int, int, int, list`. Source: `emit/rd_step.rs`.

Equation: 5,586 characters, see `emit/index.json` (`rd_step`).

## `rd_packet`
Arguments: `list, list`. Source: `emit/rd_packet.rs`.

```
(get2 (while0.1 (tup 0 (- 127 (>> (sel (> (len a0) 0) (idx a0 0) 0) 1)) 128 (sel (> (len a0) 0) (idx a0 0) 0) 1 0 0) (or (< st1.0 (len a1)) (<= st1.2 8388608)) (tup (sel (<= st1.2 8388608) st1.0 (sel (== st1.5 0) st1.0 (+ (+ st1.0 2) (idx a1 (+ st1.0 1))))) (sel (<= st1.2 8388608) (& (+ (<< st1.1 8) (- 255 (& (>> (| (<< st1.3 8) (sel (< st1.4 (len a0)) (idx a0 st1.4) 0)) 1) 255))) 2147483647) (sel (== st1.5 0) st1.1 (- st1.1 (* (max 1 (/ st1.2 (max 1 (idx a1 st1.0)))) (- (idx a1 st1.0) (idx a1 (+ (+ st1.0 2) st1.6))))))) (sel (<= st1.2 8388608) (<< st1.2 8) (sel (== st1.5 0) st1.2 (sel (> (sel (> st1.6 0) (idx a1 (+ (+ st1.0 2) (- st1.6 1))) 0) 0) (* (max 1 (/ st1.2 (max 1 (idx a1 st1.0)))) (- (idx a1 (+ (+ st1.0 2) st1.6)) (sel (> st1.6 0) (idx a1 (+ (+ st1.0 2) (- st1.6 1))) 0))) (- st1.2 (* (max 1 (/ st1.2 (max 1 (idx a1 st1.0)))) (- (idx a1 st1.0) (idx a1 (+ (+ st1.0 2) st1.6)))))))) (sel (<= st1.2 8388608) (sel (< st1.4 (len a0)) (idx a0 st1.4) 0) (sel (== st1.5 0) st1.3 st1.3)) (sel (<= st1.2 8388608) (sel (< st1.4 (len a0)) (+ st1.4 1) st1.4) (sel (== st1.5 0) st1.4 st1.4)) (sel (<= st1.2 8388608) st1.5 (sel (== st1.5 0) 1 0)) (sel (<= st1.2 8388608) st1.6 (sel (== st1.5 0) (fold (range 0 (idx a1 (+ st1.0 1))) 0 (+ acc2.0 (sel (<= (idx a1 (+ (+ st1.0 2) item2)) (- (idx a1 st1.0) (min (+ (/ st1.1 (max 1 (/ st1.2 (max 1 (idx a1 st1.0))))) 1) (idx a1 st1.0)))) 1 0))) st1.6)))))
```

## `vp9_butterfly`
Arguments: `int, int, int`. Source: `emit/vp9_butterfly.rs`.

Equation: 2,733 characters, see `emit/index.json` (`vp9_butterfly`).

## `idct4`
Arguments: `list`. Source: `emit/idct4.rs`.

```
(fold (range 0 1) (fold (range 0 4) (map (range 0 8) (sel (< item3 4) (idx a0 item3) 0)) (map (range 0 8) (sel (== item2 0) (sel (== item3 0) (idx acc2.0 0) (sel (== item3 1) (idx acc2.0 2) (sel (== item3 2) (idx acc2.0 1) (sel (== item3 3) (idx acc2.0 3) (idx acc2.0 item3))))) (sel (== item2 1) (sel (== item3 0) (>> (+ (+ (* (idx acc2.0 0) 11585) (* (idx acc2.0 1) 11585)) 8192) 14) (sel (== item3 1) (>> (+ (- (* (idx acc2.0 0) 11585) (* (idx acc2.0 1) 11585)) 8192) 14) (idx acc2.0 item3))) (sel (== item2 2) (sel (== item3 2) (>> (+ (- (* (idx acc2.0 2) 6270) (* (idx acc2.0 3) 15137)) 8192) 14) (sel (== item3 3) (>> (+ (+ (* (idx acc2.0 2) 15137) (* (idx acc2.0 3) 6270)) 8192) 14) (idx acc2.0 item3))) (sel (== item2 3) (sel (== item3 0) (+ (idx acc2.0 0) (idx acc2.0 3)) (sel (== item3 1) (+ (idx acc2.0 1) (idx acc2.0 2)) (sel (== item3 2) (- (idx acc2.0 1) (idx acc2.0 2)) (sel (== item3 3) (- (idx acc2.0 0) (idx acc2.0 3)) (idx acc2.0 item3))))) (idx acc2.0 item3))))))) (map (range 0 4) (idx acc1.0 item2)))
```

## `idct8`
Arguments: `list`. Source: `emit/idct8.rs`.

Equation: 2,484 characters, see `emit/index.json` (`idct8`).

## `idct16`
Arguments: `list`. Source: `emit/idct16.rs`.

Equation: 6,226 characters, see `emit/index.json` (`idct16`).

## `idct32`
Arguments: `list`. Source: `emit/idct32.rs`.

Equation: 15,268 characters, see `emit/index.json` (`idct32`).

## `iadst4`
Arguments: `list`. Source: `emit/iadst4.rs`.

```
(fold (range 0 1) (fold (range 0 1) (map (range 0 8) (sel (< item3 4) (idx a0 item3) 0)) (map (range 0 8) (sel (== item2 0) (sel (== item3 0) (>> (+ (+ (+ (+ (* 5283 (idx acc2.0 0)) (* 15212 (idx acc2.0 2))) (* 9929 (idx acc2.0 3))) (* 13377 (idx acc2.0 1))) 8192) 14) (sel (== item3 1) (>> (+ (+ (- (- (* 9929 (idx acc2.0 0)) (* 5283 (idx acc2.0 2))) (* 15212 (idx acc2.0 3))) (* 13377 (idx acc2.0 1))) 8192) 14) (sel (== item3 2) (>> (+ (* 13377 (+ (- (idx acc2.0 0) (idx acc2.0 2)) (idx acc2.0 3))) 8192) 14) (sel (== item3 3) (>> (+ (- (+ (+ (+ (* 5283 (idx acc2.0 0)) (* 15212 (idx acc2.0 2))) (* 9929 (idx acc2.0 3))) (- (- (* 9929 (idx acc2.0 0)) (* 5283 (idx acc2.0 2))) (* 15212 (idx acc2.0 3)))) (* 13377 (idx acc2.0 1))) 8192) 14) (idx acc2.0 item3))))) (idx acc2.0 item3)))) (map (range 0 4) (idx acc1.0 item2)))
```

## `iadst8`
Arguments: `list`. Source: `emit/iadst8.rs`.

Equation: 3,346 characters, see `emit/index.json` (`iadst8`).

## `iadst16`
Arguments: `list`. Source: `emit/iadst16.rs`.

Equation: 8,010 characters, see `emit/index.json` (`iadst16`).

## `inv_txfm_2d`
Arguments: `list, int, int`. Source: `emit/inv_txfm_2d.rs`.

Equation: 91,629 characters, see `emit/index.json` (`inv_txfm_2d`).

## `vp9_intra_dc`
Arguments: `list, list, int, int, int, int`. Source: `emit/vp9_intra_dc.rs`.

```
(fold (range 0 (len a0)) (range 0 0) (fold (range 0 (len a0)) acc1.0 (push acc2.0 (sel (and a2 a3) (>> (+ (+ (sum a0) (sum a1)) (len a0)) (+ a4 1)) (sel a3 (>> (+ (sum a1) (<< 1 (- a4 1))) a4) (sel a2 (>> (+ (sum a0) (<< 1 (- a4 1))) a4) (<< 1 (- a5 1))))))))
```

## `vp9_intra_tm`
Arguments: `list, list, int, int`. Source: `emit/vp9_intra_tm.rs`.

```
(fold (range 0 (len a0)) (range 0 0) (fold (range 0 (len a0)) acc1.0 (push acc2.0 (max 0 (min (- (<< 1 a3) 1) (- (+ (idx a0 item2) (idx a1 item1)) a2))))))
```

## `vp9_intra_v`
Arguments: `list, list, int`. Source: `emit/vp9_intra_v.rs`.

```
(fold (range 0 (len a0)) (range 0 0) (fold (range 0 (len a0)) acc1.0 (push acc2.0 (idx a0 item2))))
```

## `vp9_intra_h`
Arguments: `list, list, int`. Source: `emit/vp9_intra_h.rs`.

```
(fold (range 0 (len a0)) (range 0 0) (fold (range 0 (len a0)) acc1.0 (push acc2.0 (idx a1 item1))))
```

## `vp9_lf4`
Arguments: `list, int, int, int, int`. Source: `emit/vp9_lf4.rs`.

Equation: 6,419 characters, see `emit/index.json` (`vp9_lf4`).

## `vp9_lf8`
Arguments: `list, int, int, int, int`. Source: `emit/vp9_lf8.rs`.

Equation: 9,975 characters, see `emit/index.json` (`vp9_lf8`).

## `vp9_lf16`
Arguments: `list, int, int, int, int`. Source: `emit/vp9_lf16.rs`.

Equation: 27,779 characters, see `emit/index.json` (`vp9_lf16`).

## `vp9_subpel8`
Arguments: `list, int, int, int, int, list, int`. Source: `emit/vp9_subpel8.rs`.

```
(sel (and (and (== (len a0) (* (+ a2 7) (+ a1 7))) (== (len a5) 128)) (and (and (and (>= a3 0) (< a3 16)) (and (>= a4 0) (< a4 16))) (and (and (> a1 0) (<= a1 32)) (and (> a2 0) (<= a2 32))))) (fold (range 0 1) (fold (range 0 (+ a2 7)) (range 0 0) (fold (range 0 a1) acc2.0 (push acc3.0 (max 0 (min (- (<< 1 a6) 1) (>> (+ (fold (range 0 8) 0 (+ acc4.0 (* (idx a5 (+ (* a3 8) item4)) (idx a0 (+ (* item2 (+ a1 7)) (+ item3 item4)))))) 64) 7)))))) (fold (range 0 a2) (range 0 0) (fold (range 0 a1) acc2.0 (push acc3.0 (max 0 (min (- (<< 1 a6) 1) (>> (+ (fold (range 0 8) 0 (+ acc4.0 (* (idx a5 (+ (* a4 8) item4)) (idx acc1.0 (+ (* (+ item2 item4) a1) item3))))) 64) 7))))))) (range 0 0))
```

## `shuffle_pow_frac`
Arguments: `list, list, int, int, int, int`. Source: `emit/shuffle_pow_frac.rs`.

```
(fold (range 0 1) (* a3 (fold (range 0 1) (- a5 1) (% (+ (% (get0 (while0.1 (tup 1 0 a4 acc1.0) (> st1.3 0) (tup st1.1 (- st1.0 (* (/ st1.2 st1.3) st1.1)) st1.3 (- st1.2 (* (/ st1.2 st1.3) st1.3))))) acc1.0) acc1.0) acc1.0))) (sel (== a2 0) (sel (== acc1.0 0) 1 0) (idx a1 (% (+ (% (* (idx a0 a2) acc1.0) (max 1 (- a5 1))) (- a5 1)) (max 1 (- a5 1))))))
```

## `ring_exp`
Arguments: `int`. Source: `emit/ring_exp.rs`.

```
(push (push (range 0 0) (fold (range 0 14) 0 (+ (sel (>= (* acc1.0 (- 0 (- a0 (/ (* (/ (/ (* a0 17473979) 12112039) 12112039) 101685723302359) 12112039)))) 0) (/ (* acc1.0 (- 0 (- a0 (/ (* (/ (/ (* a0 17473979) 12112039) 12112039) 101685723302359) 12112039)))) 12112039) (- (/ (+ (* acc1.0 (- 0 (- a0 (/ (* (/ (/ (* a0 17473979) 12112039) 12112039) 101685723302359) 12112039)))) 1) 12112039) 1)) (idx (push (push (push (push (push (push (push (push (push (push (push (push (push (push (range 0 0) 12112039) 12112039) 6056020) 2018673) 504668) 100934) 16822) 2403) 300) 33) 3) 0) 0) 0) (- 13 item1))))) (/ (/ (* a0 17473979) 12112039) 12112039))
```

## `ring_ln`
Arguments: `int`. Source: `emit/ring_ln.rs`.

Equation: 2,750 characters, see `emit/index.json` (`ring_ln`).

## `celt_energy_pred`
Arguments: `list, list, int, int, int`. Source: `emit/celt_energy_pred.rs`.

```
(sel (== (len a1) (len a0)) (map (range 0 (len a0)) (+ (+ (>> (* a2 (max (idx a1 item1) a4)) 15) (fold (range 0 item1) 0 (+ acc2.0 (- (* (idx a0 item2) 256) (>> (* a3 (* (idx a0 item2) 256)) 15))))) (* (idx a0 item1) 256))) (range 0 0))
```

## `silk_lpc_synth`
Arguments: `list, list, int, list`. Source: `emit/silk_lpc_synth.rs`.

```
(sel (and (== (len a0) (len a3)) (> (len a1) 0)) (fold (range 0 1) (fold (range 0 (len a1)) (map (range 0 (len a0)) (idx a0 item3)) (push acc2.0 (>> (+ (+ (* a2 (idx a1 item2)) (* 16 (fold (range 0 (len a3)) 0 (+ acc3.0 (* (idx acc2.0 (- (+ (len a3) item2) (+ item3 1))) (idx a3 item3)))))) 32768) 16))) (map (range 0 (len a1)) (idx acc1.0 (+ item2 (len a3))))) (range 0 0))
```

## `ring_softmax`
Arguments: `list`. Source: `emit/ring_softmax.rs`.

Equation: 2,361 characters, see `emit/index.json` (`ring_softmax`).
