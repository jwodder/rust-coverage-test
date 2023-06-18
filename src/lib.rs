/// `modinverse(a, n)` returns the [modular multiplicative
/// inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse) of
/// `a` *modulo* `n`, i.e., the smallest positive integer `x` such that `(a *
/// x) % n == 1`.  `None` is returned if `a` is not relatively prime to `n`.
pub fn modinverse(a: i64, n: i64) -> Option<i64> {
    let (mut upper, mut uc) = (n.abs(), 0);
    let (mut lower, mut lc) = (a.rem_euclid(upper), 1);
    while lower > 1 {
        let d = upper.div_euclid(lower);
        let m = upper.rem_euclid(lower);
        (upper, uc, lower, lc) = (lower, lc, m, uc - lc * d);
    }
    if lower == 1 {
        Some(lc.rem_euclid(n.abs()))
    } else {
        None
    }
}

/// # Doctest
///
/// ```rust
/// # use rust_coverage_test::another_function;
/// let x = another_function().unwrap();
/// ```
pub fn another_function() -> Option<i32> {
    Some(1)
}
