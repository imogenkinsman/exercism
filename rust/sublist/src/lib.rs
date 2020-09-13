#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if _first_list.len() == 0 {
        Comparison::Sublist
    } else if _second_list.len() == 0 {
        Comparison::Superlist
    } else {
        Comparison::Equal
    }
}
