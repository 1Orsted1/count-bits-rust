
use count_bits;

#[test]
fn basic() {
    assert_eq!(count_bits::perform_counting(0), 0);
    assert_eq!(count_bits::perform_counting(4), 1);
    assert_eq!(count_bits::perform_counting(7), 3);
    assert_eq!(count_bits::perform_counting(9), 2);
    assert_eq!(count_bits::perform_counting(10), 2);
    assert_eq!(count_bits::perform_counting(1234), 5);
}
