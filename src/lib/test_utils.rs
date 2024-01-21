use std::fmt::Debug;

/**
 * Asserts that two slices are equal, ignoring order.
 */
pub fn assert_eq_ignore_order<T>(a: &[T], b: &[T])
    where
        T: PartialEq + Ord + Debug,
{
    let mut a: Vec<_> = a.iter().collect();
    let mut b: Vec<_> = b.iter().collect();
    a.sort();
    b.sort();

    pretty_assertions::assert_eq!(a, b);
}