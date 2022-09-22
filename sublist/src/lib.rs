use std::cmp::Ordering;
#[derive(PartialEq,Eq,Debug)]
pub enum Comparison {
    Unequal,
    Equal,
    Sublist,
    Superlist
}
pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => if a == b { Comparison::Equal } else { Comparison::Unequal },
        Ordering::Less => if contains(b, a) { Comparison::Sublist } else { Comparison::Unequal },
        Ordering::Greater => if contains(a, b) { Comparison::Superlist } else { Comparison::Unequal }
    }
}
fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|candidate| candidate == b)
}