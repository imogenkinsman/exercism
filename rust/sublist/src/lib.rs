#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    for (i, _) in b.iter().enumerate() {
        if a.len() + i > b.len() {
            return false;
        } else if &b[i..(a.len() + i)] == a {
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
        (x, y) if is_sublist(x, y) => Comparison::Sublist,
        (x, y) if is_sublist(y, x) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
