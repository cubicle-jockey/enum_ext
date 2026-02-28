use std::fmt::Debug;

pub fn assert_iter_order3<T>(iter: impl Iterator<Item = &'static T>, expected: [&'static T; 3])
where
    T: PartialEq + Debug + 'static,
{
    for (i, v) in iter.enumerate() {
        assert_eq!(v, expected[i]);
    }
}
