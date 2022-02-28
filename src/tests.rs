use crate::logic::calc;

#[test]
fn two_terms() {
    assert_eq!(calc("5 8 +").unwrap(), 13);
    assert_eq!(calc("5 8 *").unwrap(), 40);
    assert_eq!(calc("5 8 -").unwrap(), 3);
    assert_eq!(calc("8 5 -").unwrap(), -3);
}

#[test]
fn three_terms() {
    assert_eq!(calc("1 5 7 + -").unwrap(), 11);
    assert_eq!(calc("1 5 + 7 -").unwrap(), 1);
    assert_eq!(calc("4 5 6 * -").unwrap(), 26);
    assert_eq!(calc("4 5 * 6 -").unwrap(), -14);
}

#[test]
#[should_panic]
fn invalid_token() {
    calc("5 8 ^").unwrap();
}

#[test]
#[should_panic]
fn too_few_arguments() {
    calc("5 +").unwrap();
}

#[test]
#[should_panic]
fn too_many_arguments() {
    calc("5 6 7 *").unwrap();
}
