

use super::*;


#[test]
fn basic_test() {
    let result = return_invalid_id_count(11,22);
    assert_eq!(result,33)
}

#[test]
fn basic_02() {
    let result = return_invalid_id_count(998,1012);
    assert_eq!(result,2009)
}

#[test]
fn basic_03() {
    let result = return_invalid_id_count(565653,565659);
    assert_eq!(result,565656)
}

