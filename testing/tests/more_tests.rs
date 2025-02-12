use testing::*;

#[test]
fn test_integration() {
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}