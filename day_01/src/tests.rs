

use super::*;

#[test]
fn left_alot() {
    let result = revolve('L', 50,1000 );
    assert_eq!(result, (50,10));
}

#[test]
fn left_from_0() {
    let result = revolve('L', 0,5 );
    assert_eq!(result, (95,0));
}

#[test]
fn left_from_1() {
    let result = revolve('L', 1,5 );
    assert_eq!(result, (96,1));
}    
#[test]
fn right_from_99() {
    let result = revolve('R', 99,1 );
    assert_eq!(result, (0,1));
}
#[test]
fn right_from_99x2() {
    let result = revolve('R', 99,101 );
    assert_eq!(result, (0,2));
}
#[test]
fn right_from_99_lots() {
    let result = revolve('R', 99,1000 );
    assert_eq!(result, (99,10));
}
#[test]
fn right_from_99_to_0_lots() {
    let result = revolve('R', 99,1001 );
    assert_eq!(result, (0,11));
}
#[test]
fn right_from_0() {
    let result = revolve('R', 0,1 );
    assert_eq!(result, (1,0));
}

#[test]
fn left_0_full_loop() {
    let result = revolve('L', 0,100 );
    assert_eq!(result, (0,1));
}
#[test]
fn left_0_full_loop_add_1() {
    let result = revolve('L', 0,101 );
    assert_eq!(result, (99,1));
}
#[test]
fn left_from_0_lots() {
    let result = revolve('L', 0,400 );
    assert_eq!(result, (0,4));
}
#[test]
fn right_from_0_lots() {
    let result = revolve('R', 0,400 );
    assert_eq!(result, (0,4));
}
#[test]
fn basic_01() {
    let result = revolve('R', 50,50 );
    assert_eq!(result, (0,1));
}

#[test]
fn basic_02() {
    let result = revolve('R', 99,1 );
    assert_eq!(result, (0,1));
}

#[test]
fn basic_03() {
    let result = revolve('L', 1,1 );
    assert_eq!(result, (0,1));
}
#[test]
fn basic_04() {
    let result = revolve('L', 1,2 );
    assert_eq!(result, (99,1));
}
#[test]
fn basic_05() {
    let result = revolve('L', 99,99 );
    assert_eq!(result, (0,1));
}
#[test]
fn basic_06() {
    let result = revolve('L', 50,150);
    assert_eq!(result, (0,2));
}

