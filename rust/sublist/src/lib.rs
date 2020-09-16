#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn includes<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    for (i, _) in first.iter().enumerate() {
        if second.len() + i > first.len() {
            return false;
        }
        if &first[i..(second.len() + i)] == second {
            return true;
        }
    }

    return false;
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (_first_list, _second_list) {
        (x, y) if x == y => Comparison::Equal,
        (x, _) if x.len() == 0 => Comparison::Sublist,
        (y, _) if y.len() == 0 => Comparison::Superlist,
        (x, y) if includes(x, y) => Comparison::Superlist,
        (x, y) if includes(y, x) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
