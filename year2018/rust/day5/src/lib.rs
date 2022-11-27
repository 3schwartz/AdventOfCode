pub mod simple;

#[test]
fn test_get_polymer_length() {
    let input = "dabAcCaCBAcCcaDA";
    let length = simple::get_polymer_length(input);
    assert_eq!(length, 10);
}