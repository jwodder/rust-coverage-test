use rstest::rstest;
use rust_coverage_test::modinverse;

#[rstest]
#[case(3, 5, Some(2))]
#[case(-2, 5, Some(2))]
#[case(3, -5, Some(2))]
#[case(2, 6, None)]
#[case(0, 3, None)]
fn test_modinverse(#[case] a: i64, #[case] n: i64, #[case] inv: Option<i64>) {
    assert_eq!(modinverse(a, n), inv);
}
