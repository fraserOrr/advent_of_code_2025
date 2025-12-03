

use super::*;

/* */
#[test]
fn basic_test_01() {
    let result = parse_bank("987654321111111".to_string());
    assert_eq!(result,98)
}

#[test]
fn basic_test_02() {
    let result = parse_bank("811111111111119".to_string());
    assert_eq!(result,89)
}

#[test]
fn basic_test_03() {
    let result = parse_bank("234234234234278".to_string());
    assert_eq!(result,78)
}

#[test]
fn basic_test_04() {
    let result = parse_bank("818181911112111".to_string());
    assert_eq!(result,92)
}