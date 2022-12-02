use super::run_test;

#[test]
fn day_18_2021() {
    let (v1, v2) = run_test(2021, 18);
    assert_eq!(v1, 4140);
    assert_eq!(v2, 3993);
}
