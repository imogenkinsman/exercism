use std::fmt;

struct Board {
    rows: [&Row],
}

struct Row {}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

// impl Row {}

impl Board {
    fn from_strings(strings: &[&str]) -> Board {
        let rows = Row {};
        Board { rows: rows }
    }

    fn as_vector(&self) -> Vec<String> {
        let mut foo = Vec::<String>::new();
        for row in self.rows.iter() {}
        foo
    }
}

struct Cell {
    bomb: bool,
}

// impl Cell {}

pub fn annotate(strings: &[&str]) -> Vec<String> {
    Board::from_strings(strings).as_vector()
}
