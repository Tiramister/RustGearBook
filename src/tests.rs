use crate::logic::calc;

#[test]
fn two_terms() {
    assert_eq!(calc("5 8 +"), 13);
    assert_eq!(calc("5 8 *"), 40);
    assert_eq!(calc("5 8 -"), 3);
    assert_eq!(calc("8 5 -"), -3);
}

#[test]
fn three_terms() {
    assert_eq!(calc("1 5 7 + -"), 11);
    assert_eq!(calc("1 5 + 7 -"), 1);
    assert_eq!(calc("4 5 6 * -"), 26);
    assert_eq!(calc("4 5 * 6 -"), -14);
}

#[test]
#[should_panic]
fn invalid_token() {
    calc("5 8 ^");
}

#[test]
#[should_panic]
fn too_few_arguments() {
    calc("5 +");
}
