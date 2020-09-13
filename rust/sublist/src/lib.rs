#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if _first_list.len() == 0 {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 {
        return Comparison::Superlist;
    }

    for (i, item) in _first_list.iter().enumerate() {
        if _second_list.len() + i > _first_list.len() {
            break;
        }
        if &_first_list[i..(_second_list.len() + i)] == _second_list {
            return Comparison::Superlist;
        }
    }

    for (i, item) in _second_list.iter().enumerate() {
        if _first_list.len() + i > _second_list.len() {
            break;
        }
        if &_second_list[i..(_first_list.len() + i)] == _first_list {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}
